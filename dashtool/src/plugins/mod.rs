use std::sync::Arc;

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use iceberg_rust::catalog::CatalogList;

use crate::error::Error;

pub mod dashbook;
pub mod sql;

#[async_trait]
pub trait Plugin {
    async fn catalog_list(&self) -> Result<Arc<dyn CatalogList>, Error>;
    fn bucket(&self, catalog_name: &str) -> Option<&str>;
    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error>;
    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error>;
}
