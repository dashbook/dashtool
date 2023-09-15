use std::{collections::HashMap, fs, sync::Arc};

use dashbook_catalog::{get_catalog, get_role};
use futures::lock::Mutex;
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
};

use self::{
    openid::authorization,
    sql::{find_relations, get_schema},
};

mod openid;
mod sql;

pub async fn run() -> Result<(), Error> {
    // Load config files
    let config_json = fs::read_to_string("dashtool.json")?;
    let config: Config = serde_json::from_str(&config_json)?;

    let state_json = fs::read_to_string(".dashtool/state.json")?;
    let state: State = serde_json::from_str(&state_json)?;

    let mut dag = get_dag()?;

    // Inspect git repo
    let repo = Repository::open(".")?;

    let previous_id = state.last_commit.as_ref();

    let current_object = repo.revparse_single("HEAD")?;

    let current_id = current_object.id();

    let previous_commit = previous_id
        .cloned()
        .map(|x| repo.find_commit(x))
        .transpose()?
        .map(|x| x.tree())
        .transpose()?;

    let current_commit = repo.find_commit(current_id)?.tree()?;

    let diff = repo.diff_tree_to_tree(previous_commit.as_ref(), Some(&current_commit), None)?;

    let deltas = diff.deltas();

    let catalogs: Arc<Mutex<HashMap<String, Arc<dyn Catalog>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let catalog_name = config.catalog.split("/").last().ok_or(Error::Text(
        "Catalog url doesn't contain catalog name.".to_string(),
    ))?;

    let (access_token, id_token) = authorization(&config.issuer, &config.client_id).await?;

    for delta in deltas {
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
                    let fields = get_schema(&sql, relations.clone(), catalog.clone()).await?;
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
                    dag.add_node(Node::new(&identifier), &relations);
                } else if path.ends_with("target.json") {
                    ()
                }
            }
            _ => (),
        }
    }

    let json = serde_json::to_string(&dag)?;

    fs::write(".dashtool/dag.json", json)?;

    Ok(())
}
