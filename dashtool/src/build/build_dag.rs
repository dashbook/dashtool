use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::anyhow;
use datafusion_iceberg::sql::get_schema;
use futures::{channel::mpsc::unbounded, stream, SinkExt, StreamExt, TryStreamExt};
use git2::{Delta, Diff};
use iceberg_rust::{
    catalog::{identifier::Identifier, tabular::Tabular as IcebergTabular},
    error::Error as IcebergError,
    materialized_view::materialized_view_builder::MaterializedViewBuilder,
    sql::find_relations,
};
use iceberg_rust_spec::spec::{
    schema::Schema,
    snapshot::{Reference, Retention},
    view_metadata::REF_PREFIX,
};
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::{
    dag::{Dag, Node, Singer, Tabular},
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
                let is_tabular = if path.ends_with(".sql") {
                    Some(true)
                } else if path.ends_with("target.json") {
                    Some(false)
                } else {
                    None
                };
                match is_tabular {
                    Some(true) => {
                        let table_name = path
                            .file_name()
                            .ok_or(Error::Text("Path doesn't contain file.".to_string()))?
                            .to_str()
                            .unwrap()
                            .trim_end_matches(".sql");
                        let table_namespace = path
                            .parent()
                            .ok_or(Error::Text("Path doesn't have parent folder.".to_string()))?
                            .to_str()
                            .unwrap();
                        let identifier = table_namespace.to_string() + "." + table_name;

                        let catalog = plugin.catalog(table_namespace, table_name).await?;

                        let sql = fs::read_to_string(path)?;
                        let relations = find_relations(&sql)?;

                        match (delta.status(), merged_branch) {
                            (Delta::Added | Delta::Modified, Some(merged_branch)) => {
                                let tabular =
                                    catalog.load_table(&Identifier::parse(&identifier)?).await?;
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
                                let fields =
                                    get_schema(&sql, relations.clone(), catalog.clone()).await?;
                                let schema = Schema {
                                    schema_id: 1,
                                    identifier_field_ids: None,
                                    fields,
                                };
                                let base_path = path
                                    .to_str()
                                    .ok_or(Error::Text("No new file in delta".to_string()))?
                                    .trim_start_matches("/")
                                    .trim_end_matches(".sql");
                                let mut builder = MaterializedViewBuilder::new(
                                    &sql,
                                    &identifier,
                                    schema,
                                    catalog,
                                )?;
                                builder.location(base_path);
                                builder.build().await?;
                            }
                            _ => (),
                        }

                        tabular_sender.send((identifier, relations)).await?;
                    }
                    Some(false) => {
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
                        singer_sender
                            .send(Node::Singer(Singer::new(parent, tap, target, branch)))
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
        env,
        fs::{self, File},
        io::Write,
        path::Path,
        sync::Arc,
    };

    use git2::DiffOptions;

    use crate::{
        build::{build_dag::build_dag, update_dag::update_dag},
        dag::Node,
        plugins::sql::SqlPlugin,
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

        let mut dag = update_dag(last_main_diff, None).expect("Failed to create dag");

        let config_path = temp_dir.path().join(Path::new("dashtool.json"));
        File::create(&config_path)
            .expect("Failed to create file")
            .write_all(
                r#"
                {
                   "name": "iceberg",
	               "url": "sqlite://",
	               "region": "null",
	               "secretName": "null"
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
}
