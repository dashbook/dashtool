use std::{fs, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use iceberg_catalog_sql::SqlCatalog;
use iceberg_rust::catalog::Catalog;
use object_store::aws::AmazonS3Builder;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::Plugin;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub url: String,
    pub region: String,
    pub bucket: String,
    pub secret_name: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

pub struct SqlPlugin {
    catalog: Arc<dyn Catalog>,
}

impl SqlPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_json)?;

        let object_store = Arc::new(
            AmazonS3Builder::new()
                .with_region(&config.region)
                .with_bucket_name(&config.bucket)
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
        );

        let catalog = Arc::new(SqlCatalog::new(&config.url, &config.name, object_store).await?);

        Ok(SqlPlugin { catalog })
    }
}

#[async_trait]
impl Plugin for SqlPlugin {
    async fn catalog(
        &self,
        _table_namespace: &str,
        _table_name: &str,
    ) -> Result<Arc<dyn Catalog>, Error> {
        Ok(self.catalog.clone())
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
