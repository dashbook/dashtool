use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::anyhow;
use datafusion_iceberg::sql::get_schema;
use futures::{channel::mpsc::unbounded, stream, SinkExt, StreamExt, TryStreamExt};
use git2::{BranchType, Delta, Diff, Repository};
use iceberg_rust::{
    catalog::{identifier::Identifier, tabular::Tabular as IcebergTabular},
    error::Error as IcebergError,
    materialized_view::materialized_view_builder::MaterializedViewBuilder,
    spec::{
        schema::Schema,
        snapshot::{Reference, Retention},
        view_metadata::REF_PREFIX,
    },
    sql::find_relations,
};
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::{
    dag::{Dag, Node, Singer, Tabular},
    error::Error,
    git::{branch, diff},
    plugins::Plugin,
    state::State,
};

pub async fn build(plugin: Arc<dyn Plugin>) -> Result<(), Error> {
    let state: State = fs::read_to_string(".dashtool/state.json")
        .ok()
        .and_then(|x| serde_json::from_str(&x).ok())
        .unwrap_or_default();

    // Inspect git repo
    let repo = Repository::open(".")?;

    // Name of the currently selected branch in the git repo
    let current_branch = branch(&repo)?;

    let current_id = repo
        .find_branch(&current_branch, BranchType::Local)?
        .into_reference()
        .target();

    let main_id = repo
        .find_branch("main", BranchType::Local)?
        .into_reference()
        .target();

    // Id of the last commit on the current brranch that was built with dashtool
    let last_id = state.commits.get(&current_branch).cloned();

    // Id of the last commit on the main branch that was built with dashtool
    let last_main_id = state.commits.get("main").cloned();

    // Check if dashtool built a branch with the same commit as the current main branch to see if the branch was merged
    let merged_branch = state
        .commits
        .iter()
        .find(|(k, v)| {
            if let Some(y) = &main_id {
                *v == y && *k != "main"
            } else {
                false
            }
        })
        .map(|(k, _)| k)
        .cloned();

    let last_main_diff = diff(&repo, &None, &main_id)?;

    let mut dag = update_dag(last_main_diff, None)?;

    let main_diff = diff(&repo, &last_main_id, &main_id)?;

    build_dag(
        &mut dag,
        main_diff,
        plugin.clone(),
        "main",
        merged_branch.as_deref(),
    )
    .await?;

    let last_diff = diff(&repo, &main_id, &last_id)?;

    let mut dag = update_dag(last_diff, Some(dag))?;

    let diff = diff(&repo, &last_id, &current_id)?;

    build_dag(&mut dag, diff, plugin.clone(), &current_branch, None).await?;

    let json = serde_json::to_string(&dag)?;

    fs::write(
        ".dashtool/dags/".to_string() + &current_branch + ".json",
        json,
    )?;

    Ok(())
}

// Converts commits into a dag and performs the according transactions on the tables
async fn build_dag<'repo>(
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

// Converts the commits into a dag without performing any operations on the tables
fn update_dag<'repo>(diff: Diff<'repo>, dag: Option<Dag>) -> Result<Dag, Error> {
    let mut dag = dag.unwrap_or(Dag::new());
    for delta in diff.deltas() {
        dbg!(&delta.new_file());
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

    use crate::{dag::Node, test::repo_init};

    use super::update_dag;

    #[test]
    fn update_add_singer() {
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
