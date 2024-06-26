use std::{fs, path::Path};

use gix::diff::tree::recorder::Change;
use iceberg_rust::sql::find_relations;

use crate::{
    dag::{identifier::FullIdentifier, Dag, Node, Singer, SingerConfig, Tabular},
    error::Error,
};

// Converts the commits into a dag without performing any operations on the tables
pub(super) fn update_dag(diff: &[Change], dag: Option<Dag>, branch: &str) -> Result<Dag, Error> {
    let mut dag = dag.unwrap_or(Dag::new());

    let mut singers = Vec::new();
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
                } else if path.ends_with(b".singer.json") {
                    singers.push(String::from_utf8(path.as_slice().to_owned())?)
                };
            }
            _ => (),
        }
    }

    for path in singers {
        let identifier = FullIdentifier::parse_path(Path::new(&path))?.to_string();

        let singer_json: SingerConfig = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let tap_json = singer_json.tap;
        let mut target_json = singer_json.target;

        let image = singer_json.image;

        target_json["branch"] = branch.to_string().into();

        dag.add_node(Node::Singer(Singer::new(
            &identifier,
            &image,
            tap_json.clone(),
            target_json,
            branch,
        )))?;
    }

    for path in tabulars {
        let sql = fs::read_to_string(&path)?;

        let children = find_relations(&sql)?;

        let identifier = FullIdentifier::parse_path(Path::new(&path))?.to_string();

        dag.add_node(Node::Tabular(Tabular::new(&identifier, branch, &sql)))?;

        for child in children {
            dag.add_edge(&identifier, &child)?
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
    fn add_singer() {
        let temp_dir = TempDir::new().unwrap();

        env::set_current_dir(temp_dir.path()).expect("Failed to set current work dir");
        std::env::current_dir().expect("Failed to sync workdir");

        let bronze_path = temp_dir.path().join("bronze");
        fs::create_dir(&bronze_path).expect("Failed to create directory");

        let bronze_inventory_path = bronze_path.join("inventory");
        fs::create_dir(&bronze_inventory_path).expect("Failed to create directory");

        let config_path = bronze_inventory_path.join(Path::new("postgres.singer.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                "image":"dashbook/pipelinewise-tap-postgres:sql",
                "tap":{
                       "host": "172.17.0.2",
    	               "port": 5432,
    	               "user": "postgres",
    	               "password": "$POSTGRES_PASSWORD",
    	               "dbname": "postgres",
    	               "filter_schemas": "inventory",
    	               "default_replication_method": "LOG_BASED"
                    },
                "target":{
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

        let dag = update_dag(&changes, None, "main").expect("Failed to create dag");

        assert_eq!(dag.singers.len(), 1);

        let orders = dag
            .singers
            .get("bronze.inventory.orders")
            .expect("Failed to get graph index");

        assert_eq!(orders, "bronze.inventory.postgres");

        let singer = &dag.dag[*dag.map.get(orders).expect("Failed to get graph index")];

        let Node::Singer(singer) = singer else {
            panic!("Node is not a singer")
        };

        assert_eq!(singer.image, "dashbook/pipelinewise-tap-postgres:sql")
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

        let config_path = bronze_inventory_path.join(Path::new("postgres.singer.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                "image":"dashbook/pipelinewise-tap-postgres:sql",
                "tap":{
                       "host": "172.17.0.2",
    	               "port": 5432,
    	               "user": "postgres",
    	               "password": "$POSTGRES_PASSWORD",
    	               "dbname": "postgres",
    	               "filter_schemas": "inventory",
    	               "default_replication_method": "LOG_BASED"
                    },
                "target":{
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
            panic!("Node is not a singer")
        };

        assert_eq!(tabular.identifier, "silver.inventory.factOrder")
    }

    #[test]
    fn add_singer_branch() {
        let temp_dir = TempDir::new().unwrap();

        env::set_current_dir(temp_dir.path()).expect("Failed to set current work dir");
        std::env::current_dir().expect("Failed to sync workdir");

        let bronze_path = temp_dir.path().join("bronze");
        fs::create_dir(&bronze_path).expect("Failed to create directory");

        let bronze_inventory_path = bronze_path.join("inventory");
        fs::create_dir(&bronze_inventory_path).expect("Failed to create directory");

        let config_path = bronze_inventory_path.join(Path::new("postgres.singer.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                "image":"dashbook/pipelinewise-tap-postgres:sql",
                "tap":{
                       "host": "172.17.0.2",
    	               "port": 5432,
    	               "user": "postgres",
    	               "password": "$POSTGRES_PASSWORD",
    	               "dbname": "postgres",
    	               "filter_schemas": "inventory",
    	               "default_replication_method": "LOG_BASED"
                    },
                "target":{
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

        let dag = update_dag(&changes, None, "expenditures").expect("Failed to create dag");

        assert_eq!(dag.singers.len(), 1);
        assert_eq!(dag.map.len(), 1);

        let orders = dag
            .singers
            .get("bronze.inventory.orders")
            .expect("Failed to get graph index");

        assert_eq!(orders, "bronze.inventory.postgres");

        let singer = &dag.dag[*dag.map.get(orders).expect("Failed to get graph index")];

        let Node::Singer(singer) = singer else {
            panic!("Node is not a singer")
        };

        assert_eq!(singer.image, "dashbook/pipelinewise-tap-postgres:sql")
    }
}
