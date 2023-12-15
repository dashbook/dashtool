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
    fn bucket(&self, catalog_name: &str) -> Option<&str>;
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
    FileSystem(FileSystemConfig),
    S3(S3Config),
    Memory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3Config {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileSystemConfig {
    pub path: String,
}

impl From<Option<ObjectStoreConfigSerde>> for ObjectStoreConfig {
    fn from(value: Option<ObjectStoreConfigSerde>) -> Self {
        match value {
            None => ObjectStoreConfig::Memory,
            Some(value) => match value {
                ObjectStoreConfigSerde::S3(value) => ObjectStoreConfig::S3(value),
                ObjectStoreConfigSerde::FileSystem(value) => ObjectStoreConfig::FileSystem(value),
            },
        }
    }
}

impl From<ObjectStoreConfig> for Option<ObjectStoreConfigSerde> {
    fn from(value: ObjectStoreConfig) -> Self {
        match value {
            ObjectStoreConfig::Memory => None,
            ObjectStoreConfig::S3(value) => Some(ObjectStoreConfigSerde::S3(value)),
            ObjectStoreConfig::FileSystem(value) => Some(ObjectStoreConfigSerde::FileSystem(value)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectStoreConfigSerde {
    FileSystem(FileSystemConfig),
    S3(S3Config),
}
