use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::anyhow;
use dashbook_catalog::{get_catalog, get_role};
use datafusion_iceberg::sql::get_schema;
use futures::{channel::mpsc::unbounded, lock::Mutex, stream, SinkExt, StreamExt, TryStreamExt};
use git2::{BranchType, Delta, Diff, Repository};
use iceberg_rust::{
    catalog::{identifier::Identifier, tabular::Tabular as IcebergTabular, Catalog},
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
    config::{Config, State},
    dag::{Dag, Node, Singer, Tabular},
    error::Error,
    git::{branch, diff},
};

use self::openid::{authorization, fetch_refresh_token, get_refresh_token};

mod openid;

pub async fn build() -> Result<(), Error> {
    // Load config files
    let config_json = fs::read_to_string("dashtool.json")?;
    let config: Arc<Config> = Arc::new(serde_json::from_str(&config_json)?);

    let refresh_token = get_refresh_token(&config).await?;

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
        config.clone(),
        "main",
        merged_branch.as_deref(),
        &refresh_token,
    )
    .await?;

    let last_diff = diff(&repo, &main_id, &last_id)?;

    let mut dag = update_dag(last_diff, Some(dag))?;

    let diff = diff(&repo, &last_id, &current_id)?;

    build_dag(
        &mut dag,
        diff,
        config.clone(),
        &current_branch,
        None,
        &refresh_token,
    )
    .await?;

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
    config: Arc<Config>,
    branch: &str,
    merged_branch: Option<&str>,
    refresh_token: &str,
) -> Result<(), Error> {
    let catalogs: Arc<Mutex<HashMap<String, Arc<dyn Catalog>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let catalog_name = config.catalog.split("/").last().ok_or(Error::Text(
        "Catalog url doesn't contain catalog name.".to_string(),
    ))?;

    let (tabular_sender, tabular_reciever) = unbounded();
    let (singer_sender, singer_reciever) = unbounded();

    let (access_token, id_token) =
        match authorization(&config.issuer, &config.client_id, refresh_token).await {
            Ok((a, i)) => (Arc::new(a), Arc::new(i)),
            Err(_) => {
                let refresh_token = fetch_refresh_token(&config).await?;
                let (a, i) =
                    authorization(&config.issuer, &config.client_id, &refresh_token).await?;
                (Arc::new(a), Arc::new(i))
            }
        };

    stream::iter(diff.deltas())
        .map(Ok::<_, Error>)
        .try_for_each(|delta| {
            let catalogs = catalogs.clone();
            let config = config.clone();
            let access_token = access_token.clone();
            let id_token = id_token.clone();
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

                        let role = get_role(
                            &access_token,
                            &catalog_name,
                            table_namespace,
                            table_name,
                            "write",
                        )
                        .await?;

                        let catalog = {
                            let mut catalogs = catalogs.lock().await;
                            match catalogs.get(&role) {
                                Some(catalog) => catalog.clone(),
                                None => {
                                    let catalog = get_catalog(
                                        &config.catalog,
                                        &access_token,
                                        &id_token,
                                        &table_namespace,
                                        &table_name,
                                        &role,
                                    )
                                    .await?;
                                    catalogs.insert(role, catalog.clone());
                                    catalog
                                }
                            }
                        };

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
                                let base_path = config.bucket.to_string()
                                    + "/"
                                    + path
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
                                builder.location(&base_path);
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
