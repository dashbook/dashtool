use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::anyhow;
use dashbook_catalog::{get_catalog, get_role};
use datafusion_iceberg::sql::get_schema;
use futures::{channel::mpsc::unbounded, lock::Mutex, stream, SinkExt, StreamExt, TryStreamExt};
use git2::{BranchType, Delta, Diff, Repository};
use iceberg_rust::{
    catalog::Catalog, spec::schema::Schema, sql::find_relations, view::view_builder::ViewBuilder,
};
use target_iceberg_nessie::config::Config as SingerConfig;

use crate::{
    config::{Config, State},
    dag::{get_dag, Dag, Node, Singer, Tabular},
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

    let state: Option<State> = fs::read_to_string(".dashtool/state.json")
        .ok()
        .and_then(|x| serde_json::from_str(&x).ok());

    // Inspect git repo
    let repo = Repository::open(".")?;

    let branch = branch(&repo)?;

    let current_id = repo
        .find_branch(&branch, BranchType::Local)?
        .into_reference()
        .target();

    let last_id = state.and_then(|state| state.commits.get(&branch).cloned());

    let diff = diff(&repo, &last_id, &current_id)?;

    let branch_dag = create_dag(diff, config.clone(), &branch, &refresh_token).await?;

    let json = serde_json::to_string(&branch_dag)?;

    fs::write(".dashtool/dags/".to_string() + &branch + ".json", json)?;

    Ok(())
}

async fn create_dag<'repo>(
    diff: Diff<'repo>,
    config: Arc<Config>,
    branch: &str,
    refresh_token: &str,
) -> Result<Dag, Error> {
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
                match (delta.status(), is_tabular) {
                    (Delta::Added, Some(true)) => {
                        let table_name = path
                            .file_name()
                            .ok_or(Error::Text("Path doesn't contain file.".to_string()))?
                            .to_str()
                            .unwrap();
                        let table_namespace = path
                            .parent()
                            .ok_or(Error::Text("Path doesn't have parent folder.".to_string()))?
                            .to_str()
                            .unwrap();

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
                        let fields = get_schema(&sql, relations.clone(), catalog.clone()).await?;
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
                        let identifier = path
                            .to_str()
                            .ok_or(Error::Text("No new file in delta".to_string()))?
                            .trim_start_matches("/")
                            .trim_end_matches(".sql")
                            .replace("/", ".");
                        let mut builder = ViewBuilder::new(&sql, &identifier, schema, catalog)?;
                        builder.location(&base_path);
                        builder.build().await?;
                        tabular_sender.send((identifier, relations)).await?;
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
                        singer_sender
                            .send(Node::Singer(Singer::new(parent, tap, target)))
                            .await?;
                    }
                    (_, None) => (),
                    _ => (),
                }
                Ok(())
            }
        })
        .await?;

    tabular_sender.close_channel();
    singer_sender.close_channel();

    let mut dag = get_dag(branch)?;

    let singers = singer_reciever.collect::<Vec<_>>().await;

    let tabs: HashMap<String, Vec<String>> =
        HashMap::from_iter(tabular_reciever.collect::<Vec<_>>().await);

    for singer in singers {
        dag.add_node(singer)
    }

    for node in tabs.keys() {
        dag.add_node(Node::Tabular(Tabular::new(node)))
    }

    for (node, children) in tabs {
        for child in children {
            dag.add_edge(&node, &child)?
        }
    }

    Ok(dag)
}
