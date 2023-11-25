use std::{collections::HashMap, fs, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use futures::lock::Mutex;
use iceberg_catalog_sql::SqlCatalog;
use iceberg_rust::catalog::Catalog;
use object_store::{aws::AmazonS3Builder, ObjectStore};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::Plugin;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub url: String,
    pub region: String,
    pub bucket: String,
    pub secret_name: String,
}

pub struct SqlPlugin {
    config: Config,
    catalog: Arc<SqlCatalog>,
    catalogs: Arc<Mutex<HashMap<String, Arc<dyn Catalog>>>>,
}

impl SqlPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_json)?;

        let object_store = Arc::new(
            AmazonS3Builder::from_env()
                .with_region(&config.region)
                .with_bucket_name(&config.bucket)
                .build()?,
        ) as Arc<dyn ObjectStore>;

        let catalog = Arc::new(SqlCatalog::new(&config.url, "__template", object_store).await?);

        let catalogs: Arc<Mutex<HashMap<String, Arc<dyn Catalog>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        Ok(SqlPlugin {
            config,
            catalog,
            catalogs,
        })
    }
}

#[cfg(test)]
impl SqlPlugin {
    pub(crate) fn new_with_catalog(
        config: Config,
        catalog: Arc<SqlCatalog>,
    ) -> Result<Self, Error> {
        let catalogs: Arc<Mutex<HashMap<String, Arc<dyn Catalog>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        Ok(SqlPlugin {
            config,
            catalog,
            catalogs,
        })
    }
}

#[async_trait]
impl Plugin for SqlPlugin {
    async fn catalog(
        &self,
        catalog_name: &str,
        _table_namespace: &str,
        _table_name: &str,
    ) -> Result<Arc<dyn Catalog>, Error> {
        let catalog_name = catalog_name.to_owned();

        let catalog = {
            let mut catalogs = self.catalogs.lock().await;
            catalogs
                .entry(catalog_name.clone())
                .or_insert(Arc::new(self.catalog.duplicate(&catalog_name)))
                .clone()
        };
        Ok(catalog)
    }
    fn bucket(&self) -> &str {
        &self.config.bucket
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
