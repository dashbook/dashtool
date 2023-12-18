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
