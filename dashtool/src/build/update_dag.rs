use std::{fs, path::Path};

use gix::diff::tree::recorder::Change;
use iceberg_rust::sql::find_relations;

use crate::{
    dag::{identifier::FullIdentifier, Dag, Ingest, IngestConfig, Node, Tabular},
    error::Error,
};

// Converts the commits into a dag without performing any operations on the tables
pub(super) fn update_dag(diff: &[Change], dag: Option<Dag>, branch: &str) -> Result<Dag, Error> {
    let mut dag = dag.unwrap_or(Dag::new());

    let mut ingests = Vec::new();
    let mut tabulars = Vec::new();

    for delta in diff {
        match delta {
            Change::Addition {
                entry_mode: _,
                oid: _,
                path,
            } => {
                if path.ends_with(b".sql") {
                    tabulars.push(String::from_utf8(path.as_slice().to_owned())?)
                } else if path.ends_with(b".ingest.json") {
                    ingests.push(String::from_utf8(path.as_slice().to_owned())?)
                };
            }
            _ => (),
        }
    }

    for path in ingests {
        let identifier = FullIdentifier::parse_path(Path::new(&path))?;

        let ingest_json: IngestConfig = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let source_json = ingest_json.source;
        let mut destination_json = ingest_json.destination;

        let image = ingest_json.image;

        destination_json["branch"] = branch.to_string().into();

        dag.add_node(Node::Ingest(Ingest::new(
            &identifier,
            &image,
            source_json.clone(),
            destination_json,
            branch,
        )))?;
    }

    for path in tabulars {
        let sql = fs::read_to_string(&path)?;

        let children = find_relations(&sql)?;

        let identifier = FullIdentifier::parse_path(Path::new(&path))?;

        dag.add_node(Node::Tabular(Tabular::new(&identifier, branch, &sql)))?;

        for child in children {
            dag.add_edge(&identifier.to_string(), &child)?
        }
    }
    Ok(dag)
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, File},
        io::Write,
        path::Path,
    };

    use gix::{diff::tree::recorder::Change, objs::tree::EntryKind, ObjectId};
    use tempfile::TempDir;

    use crate::{build::update_dag::update_dag, dag::Node};

    #[test]
    fn add_ingest() {
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

        let dag = update_dag(&changes, None, "main").expect("Failed to create dag");

        assert_eq!(dag.ingests.len(), 1);

        let orders = dag
            .ingests
            .get("bronze.inventory")
            .expect("Failed to get graph index");

        assert_eq!(orders[0], "bronze.inventory.postgres");

        let ingest = &dag.dag[*dag.map.get(&orders[0]).expect("Failed to get graph index")];

        let Node::Ingest(ingest) = ingest else {
            panic!("Node is not an ingest")
        };

        assert_eq!(ingest.image, "dashbook/source-postgres:sql")
    }

    #[test]
    fn add_tabular() {
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
                select order_id from bronze.inventory.orders;
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

        let dag = update_dag(&changes, None, "main").expect("Failed to create dag");

        assert_eq!(dag.map.len(), 2);

        let tabular = &dag.dag[*dag
            .map
            .get("silver.inventory.factOrder")
            .expect("Failed to get graph index")];

        let Node::Tabular(tabular) = tabular else {
            panic!("Node is not a tabular")
        };

        assert_eq!(
            &tabular.identifier.to_string(),
            "silver.inventory.factOrder"
        )
    }

    #[test]
    fn add_ingest_branch() {
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

        let dag = update_dag(&changes, None, "expenditures").expect("Failed to create dag");

        assert_eq!(dag.ingests.len(), 1);
        assert_eq!(dag.map.len(), 1);

        let orders = dag
            .ingests
            .get("bronze.inventory")
            .expect("Failed to get graph index");

        assert_eq!(orders[0], "bronze.inventory.postgres");

        let ingest = &dag.dag[*dag.map.get(&orders[0]).expect("Failed to get graph index")];

        let Node::Ingest(ingest) = ingest else {
            panic!("Node is not an ingest")
        };

        assert_eq!(ingest.image, "dashbook/source-postgres:sql")
    }
}
