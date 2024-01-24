use argo_workflow::schema::{
    EnvVarBuilder, EnvVarSourceBuilder, SecretKeySelectorBuilder, UserContainerBuilder,
    VolumeMountBuilder,
};
use dashtool_common::ObjectStoreConfig;
use std::{collections::HashMap, fs, sync::Arc};

use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use iceberg_catalog_sql::SqlCatalogList;
use iceberg_rust::catalog::CatalogList;
use object_store::{aws::AmazonS3Builder, memory::InMemory, ObjectStore};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::Plugin;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(flatten)]
    pub object_store: ObjectStoreConfig,
    pub catalog_url: String,
    pub bucket: String,
    /// A nested map that maps a kubernetes secret name to a map from a environement name to the
    /// key of the secret value in the secret.
    pub secrets: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug)]
pub struct SqlPlugin {
    config: Config,
    catalog_list: Arc<dyn CatalogList>,
}

impl SqlPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let mut config: Config = serde_json::from_str(&config_json)?;

        let mut full_bucket_name = config.bucket.clone();
        let object_store: Arc<dyn ObjectStore> = match &config.object_store {
            ObjectStoreConfig::Memory => Arc::new(InMemory::new()),
            ObjectStoreConfig::S3(s3_config) => {
                full_bucket_name = "s3://".to_owned() + config.bucket.trim_start_matches("s3://");

                let mut builder = AmazonS3Builder::from_env()
                    .with_region(&s3_config.aws_region)
                    .with_bucket_name(config.bucket.clone())
                    .with_access_key_id(&s3_config.aws_access_key_id);

                if let Some(endpoint) = &s3_config.aws_endpoint {
                    builder = builder.with_endpoint(endpoint);
                }

                Arc::new(builder.build()?)
            }
        };

        config.bucket = full_bucket_name;

        let catalog_list = Arc::new(SqlCatalogList::new(&config.catalog_url, object_store).await?);

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshConfig {
    #[serde(flatten)]
    pub object_store: ObjectStoreConfig,
    pub catalog_url: String,
    pub identifier: String,
    pub bucket: Option<String>,
    pub branch: Option<String>,
}

#[async_trait]
impl Plugin for SqlPlugin {
    async fn catalog_list(&self) -> Result<Arc<dyn CatalogList>, Error> {
        Ok(self.catalog_list.clone())
    }

    fn bucket(&self, _catalog_name: &str) -> &str {
        &self.config.bucket
    }

    fn refresh_image(&self) -> &str {
        "ghcr.io/dashbook/refresh-iceberg-datafusion:sql"
    }

    fn refresh_config(&self, identifier: &str, branch: &str) -> Result<String, Error> {
        let mut object_store_config = self.config.object_store.clone();
        match &mut object_store_config {
            ObjectStoreConfig::S3(config) => {
                config.aws_secret_access_key = Some("$AWS_SECRET_ACCESS_KEY".to_owned())
            }
            _ => (),
        }
        let config = RefreshConfig {
            identifier: identifier.to_owned(),
            branch: Some(branch.to_owned()),
            object_store: object_store_config,
            catalog_url: self
                .config
                .env
                .get("CATALOG_URL")
                .cloned()
                .unwrap_or(self.config.catalog_url.clone()),
            bucket: Some(self.config.bucket.clone()),
        };
        Ok(serde_json::to_string(&config).unwrap())
    }

    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error> {
        let mut builder = UserContainerBuilder::default();
        builder
            .name("envsubst".to_string())
            .image(Some("dibi/envsubst".to_string()))
            .volume_mounts(vec![
                VolumeMountBuilder::default()
                    .name("config-template".to_string())
                    .mount_path("/workdir".to_string())
                    .build()?,
                VolumeMountBuilder::default()
                    .name("config".to_string())
                    .mount_path("/processed".to_string())
                    .build()?,
            ]);

        builder.env(
            self.config
                .secrets
                .iter()
                .flat_map(|(secret, map)| {
                    map.iter().map(|(key, value)| {
                        Ok(EnvVarBuilder::default()
                            .name(value.trim_start_matches('$').to_owned())
                            .value_from(Some(
                                EnvVarSourceBuilder::default()
                                    .secret_key_ref(Some(
                                        SecretKeySelectorBuilder::default()
                                            .name(Some(secret.clone()))
                                            .key(key.clone())
                                            .optional(None)
                                            .build()?,
                                    ))
                                    .config_map_key_ref(None)
                                    .field_ref(None)
                                    .resource_field_ref(None)
                                    .build()?,
                            ))
                            .value(None)
                            .build()?)
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?,
        );

        Ok(Some(vec![builder.build()?]))
    }
    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error> {
        Ok(None)
    }
}
