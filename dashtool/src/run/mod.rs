use std::{collections::HashMap, fs, sync::Arc};

use dashbook_catalog::{get_catalog, get_role};
use futures::{channel::mpsc::unbounded, lock::Mutex, stream, SinkExt, StreamExt, TryStreamExt};
use git2::{Delta, Repository};
use iceberg_rust::{
    catalog::{identifier::Identifier, Catalog},
    model::schema::SchemaV2,
    view::view_builder::ViewBuilder,
};

use crate::{
    config::{Config, State},
    dag::{get_dag, Node},
    error::Error,
    git::diff,
    sql::{find_relations, get_schema},
};

use self::openid::authorization;

mod openid;

pub async fn run() -> Result<(), Error> {
    // Load config files
    let config_json = fs::read_to_string("dashtool.json")?;
    let config: Arc<Config> = Arc::new(serde_json::from_str(&config_json)?);

    let state_json = fs::read_to_string(".dashtool/state.json")?;
    let state: State = serde_json::from_str(&state_json)?;

    // Inspect git repo
    let repo = Repository::open(".")?;

    let diff = diff(&repo, &state.last_commit)?;

    let catalogs: Arc<Mutex<HashMap<String, Arc<dyn Catalog>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let catalog_name = config.catalog.split("/").last().ok_or(Error::Text(
        "Catalog url doesn't contain catalog name.".to_string(),
    ))?;

    let (node_sender, node_reciever) = unbounded();

    let (access_token, id_token) = authorization(&config.issuer, &config.client_id)
        .await
        .map(|(a, i)| (Arc::new(a), Arc::new(i)))?;

    stream::iter(diff.deltas())
        .map(Ok::<_, Error>)
        .try_for_each_concurrent(Some(8), |delta| {
            let catalogs = catalogs.clone();
            let config = config.clone();
            let access_token = access_token.clone();
            let id_token = id_token.clone();
            let node_sender = node_sender.clone();
            async move {
                match delta.status() {
                    Delta::Added => {
                        let path = delta
                            .new_file()
                            .path()
                            .ok_or(Error::Text("No new file in delta".to_string()))?;
                        if path.ends_with(".sql") {
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
                            let fields =
                                get_schema(&sql, relations.clone(), catalog.clone()).await?;
                            let schema = SchemaV2 {
                                schema_id: 0,
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
                            ViewBuilder::new_metastore_view(
                                &sql,
                                &base_path,
                                schema,
                                Identifier::parse(&identifier)?,
                                catalog,
                            )?
                            .commit()
                            .await?;
                            node_sender.clone().send((identifier, relations)).await?;
                        } else if path.ends_with("target.json") {
                            ()
                        }
                    }
                    _ => (),
                }
                Ok(())
            }
        })
        .await?;

    node_sender.close_channel();

    let mut dag = get_dag()?;

    let nodes: HashMap<String, Vec<String>> =
        HashMap::from_iter(node_reciever.collect::<Vec<_>>().await);

    for node in nodes.keys() {
        dag.add_node(Node::new(node))
    }

    for (node, children) in nodes {
        for child in children {
            dag.add_edge(&node, &child)?
        }
    }

    let json = serde_json::to_string(&dag)?;

    fs::write(".dashtool/dag.json", json)?;

    Ok(())
}
