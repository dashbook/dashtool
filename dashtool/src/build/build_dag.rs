use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::anyhow;
use datafusion_iceberg_sql::schema::get_schema;
use futures::{channel::mpsc::unbounded, stream, SinkExt, StreamExt, TryStreamExt};
use gix::diff::tree::recorder::Change;
use iceberg_rust::{
    catalog::tabular::Tabular as IcebergTabular, error::Error as IcebergError,
    materialized_view::materialized_view_builder::MaterializedViewBuilder, sql::find_relations,
};
use iceberg_rust_spec::spec::{
    schema::SchemaBuilder,
    snapshot::{SnapshotReference, SnapshotRetention},
    view_metadata::{ViewProperties, REF_PREFIX},
};
use serde_json::{Map, Value as JsonValue};

use crate::{
    dag::{identifier::FullIdentifier, Dag, Node, Singer, Tabular},
    error::Error,
    plugins::Plugin,
};

// Converts commits into a dag and performs the according transactions on the tables
pub(super) async fn build_dag<'repo>(
    dag: &mut Dag,
    diff: &[Change],
    plugin: Arc<dyn Plugin>,
    branch: &str,
    merged_branch: Option<&str>,
) -> Result<(), Error> {
    let (tabular_sender, tabular_reciever) = unbounded();
    let (singer_sender, singer_reciever) = unbounded();

    let catalog_list = plugin.catalog_list().await?;

    stream::iter(diff)
        .map(Ok::<_, Error>)
        .try_for_each(|change| {
            let plugin = plugin.clone();
            let catalog_list = catalog_list.clone();

            let mut tabular_sender = tabular_sender.clone();
            let mut singer_sender = singer_sender.clone();
            async move {
                let path = match &change {
                    Change::Addition {
                        entry_mode: _,
                        oid: _,
                        path,
                    } => path,
                    Change::Deletion {
                        entry_mode: _,
                        oid: _,
                        path,
                    } => path,
                    Change::Modification {
                        previous_entry_mode: _,
                        previous_oid: _,
                        entry_mode: _,
                        oid: _,
                        path,
                    } => path,
                }
                .to_string();
                let is_tabular = if path.ends_with(".sql") {
                    Some(true)
                } else if path.ends_with("target.json") {
                    Some(false)
                } else {
                    None
                };
                match is_tabular {
                    Some(true) => {
                        let identifier = FullIdentifier::parse_path(&Path::new(&path))?;

                        let catalog_name = identifier.catalog_name();

                        let catalog = catalog_list.catalog(catalog_name).await.ok_or(
                            IcebergError::NotFound("Catalog".to_string(), catalog_name.to_string()),
                        )?;

                        let sql = fs::read_to_string(&path)?;
                        let relations = find_relations(&sql)?;

                        match (change, merged_branch) {
                            (
                                Change::Addition {
                                    entry_mode: _,
                                    oid: _,
                                    path: _,
                                }
                                | Change::Modification {
                                    previous_entry_mode: _,
                                    previous_oid: _,
                                    entry_mode: _,
                                    oid: _,
                                    path: _,
                                },
                                Some(merged_branch),
                            ) => {
                                let tabular =
                                    catalog.load_tabular(&identifier.identifier()?).await?;
                                let mut matview =
                                    if let IcebergTabular::MaterializedView(matview) = tabular {
                                        Ok(matview)
                                    } else {
                                        Err(Error::Iceberg(IcebergError::Type(
                                            "Entity".to_string(),
                                            "not materialized view".to_string(),
                                        )))
                                    }?;
                                let version_id = matview.metadata().current_version_id;
                                let mut storage_table = matview.storage_table().await?;
                                let snapshot_id = *storage_table
                                    .metadata()
                                    .current_snapshot(Some(merged_branch))?
                                    .ok_or(Error::Iceberg(IcebergError::NotFound(
                                        "Snapshot id".to_string(),
                                        "branch".to_string() + merged_branch,
                                    )))?
                                    .snapshot_id();
                                storage_table
                                    .new_transaction(Some(merged_branch))
                                    .set_snapshot_ref((
                                        branch.to_string(),
                                        SnapshotReference {
                                            snapshot_id,
                                            retention: SnapshotRetention::default(),
                                        },
                                    ))
                                    .commit()
                                    .await?;
                                matview
                                    .new_transaction(Some(merged_branch))
                                    .update_properties(vec![(
                                        REF_PREFIX.to_string() + branch,
                                        version_id.to_string(),
                                    )])
                                    .commit()
                                    .await?;
                            }
                            (
                                Change::Addition {
                                    entry_mode: _,
                                    oid: _,
                                    path: _,
                                },
                                None,
                            ) => {
                                let relations = relations
                                    .iter()
                                    .map(|x| {
                                        FullIdentifier::parse(x).map(|y| {
                                            (
                                                y.catalog_name().clone(),
                                                y.namespace_name().clone(),
                                                y.table_name().clone(),
                                            )
                                        })
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;

                                let fields = get_schema(
                                    &sql,
                                    &relations,
                                    catalog_list.clone(),
                                    Some(branch),
                                )
                                .await?;

                                let schema = SchemaBuilder::default()
                                    .with_schema_id(1)
                                    .with_fields(fields)
                                    .build()
                                    .map_err(iceberg_rust_spec::error::Error::from)?;

                                let base_path = plugin
                                    .bucket(catalog_name)
                                    .trim_end_matches('/')
                                    .to_string()
                                    + "/"
                                    + path
                                        .as_str()
                                        .trim_start_matches('/')
                                        .trim_end_matches(".sql");
                                let mut builder = MaterializedViewBuilder::new(
                                    &sql,
                                    identifier.identifier()?,
                                    schema,
                                    catalog,
                                )?;
                                builder.location(&base_path);
                                builder.properties(ViewProperties {
                                    storage_table: identifier.to_string() + "__storage",
                                    other: HashMap::from_iter(vec![(
                                        REF_PREFIX.to_string() + branch,
                                        1.to_string(),
                                    )]),
                                });
                                builder.build().await?;
                            }
                            _ => (),
                        }

                        tabular_sender
                            .send((identifier.to_string(), (sql, relations)))
                            .await?;
                    }
                    Some(false) => {
                        let tap_path =
                            path.trim_end_matches("target.json").to_string() + "tap.json";
                        let tap_json: JsonValue =
                            serde_json::from_str(&fs::read_to_string(&tap_path)?)?;
                        let mut target_json: JsonValue =
                            serde_json::from_str(&fs::read_to_string(path)?)?;
                        target_json["branch"] = branch.to_string().into();

                        let streams = if let JsonValue::Object(object) = &target_json["streams"] {
                            Ok(object)
                        } else {
                            Err(Error::Text(
                                "Streams in config must be an object.".to_string(),
                            ))
                        }?;

                        for (stream, stream_config) in streams.iter() {
                            let name = if let JsonValue::String(name) = &stream_config["identifier"]
                            {
                                Ok(name)
                            } else {
                                Err(Error::Text("Stream identifer is not a string.".to_string()))
                            }?;

                            if let Some(merged_branch) = merged_branch {
                                let identifier = FullIdentifier::parse(name)?;
                                let catalog_name = identifier.catalog_name();

                                let catalog = catalog_list.catalog(catalog_name).await.ok_or(
                                    IcebergError::NotFound(
                                        "Catalog".to_string(),
                                        catalog_name.to_string(),
                                    ),
                                )?;

                                let tabular =
                                    catalog.load_tabular(&identifier.identifier()?).await?;

                                let mut table = if let IcebergTabular::Table(table) = tabular {
                                    Ok(table)
                                } else {
                                    Err(Error::Iceberg(IcebergError::Type(
                                        "Entity".to_string(),
                                        "not table".to_string(),
                                    )))
                                }?;
                                let snapshot_id = *table
                                    .metadata()
                                    .current_snapshot(Some(merged_branch))?
                                    .ok_or(Error::Iceberg(IcebergError::NotFound(
                                        "Snapshot id".to_string(),
                                        "branch".to_string() + merged_branch,
                                    )))?
                                    .snapshot_id();
                                table
                                    .new_transaction(Some(merged_branch))
                                    .set_snapshot_ref((
                                        branch.to_string(),
                                        SnapshotReference {
                                            snapshot_id,
                                            retention: SnapshotRetention::default(),
                                        },
                                    ))
                                    .commit()
                                    .await?;
                            }
                            let mut target_json = target_json.clone();
                            target_json["streams"] =
                                Map::from_iter(vec![(stream.clone(), stream_config.clone())])
                                    .into();
                            singer_sender
                                .send(Node::Singer(Singer::new(
                                    name,
                                    tap_json.clone(),
                                    target_json,
                                    branch,
                                )))
                                .await?;
                        }
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

    let tabs: HashMap<String, (String, Vec<String>)> =
        HashMap::from_iter(tabular_reciever.collect::<Vec<_>>().await);

    for singer in singers {
        dag.add_node(singer)
    }

    for (node, (sql, _)) in &tabs {
        dag.add_node(Node::Tabular(Tabular::new(node, branch, sql)))
    }

    for (node, (_, children)) in tabs {
        for child in children {
            dag.add_edge(&node, &child)?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, File},
        io::Write,
        path::Path,
        sync::Arc,
    };

    use gix::{diff::tree::recorder::Change, objs::tree::EntryKind, ObjectId};
    use iceberg_rust::{
        catalog::{identifier::Identifier, tabular::Tabular, CatalogList},
        table::table_builder::TableBuilder,
    };
    use iceberg_rust_spec::spec::{
        partition::{PartitionField, PartitionSpecBuilder, Transform},
        schema::SchemaBuilder,
        types::{PrimitiveType, StructField, StructTypeBuilder, Type},
    };
    use iceberg_sql_catalog::SqlCatalogList;
    use object_store::memory::InMemory;
    use tempfile::TempDir;

    use crate::{
        build::{build_dag::build_dag, update_dag::update_dag},
        dag::Node,
        plugins::{sql::SqlPlugin, Config, Plugin},
    };

    #[tokio::test]
    async fn add_singer() {
        let temp_dir = TempDir::new().unwrap();

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
                    "image": "dashbook/pipelinewise-tap-postgres:iceberg",
                    "streams": {"inventory-orders": { "identifier": "bronze.inventory.orders" }},
                    "catalog": "https://api.dashbook.dev/nessie/cat-1w0qookj",
                    "bucket": "s3://example-postgres/",
                    "access_token": "$ACCESS_TOKEN",
                    "id_token": "$ID_TOKEN"
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let changes = vec![
            Change::Addition {
                entry_mode: EntryKind::Tree
                    .try_into()
                    .expect("Failed to create git entry"),
                oid: ObjectId::null(gix::hash::Kind::Sha1),
                path: tap_path.to_str().unwrap().into(),
            },
            Change::Addition {
                entry_mode: EntryKind::Tree
                    .try_into()
                    .expect("Failed to create git entry"),
                oid: ObjectId::null(gix::hash::Kind::Sha1),
                path: target_path.to_str().unwrap().into(),
            },
        ];

        let mut dag = update_dag(&vec![], None, "main").expect("Failed to create dag");

        let config_json = r#"
                {
                   "plugin": "sql",
	               "catalogUrl": "sqlite://",
	               "secrets": {}, 
                   "bucket": ""
                }
                "#;

        let config = match serde_json::from_str(&config_json).expect("Failed to parse sql config") {
            Config::Sql(config) => config,
        };

        let plugin = Arc::new(
            SqlPlugin::new(config)
                .await
                .expect("Failed to create plugin"),
        );

        build_dag(&mut dag, &changes, plugin, "main", None)
            .await
            .expect("Failed to build dag");

        assert_eq!(dag.map.len(), 1);

        let singer = &dag.dag[*dag
            .map
            .get("bronze.inventory.orders")
            .expect("Failed to get graph index")];

        let Node::Singer(singer) = singer else {
            panic!("Node is not a singer")
        };

        assert_eq!(
            singer.target["image"],
            "dashbook/pipelinewise-tap-postgres:iceberg"
        );
        assert_eq!(singer.target["branch"], "main");
    }

    #[tokio::test]
    async fn add_tabular() {
        let temp_dir = TempDir::new().unwrap();

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
                    "image": "dashbook/pipelinewise-tap-postgres:iceberg",
                    "streams": {"inventory-orders": { "identifier": "bronze.inventory.orders" }},
                    "catalog": "https://api.dashbook.dev/nessie/cat-1w0qookj",
                    "bucket": "s3://example-postgres/",
                    "access_token": "$ACCESS_TOKEN",
                    "id_token": "$ID_TOKEN"
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

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

        let changes = vec![
            Change::Addition {
                entry_mode: EntryKind::Tree
                    .try_into()
                    .expect("Failed to create git entry"),
                oid: ObjectId::null(gix::hash::Kind::Sha1),
                path: tap_path.to_str().unwrap().into(),
            },
            Change::Addition {
                entry_mode: EntryKind::Tree
                    .try_into()
                    .expect("Failed to create git entry"),
                oid: ObjectId::null(gix::hash::Kind::Sha1),
                path: target_path.to_str().unwrap().into(),
            },
            Change::Addition {
                entry_mode: EntryKind::Tree
                    .try_into()
                    .expect("Failed to create git entry"),
                oid: ObjectId::null(gix::hash::Kind::Sha1),
                path: tabular_path.to_str().unwrap().into(),
            },
        ];

        let mut dag = update_dag(&vec![], None, "main").expect("Failed to create dag");

        let object_store = Arc::new(InMemory::new());

        let catalog_list = Arc::new(
            SqlCatalogList::new("sqlite://", object_store.clone())
                .await
                .expect("Failed to create catalog list"),
        );

        let bronze_catalog = catalog_list
            .catalog("bronze")
            .await
            .expect("Failed to create catalog");

        let schema = SchemaBuilder::default()
            .with_schema_id(1)
            .with_fields(
                StructTypeBuilder::default()
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
            )
            .build()
            .unwrap();

        let partition_spec = PartitionSpecBuilder::default()
            .with_spec_id(1)
            .with_partition_field(PartitionField::new(4, 1000, "day", Transform::Day))
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

        let config = match serde_json::from_str(
            r#"
                {
                    "plugin": "sql",
                    "catalogUrl": "sqlite://",
                    "bucket": "test",
                    "secrets": {} 
                }
                "#,
        )
        .expect("Failed to parse sql config")
        {
            Config::Sql(config) => config,
        };

        let plugin = Arc::new(
            SqlPlugin::new_with_catalog(config, catalog_list).expect("Failed to create plugin"),
        );

        build_dag(&mut dag, &changes, plugin.clone(), "main", None)
            .await
            .expect("Failed to build dag");

        assert_eq!(dag.map.len(), 2);

        let tabular = &dag.dag[*dag
            .map
            .get("silver.inventory.factOrder")
            .expect("Failed to get graph index")];

        let Node::Tabular(tabular) = tabular else {
            panic!("Node is not a singer")
        };

        assert_eq!(tabular.identifier, "silver.inventory.factOrder");

        let catalog_list = plugin
            .catalog_list()
            .await
            .expect("Failed to get catalog list.");

        let catalog = catalog_list
            .catalog("silver")
            .await
            .expect("Failed to get catalog");

        let matview = if let Tabular::MaterializedView(matview) = catalog
            .load_tabular(
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

        assert_eq!(schema.fields()[0].name, "product_id");
        assert_eq!(schema.fields()[0].field_type.to_string(), "long");

        assert_eq!(
            schema.fields()[1].name,
            "SUM(bronze.inventory.orders.amount)"
        );
        assert_eq!(schema.fields()[1].field_type.to_string(), "long");
    }
}
