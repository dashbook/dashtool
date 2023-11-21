use std::{fs, path::Path};

use anyhow::anyhow;
use git2::{Delta, Diff};
use iceberg_rust::sql::find_relations;
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::{
    dag::{Dag, Node, Singer, Tabular},
    error::Error,
};

// Converts the commits into a dag without performing any operations on the tables
pub(super) fn update_dag<'repo>(diff: Diff<'repo>, dag: Option<Dag>) -> Result<Dag, Error> {
    let mut dag = dag.unwrap_or(Dag::new());
    for delta in diff.deltas() {
        let path = delta
            .new_file()
            .path()
            .ok_or(Error::Text("No new file in delta".to_string()))?;
        let is_tabular = if path.ends_with(".sql") {
            Some(true)
        } else if path.ends_with("target.json") {
            Some(false)
        } else {
            None
        };
        match (delta.status(), is_tabular) {
            (Delta::Added, Some(true)) => {
                let sql = fs::read_to_string(path)?;

                let children = find_relations(&sql)?;

                let identifier = path
                    .to_str()
                    .ok_or(Error::Text("No new file in delta".to_string()))?
                    .trim_start_matches("/")
                    .trim_end_matches(".sql")
                    .replace("/", ".");

                dag.add_node(Node::Tabular(Tabular::new(&identifier, "main")));

                for child in children {
                    dag.add_edge(&identifier, &child)?
                }
            }
            (Delta::Added, Some(false)) => {
                let parent = Path::new(path)
                    .parent()
                    .ok_or(Error::Anyhow(anyhow!(
                        "target.json must be inside a subfolder."
                    )))?
                    .to_str()
                    .ok_or(Error::Anyhow(anyhow!("Failed to convert path to string.")))?;
                let tap_path = parent.to_string() + "/tap.json";
                let tap = fs::read_to_string(&tap_path)?;
                let target_json = fs::read_to_string(path)?;
                let target: SingerConfig = serde_json::from_str(&target_json)?;
                dag.add_node(Node::Singer(Singer::new(parent, tap, target, "main")));
            }
            (Delta::Added, None) => (),
            _ => (),
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

    use git2::DiffOptions;

    use crate::{build::update_dag::update_dag, dag::Node, test::repo_init};

    #[test]
    fn add_singer() {
        let (temp_dir, repo) = repo_init();

        env::set_current_dir(temp_dir.path()).expect("Failed to set current work dir");

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

        let diff = repo
            .diff_tree_to_index(None, Some(&index), Some(&mut opts))
            .expect("Failed to create diff for repo");

        let dag = update_dag(diff, None).expect("Failed to create dag");

        assert_eq!(dag.singers.len(), 1);

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
        )
    }
}
