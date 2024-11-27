use std::{fmt::Debug, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use file::FileConfig;
use iceberg_rust::catalog::CatalogList;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use self::sql::SqlConfig;

pub mod file;
pub mod sql;

#[async_trait]
pub trait Plugin: Debug {
    async fn catalog_list(&self) -> Result<Arc<dyn CatalogList>, Error>;
    fn bucket(&self, catalog_name: &str) -> &str;
    fn refresh_image(&self) -> &str;
    fn refresh_config(&self, identifier: &str, branch: &str) -> Result<String, Error>;
    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error>;
    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error>;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "plugin", rename_all = "lowercase")]
pub enum Config {
    Sql(SqlConfig),
    File(FileConfig),
}
