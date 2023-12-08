use std::{fs, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use iceberg_catalog_sql::SqlCatalogList;
use iceberg_rust::catalog::CatalogList;
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
    catalog_list: Arc<dyn CatalogList>,
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

        let catalog_list = Arc::new(SqlCatalogList::new(&config.url, object_store).await?);

        Ok(SqlPlugin {
            config,
            catalog_list,
        })
    }
}

#[cfg(test)]
impl SqlPlugin {
    pub(crate) fn new_with_catalog(
        config: Config,
        catalog_list: Arc<SqlCatalogList>,
    ) -> Result<Self, Error> {
        Ok(SqlPlugin {
            config,
            catalog_list,
        })
    }
}

#[async_trait]
impl Plugin for SqlPlugin {
    async fn catalog_list(&self) -> Result<Arc<dyn CatalogList>, Error> {
        Ok(self.catalog_list.clone())
    }
    fn bucket(&self, _catalog_name: &str) -> Option<&str> {
        Some(&self.config.bucket)
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
