use std::{collections::HashMap, fs, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use iceberg_catalog_sql::SqlCatalog;
use iceberg_rust::catalog::{identifier::Identifier, Catalog};
use object_store::{aws::AmazonS3Builder, memory::InMemory, ObjectStore};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::Plugin;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub url: String,
    pub region: String,
    pub bucket: Option<String>,
    pub secret_name: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub includes: Vec<Config>,
}

pub struct SqlPlugin {
    catalog: Arc<dyn Catalog>,
    includes: HashMap<String, Arc<dyn Catalog>>,
}

impl SqlPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_json)?;

        let object_store = if let Some(bucket) = config.bucket {
            Ok(Arc::new(
                AmazonS3Builder::new()
                    .with_region(&config.region)
                    .with_bucket_name(&bucket)
                    .with_access_key_id(
                        config
                            .access_key_id
                            .as_ref()
                            .ok_or(Error::NoToken("ACCESS_KEY_ID".to_string()))?,
                    )
                    .with_secret_access_key(
                        config
                            .secret_access_key
                            .as_ref()
                            .ok_or(Error::NoToken("SECRET_ACCESS_KEY".to_string()))?,
                    )
                    .build()?,
            ) as Arc<dyn ObjectStore>)
        } else {
            // Enables InMemory ObjectStore for tests
            if cfg!(test) {
                Ok(Arc::new(InMemory::new()) as Arc<dyn ObjectStore>)
            } else {
                Err(Error::NoToken("BUCKET".to_string()))
            }
        }?;

        let catalog = Arc::new(SqlCatalog::new(&config.url, &config.name, object_store).await?);

        Ok(SqlPlugin {
            catalog,
            // TODO!!
            includes: HashMap::new(),
        })
    }
}

#[cfg(test)]
impl SqlPlugin {
    pub(crate) fn new_with_catalogs(
        catalog: Arc<dyn Catalog>,
        includes: HashMap<String, Arc<dyn Catalog>>,
    ) -> Result<Self, Error> {
        Ok(SqlPlugin { catalog, includes })
    }
}

#[async_trait]
impl Plugin for SqlPlugin {
    async fn catalog(
        &self,
        catalog_name: &str,
        table_namespace: &str,
        table_name: &str,
    ) -> Result<Arc<dyn Catalog>, Error> {
        let identifier = Identifier::parse(&(table_namespace.to_string() + "." + table_name))?;
        let catalog = Box::pin(
            stream::iter(self.includes.iter()).filter_map(|(name, catalog)| {
                let identifier = identifier.clone();
                async move {
                    catalog
                        .clone()
                        .table_exists(&identifier)
                        .await
                        .map(|x| {
                            if x && name == catalog_name {
                                Some(catalog)
                            } else {
                                None
                            }
                        })
                        .transpose()
                }
            }),
        )
        .next()
        .await
        .transpose()?;

        Ok(catalog.cloned().unwrap_or(self.catalog.clone()))
    }
    fn bucket(&self) -> &str {
        unimplemented!()
    }
    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error> {
        Ok(None)
    }
    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error> {
        Ok(None)
    }
}
