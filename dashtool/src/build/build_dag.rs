use std::{collections::HashMap, fs, path::Path, sync::Arc};

use datafusion_iceberg_sql::schema::get_schema;
use futures::{channel::mpsc::unbounded, stream, SinkExt, StreamExt, TryStreamExt};
use gix::diff::tree::recorder::Change;
use iceberg_rust::catalog::namespace::Namespace;
use iceberg_rust::spec::view_metadata::{Version, ViewRepresentation};
use iceberg_rust::{
    catalog::tabular::Tabular as IcebergTabular, error::Error as IcebergError, sql::find_relations,
};
use iceberg_rust::{
    materialized_view::MaterializedView,
    spec::{
        schema::SchemaBuilder,
        snapshot::{SnapshotReference, SnapshotRetention},
        view_metadata::REF_PREFIX,
    },
};
use itertools::Itertools;

use crate::{
    dag::{identifier::FullIdentifier, Dag, Ingest, IngestConfig, Node, Tabular},
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
    let (ingest_sender, ingest_reciever) = unbounded();

    let catalog_list = plugin.catalog_list().await?;

    stream::iter(diff)
        .map(Ok::<_, Error>)
        .try_for_each(|change| {
            let plugin = plugin.clone();
            let catalog_list = catalog_list.clone();

            let mut tabular_sender = tabular_sender.clone();
            let mut ingest_sender = ingest_sender.clone();
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
                } else if path.ends_with(".ingest.json") {
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
                                    .with_fields(fields)
                                    .build()
                                    .map_err(iceberg_rust::spec::error::Error::from)?;

                                let base_path = plugin
                                    .bucket(catalog_name)
                                    .trim_end_matches('/')
                                    .to_string()
                                    + "/"
                                    + path
                                        .as_str()
                                        .trim_start_matches('/')
                                        .trim_end_matches(".sql");

                                MaterializedView::builder()
                                    .with_name(identifier.table_name())
                                    .with_location(&base_path)
                                    .with_schema(schema)
                                    .with_view_version(
                                        Version::builder()
                                            .with_representation(ViewRepresentation::sql(
                                                &sql, None,
                                            ))
                                            .build()
                                            .map_err(IcebergError::from)?,
                                    )
                                    .with_property((REF_PREFIX.to_string() + branch, 0.to_string()))
                                    .build(identifier.identifier()?.namespace(), catalog)
                                    .await?;
                            }
                            _ => (),
                        }

                        tabular_sender
                            .send((identifier.to_string(), (sql, relations)))
                            .await?;
                    }
                    Some(false) => {
                        let ingest_json: IngestConfig =
                            serde_json::from_str(&fs::read_to_string(&path)?)?;
                        let source_json = ingest_json.source;
                        let mut destination_json = ingest_json.destination;

                        let image = ingest_json.image;

                        destination_json["branch"] = branch.to_string().into();

                        let (catalog_name, namespace_name) =
                            path.split("/").next_tuple().ok_or(Error::Text(
                                "File path doesn't contain catalog name and namespace".to_owned(),
                            ))?;

                        let catalog =
                            catalog_list.catalog(catalog_name).await.ok_or(Error::Text(
                                format!("Catalog {} not found in catalog list", catalog_name),
                            ))?;

                        let namespace = Namespace::try_new(&[namespace_name.to_string()])?;

                        let identifiers = catalog.list_tabulars(&namespace).await?;

                        for identifier in identifiers.iter() {
                            if let Some(merged_branch) = merged_branch {
                                let tabular = catalog.clone().load_tabular(&identifier).await?;

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
                        }
                        let ingest_key = FullIdentifier::parse_path(Path::new(&path))?;
                        ingest_sender
                            .send(Node::Ingest(Ingest::new(
                                &ingest_key.to_string(),
                                &image,
                                source_json.clone(),
                                destination_json,
                                branch,
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
    ingest_sender.close_channel();

    let ingests = ingest_reciever.collect::<Vec<_>>().await;

    let tabs: HashMap<String, (String, Vec<String>)> =
        HashMap::from_iter(tabular_reciever.collect::<Vec<_>>().await);

    for ingest in ingests {
        dag.add_node(ingest)?;
    }

    for (node, (sql, _)) in &tabs {
        dag.add_node(Node::Tabular(Tabular::new(node, branch, sql)))?;
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
    use iceberg_rust::spec::{
        partition::{PartitionField, PartitionSpecBuilder, Transform},
        schema::SchemaBuilder,
        types::{PrimitiveType, StructField, StructTypeBuilder, Type},
    };
    use iceberg_rust::{
        catalog::{identifier::Identifier, tabular::Tabular, CatalogList},
        table::Table,
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
    async fn add_ingest() {
        let temp_dir = TempDir::new().unwrap();

        env::set_current_dir(temp_dir.path()).expect("Failed to set current work dir");
        std::env::current_dir().expect("Failed to sync workdir");

        let bronze_path = temp_dir.path().join("bronze");
        fs::create_dir(&bronze_path).expect("Failed to create directory");

        let bronze_inventory_path = bronze_path.join("inventory");
        fs::create_dir(&bronze_inventory_path).expect("Failed to create directory");

        let config_path = bronze_inventory_path.join(Path::new("postgres.ingest.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                "image":"dashbook/source-postgres:sql",
                "source":{
                       "host": "172.17.0.2",
    	               "port": 5432,
    	               "user": "postgres",
    	               "password": "$POSTGRES_PASSWORD",
    	               "dbname": "postgres",
    	               "filter_schemas": "inventory",
    	               "default_replication_method": "LOG_BASED"
                    },
                "destination":{
                        "streams": {"inventory-orders": { "identifier": "bronze.inventory.orders" }},
                        "catalog": "https://api.dashbook.dev/nessie/cat-1w0qookj",
                        "bucket": "s3://example-postgres/",
                        "access_token": "$ACCESS_TOKEN",
                        "id_token": "$ID_TOKEN"
                    }
                }
                "#
                .as_bytes(),
            )
            .expect("Failed to write to file");

        let changes = vec![Change::Addition {
            entry_mode: EntryKind::Tree
                .try_into()
                .expect("Failed to create git entry"),
            oid: ObjectId::null(gix::hash::Kind::Sha1),
            path: config_path.to_str().unwrap().into(),
        }];

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

        assert_eq!(dag.ingests.len(), 1);
        assert_eq!(dag.map.len(), 1);

        let orders = dag
            .ingests
            .get("bronze.inventory.orders")
            .expect("Failed to get graph index");

        assert_eq!(orders, "bronze.inventory.postgres");

        let ingest = &dag.dag[*dag.map.get(orders).expect("Failed to get graph index")];

        let Node::Ingest(ingest) = ingest else {
            panic!("Node is not an ingest")
        };

        assert_eq!(ingest.image, "dashbook/source-postgres:sql");
        assert_eq!(ingest.destination["branch"], "main");
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

        let config_path = bronze_inventory_path.join(Path::new("postgres.ingest.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                "image":"dashbook/source-postgres:sql",
                "source":{
                       "host": "172.17.0.2",
    	               "port": 5432,
    	               "user": "postgres",
    	               "password": "$POSTGRES_PASSWORD",
    	               "dbname": "postgres",
    	               "filter_schemas": "inventory",
    	               "default_replication_method": "LOG_BASED"
                    },
                "destination":{
                        "streams": {"inventory-orders": { "identifier": "bronze.inventory.orders" }},
                        "catalog": "https://api.dashbook.dev/nessie/cat-1w0qookj",
                        "bucket": "s3://example-postgres/",
                        "access_token": "$ACCESS_TOKEN",
                        "id_token": "$ID_TOKEN"
                    }
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
                path: config_path.to_str().unwrap().into(),
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

        Table::builder()
            .with_name("orders")
            .with_location("/bronze/inventory/orders")
            .with_schema(schema)
            .with_partition_spec(partition_spec)
            .build(&["inventory".to_owned()], bronze_catalog.clone())
            .await
            .expect("Failed to creat table.");

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

        assert_eq!(dag.ingests.len(), 1);
        assert_eq!(dag.map.len(), 2);

        let tabular = &dag.dag[*dag
            .map
            .get("silver.inventory.factOrder")
            .expect("Failed to get graph index")];

        let Node::Tabular(tabular) = tabular else {
            panic!("Node is not a tabular")
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
