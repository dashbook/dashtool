use std::{collections::HashMap, fs, sync::Arc};

use dashbook_catalog::{get_catalog, get_role};
use futures::lock::Mutex;
use git2::{Delta, Repository};
use iceberg_rust::catalog::Catalog;

use crate::{
    config::{Config, State},
    error::Error,
};

use self::openid::authorization;

mod openid;
mod sql;

pub async fn run() -> Result<(), Error> {
    // Load config files
    let config_json = fs::read_to_string("dashtool.json")?;
    let config: Config = serde_json::from_str(&config_json)?;

    let state_json = fs::read_to_string(".dashtool/state.json")?;
    let state: State = serde_json::from_str(&state_json)?;

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
                } else if path.ends_with("target.json") {
                    ()
                }
            }
            _ => (),
        }
    }

    Ok(())
}
