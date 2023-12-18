use std::{fmt::Debug, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use iceberg_rust::catalog::CatalogList;
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub mod dashbook;
pub mod sql;

#[async_trait]
pub trait Plugin: Debug {
    async fn catalog_list(&self) -> Result<Arc<dyn CatalogList>, Error>;
    fn bucket(&self, catalog_name: &str) -> &str;
    fn refresh_image(&self) -> &str;
    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error>;
    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(
    from = "Option<ObjectStoreConfigSerde>",
    into = "Option<ObjectStoreConfigSerde>"
)]
pub enum ObjectStoreConfig {
    S3(S3Config),
    Memory,
}

/// Config for the s3 object-store. The secret_access_key is read from the environment variable
/// AWS_SECRET_ACCESS_KEY
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct S3Config {
    pub aws_access_key_id: String,
    pub aws_region: String,
}

impl From<Option<ObjectStoreConfigSerde>> for ObjectStoreConfig {
    fn from(value: Option<ObjectStoreConfigSerde>) -> Self {
        match value {
            None => ObjectStoreConfig::Memory,
            Some(value) => match value {
                ObjectStoreConfigSerde::S3(value) => ObjectStoreConfig::S3(value),
            },
        }
    }
}

impl From<ObjectStoreConfig> for Option<ObjectStoreConfigSerde> {
    fn from(value: ObjectStoreConfig) -> Self {
        match value {
            ObjectStoreConfig::Memory => None,
            ObjectStoreConfig::S3(value) => Some(ObjectStoreConfigSerde::S3(value)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectStoreConfigSerde {
    S3(S3Config),
}
