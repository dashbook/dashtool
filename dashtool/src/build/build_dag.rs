use std::{collections::HashMap, ffi::OsStr, fs, path::Path, sync::Arc};

use anyhow::anyhow;
use datafusion_iceberg_sql::schema::get_schema;
use futures::{channel::mpsc::unbounded, stream, SinkExt, StreamExt, TryStreamExt};
use git2::{Delta, Diff};
use iceberg_rust::{
    catalog::tabular::Tabular as IcebergTabular, error::Error as IcebergError,
    materialized_view::materialized_view_builder::MaterializedViewBuilder, sql::find_relations,
};
use iceberg_rust_spec::spec::{
    schema::Schema,
    snapshot::{Reference, Retention},
    view_metadata::REF_PREFIX,
};
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::{
    dag::{identifier::FullIdentifier, Dag, Node, Singer, Tabular},
    error::Error,
    plugins::Plugin,
};

// Converts commits into a dag and performs the according transactions on the tables
pub(super) async fn build_dag<'repo>(
    dag: &mut Dag,
    diff: Diff<'repo>,
    plugin: Arc<dyn Plugin>,
    branch: &str,
    merged_branch: Option<&str>,
) -> Result<(), Error> {
    let (tabular_sender, tabular_reciever) = unbounded();
    let (singer_sender, singer_reciever) = unbounded();

    stream::iter(diff.deltas())
        .map(Ok::<_, Error>)
        .try_for_each(|delta| {
            let plugin = plugin.clone();
            let mut tabular_sender = tabular_sender.clone();
            let mut singer_sender = singer_sender.clone();
            async move {
                let path = delta
                    .new_file()
                    .path()
                    .ok_or(Error::Text("No new file in delta".to_string()))?;
                let is_tabular = if path.extension() == Some(&OsStr::new("sql")) {
                    Some(true)
                } else if path.ends_with("target.json") {
                    Some(false)
                } else {
                    None
                };
                match is_tabular {
                    Some(true) => {
                        let identifier = FullIdentifier::parse_path(&path)?;

                        let catalog = plugin
                            .catalog(
                                &identifier.catalog_name(),
                                &identifier.namespace_name(),
                                &identifier.table_name(),
                            )
                            .await?;

                        let sql = fs::read_to_string(path)?;
                        let relations = find_relations(&sql)?;

                        match (delta.status(), merged_branch) {
                            (Delta::Added | Delta::Modified, Some(merged_branch)) => {
                                let tabular = catalog.load_table(&identifier.identifier()?).await?;
                                let mut matview =
                                    if let IcebergTabular::MaterializedView(matview) = tabular {
                                        Ok(matview)
                                    } else {
                                        Err(Error::Iceberg(IcebergError::Type(
                                            "Entity".to_string(),
                                            "not materialized view".to_string(),
                                        )))
                                    }?;
                                let mut storage_table =
                                    matview.storage_table(Some(merged_branch)).await?;
                                let snapshot_id = storage_table
                                    .metadata()
                                    .current_snapshot(Some(merged_branch))?
                                    .ok_or(Error::Iceberg(IcebergError::NotFound(
                                        "Snapshot id".to_string(),
                                        "branch".to_string() + merged_branch,
                                    )))?
                                    .snapshot_id;
                                matview
                                    .new_transaction(Some(merged_branch))
                                    .update_properties(vec![(
                                        REF_PREFIX.to_string() + branch,
                                        snapshot_id.to_string(),
                                    )])
                                    .commit()
                                    .await?;
                                storage_table
                                    .new_transaction(Some(merged_branch))
                                    .set_ref((
                                        branch.to_string(),
                                        Reference {
                                            snapshot_id,
                                            retention: Retention::default(),
                                        },
                                    ))
                                    .commit()
                                    .await?;
                            }
                            (Delta::Added, None) => {
                                let relations = relations
                                    .iter()
                                    .map(|x| {
                                        FullIdentifier::parse(x).and_then(|y| {
                                            Ok((
                                                y.catalog_name().clone(),
                                                y.namespace_name().clone(),
                                                y.table_name().clone(),
                                            ))
                                        })
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;

                                let catalogs = stream::iter(relations.iter())
                                    .then(|(catalog_name, namespace_name, table_name)| {
                                        let plugin = plugin.clone();
                                        async move {
                                            Ok::<_, Error>((
                                                catalog_name.clone(),
                                                plugin
                                                    .catalog(
                                                        catalog_name,
                                                        namespace_name,
                                                        table_name,
                                                    )
                                                    .await?,
                                            ))
                                        }
                                    })
                                    .try_collect::<HashMap<_, _>>()
                                    .await?;

                                let fields =
                                    get_schema(&sql, &relations, &catalogs, Some(&branch)).await?;

                                let schema = Schema {
                                    schema_id: 1,
                                    identifier_field_ids: None,
                                    fields,
                                };
                                let base_path = plugin.bucket().trim_end_matches("/").to_string()
                                    + path
                                        .to_str()
                                        .ok_or(Error::Text("No new file in delta".to_string()))?
                                        .trim_start_matches("/")
                                        .trim_end_matches(".sql");
                                let mut builder = MaterializedViewBuilder::new(
                                    &sql,
                                    &identifier.identifier()?,
                                    schema,
                                    catalog,
                                )?;
                                builder.location(base_path);
                                builder.build().await?;
                            }
                            _ => (),
                        }

                        tabular_sender
                            .send((identifier.to_string(), relations))
                            .await?;
                    }
                    Some(false) => {
                        let parent_path = Path::new(path).parent().ok_or(Error::Anyhow(
                            anyhow!("target.json must be inside a subfolder."),
                        ))?;
                        let tap_path = parent_path.join("tap.json");
                        let tap_json = fs::read_to_string(&tap_path)?;
                        let target_json = fs::read_to_string(path)?;
                        let target: SingerConfig = serde_json::from_str(&target_json)?;
                        let identifier = parent_path
                            .to_str()
                            .ok_or(Error::Anyhow(anyhow!("Failed to convert path to string.")))?;
                        singer_sender
                            .send(Node::Singer(Singer::new(
                                identifier, tap_json, target, branch,
                            )))
                            .await?;
                    }
                    _ => (),
                };
                Ok(())
            }
        })
        .await?;

    tabular_sender.close_channel();
    singer_sender.close_channel();

    let singers = singer_reciever.collect::<Vec<_>>().await;

    let tabs: HashMap<String, Vec<String>> =
        HashMap::from_iter(tabular_reciever.collect::<Vec<_>>().await);

    for singer in singers {
        dag.add_node(singer)
    }

    for node in tabs.keys() {
        dag.add_node(Node::Tabular(Tabular::new(node, branch)))
    }

    for (node, children) in tabs {
        for child in children {
            dag.add_edge(&node, &child)?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        env,
        fs::{self, File},
        io::Write,
        path::Path,
        sync::Arc,
    };

    use git2::DiffOptions;
    use iceberg_catalog_sql::SqlCatalog;
    use iceberg_rust::{
        catalog::{identifier::Identifier, tabular::Tabular, Catalog},
        table::table_builder::TableBuilder,
    };
    use iceberg_rust_spec::spec::{
        partition::{PartitionField, PartitionSpecBuilder, Transform},
        schema::Schema,
        types::{PrimitiveType, StructField, StructTypeBuilder, Type},
    };
    use object_store::memory::InMemory;

    use crate::{
        build::{build_dag::build_dag, update_dag::update_dag},
        dag::Node,
        plugins::{sql::SqlPlugin, Plugin},
        test::repo_init,
    };

    #[tokio::test]
    async fn add_singer() {
        let (temp_dir, repo) = repo_init();

        env::set_current_dir(temp_dir.path()).expect("Failed to set current work dir");
        std::env::current_dir().expect("Failed to sync workdir");

        let bronze_path = temp_dir.path().join("bronze");
        fs::create_dir(&bronze_path).expect("Failed to create directory");

        let bronze_inventory_path = bronze_path.join("inventory");
        fs::create_dir(&bronze_inventory_path).expect("Failed to create directory");

        let tap_path = bronze_inventory_path.join(Path::new("tap.json"));
        File::create(&tap_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                   "host": "172.17.0.2",
	               "port": 5432,
	               "user": "postgres",
	               "password": "$POSTGRES_PASSWORD",
	               "dbname": "postgres",
	               "filter_schemas": "inventory",
	               "default_replication_method": "LOG_BASED"
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let target_path = bronze_inventory_path.join(Path::new("target.json"));
        File::create(&target_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                    "image": "ghcr.io/dashbook/pipelinewise-tap-postgres:iceberg",
                    "streams": {"inventory_orders": "bronze.inventory.orders"},
                    "catalog": "https://api.dashbook.dev/nessie/cat-1w0qookj",
                    "bucket": "s3://example-postgres/",
                    "access_token": "$ACCESS_TOKEN",
                    "id_token": "$ID_TOKEN"
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let mut opts = DiffOptions::new();
        opts.include_untracked(true);

        let mut index = repo.index().expect("Failed to create index");
        index
            .add_path(
                &tap_path
                    .as_path()
                    .strip_prefix(temp_dir.path())
                    .expect("Failed to get relative path of file"),
            )
            .expect("Failed to add path to index");
        index
            .add_path(
                &target_path
                    .as_path()
                    .strip_prefix(temp_dir.path())
                    .expect("Failed to get relative path of file"),
            )
            .expect("Failed to add path to index");

        let last_main_diff = repo
            .diff_tree_to_tree(None, None, None)
            .expect("Failed to create diff for repo");

        let main_diff = repo
            .diff_tree_to_index(None, Some(&index), Some(&mut opts))
            .expect("Failed to create diff for repo");

        let mut dag = update_dag(last_main_diff, None, "main").expect("Failed to create dag");

        let config_path = temp_dir.path().join(Path::new("dashtool.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                   "name": "iceberg",
	               "url": "sqlite://",
	               "region": "null",
	               "secretName": "null",
	               "includes": []
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let plugin = Arc::new(
            SqlPlugin::new(config_path.to_str().expect("Failed to convert path to str"))
                .await
                .expect("Failed to create plugin"),
        );

        build_dag(&mut dag, main_diff, plugin, "main", None)
            .await
            .expect("Failed to build dag");

        assert_eq!(dag.singers.len(), 1);
        assert_eq!(dag.map.len(), 1);

        let orders = dag
            .singers
            .get("bronze.inventory.orders")
            .expect("Failed to get singer");

        assert_eq!(orders, "bronze/inventory");

        let singer = &dag.dag[dag
            .map
            .get(orders)
            .expect("Failed to get graph index")
            .clone()];

        let Node::Singer(singer) = singer else {
            panic!("Node is not a singer")
        };

        assert_eq!(
            singer.target.image,
            "ghcr.io/dashbook/pipelinewise-tap-postgres:iceberg"
        );
    }

    #[tokio::test]
    async fn add_tabular() {
        let (temp_dir, repo) = repo_init();

        env::set_current_dir(temp_dir.path()).expect("Failed to set current work dir");
        std::env::current_dir().expect("Failed to sync workdir");

        let bronze_path = temp_dir.path().join("bronze");
        fs::create_dir(&bronze_path).expect("Failed to create directory");

        let bronze_inventory_path = bronze_path.join("inventory");
        fs::create_dir(&bronze_inventory_path).expect("Failed to create directory");

        let tap_path = bronze_inventory_path.join(Path::new("tap.json"));
        File::create(&tap_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                   "host": "172.17.0.2",
	               "port": 5432,
	               "user": "postgres",
	               "password": "$POSTGRES_PASSWORD",
	               "dbname": "postgres",
	               "filter_schemas": "inventory",
	               "default_replication_method": "LOG_BASED"
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let target_path = bronze_inventory_path.join(Path::new("target.json"));
        File::create(&target_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                    "image": "ghcr.io/dashbook/pipelinewise-tap-postgres:iceberg",
                    "streams": {"inventory_orders": "bronze.inventory.orders"},
                    "catalog": "https://api.dashbook.dev/nessie/cat-1w0qookj",
                    "bucket": "s3://example-postgres/",
                    "access_token": "$ACCESS_TOKEN",
                    "id_token": "$ID_TOKEN"
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let mut opts = DiffOptions::new();
        opts.include_untracked(true);

        let mut index = repo.index().expect("Failed to create index");
        index
            .add_path(
                &tap_path
                    .as_path()
                    .strip_prefix(temp_dir.path())
                    .expect("Failed to get relative path of file"),
            )
            .expect("Failed to add path to index");
        index
            .add_path(
                &target_path
                    .as_path()
                    .strip_prefix(temp_dir.path())
                    .expect("Failed to get relative path of file"),
            )
            .expect("Failed to add path to index");

        let silver_path = temp_dir.path().join("silver");
        fs::create_dir(&silver_path).expect("Failed to create directory");

        let silver_inventory_path = silver_path.join("inventory");
        fs::create_dir(&silver_inventory_path).expect("Failed to create directory");

        let tabular_path = silver_inventory_path.join(Path::new("factOrder.sql"));
        File::create(&tabular_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                select product_id, sum(amount) from bronze.inventory.orders group by product_id;
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        index
            .add_path(
                &tabular_path
                    .as_path()
                    .strip_prefix(temp_dir.path())
                    .expect("Failed to get relative path of file"),
            )
            .expect("Failed to add path to index");

        let last_main_diff = repo
            .diff_tree_to_tree(None, None, None)
            .expect("Failed to create diff for repo");

        let main_diff = repo
            .diff_tree_to_index(None, Some(&index), Some(&mut opts))
            .expect("Failed to create diff for repo");

        let mut dag = update_dag(last_main_diff, None, "main").expect("Failed to create dag");

        let object_store = Arc::new(InMemory::new());

        let bronze_catalog = Arc::new(
            SqlCatalog::new("sqlite://", "bronze", object_store.clone())
                .await
                .expect("Failed to create catalog"),
        ) as Arc<dyn Catalog>;

        let silver_catalog = Arc::new(
            SqlCatalog::new("sqlite://", "silver", object_store)
                .await
                .expect("Failed to create catalog"),
        );

        let schema = Schema {
            schema_id: 1,
            identifier_field_ids: None,
            fields: StructTypeBuilder::default()
                .with_struct_field(StructField {
                    id: 1,
                    name: "id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 2,
                    name: "customer_id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 3,
                    name: "product_id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 4,
                    name: "date".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Date),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 5,
                    name: "amount".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Int),
                    doc: None,
                })
                .build()
                .unwrap(),
        };
        let partition_spec = PartitionSpecBuilder::default()
            .spec_id(1)
            .with_partition_field(PartitionField {
                source_id: 4,
                field_id: 1000,
                name: "day".to_string(),
                transform: Transform::Day,
            })
            .build()
            .expect("Failed to create partition spec");

        let mut builder = TableBuilder::new("inventory.orders", bronze_catalog.clone())
            .expect("Failed to create table builder");
        builder
            .location("/bronze/inventory/orders")
            .with_schema((1, schema))
            .current_schema_id(1)
            .with_partition_spec((1, partition_spec))
            .default_spec_id(1);

        builder.build().await.expect("Failed to create table.");

        let plugin = Arc::new(
            SqlPlugin::new_with_catalogs(
                silver_catalog,
                HashMap::from_iter(vec![("bronze".to_string(), bronze_catalog)]),
            )
            .expect("Failed to create plugin"),
        );

        build_dag(&mut dag, main_diff, plugin.clone(), "main", None)
            .await
            .expect("Failed to build dag");

        assert_eq!(dag.singers.len(), 1);
        assert_eq!(dag.map.len(), 2);

        let tabular = &dag.dag[dag
            .map
            .get("silver.inventory.factOrder")
            .expect("Failed to get graph index")
            .clone()];

        let Node::Tabular(tabular) = tabular else {
            panic!("Node is not a singer")
        };

        assert_eq!(tabular.identifier, "silver.inventory.factOrder");

        let catalog = plugin
            .catalog("silver", "inventory", "factOrder")
            .await
            .expect("Failed to get catalog");

        let matview = if let Tabular::MaterializedView(matview) = catalog
            .load_table(
                &Identifier::parse("inventory.factOrder").expect("Failed to parse identifier"),
            )
            .await
            .expect("Failed to load table")
        {
            matview
        } else {
            panic!("Result is not a materialized view")
        };

        let schema = matview
            .metadata()
            .current_schema(None)
            .expect("Failed to get schema");

        assert_eq!(schema.fields.fields[0].name, "product_id");
        assert_eq!(schema.fields.fields[0].field_type.to_string(), "long");

        assert_eq!(
            schema.fields.fields[1].name,
            "SUM(bronze.inventory.orders.amount)"
        );
        assert_eq!(schema.fields.fields[1].field_type.to_string(), "long");
    }
}
