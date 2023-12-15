use argo_workflow::schema::{
    EnvVarBuilder, EnvVarSourceBuilder, SecretKeySelectorBuilder, UserContainerBuilder,
    VolumeMountBuilder,
};
use std::{collections::HashMap, fs, sync::Arc};

use anyhow::anyhow;
use argo_workflow::schema::{IoArgoprojWorkflowV1alpha1UserContainer, IoK8sApiCoreV1Volume};
use async_trait::async_trait;
use iceberg_catalog_sql::SqlCatalogList;
use iceberg_rust::catalog::CatalogList;
use object_store::{aws::AmazonS3Builder, local::LocalFileSystem, memory::InMemory, ObjectStore};
use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::{ObjectStoreConfig, Plugin};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(flatten)]
    pub object_store: ObjectStoreConfig,
    pub url: String,
    pub bucket: Option<String>,
    pub secrets: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug)]
pub struct SqlPlugin {
    config: Config,
    catalog_list: Arc<dyn CatalogList>,
}

impl SqlPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_json)?;

        let object_store: Arc<dyn ObjectStore> = match &config.object_store {
            ObjectStoreConfig::Memory => Arc::new(InMemory::new()),
            ObjectStoreConfig::FileSystem(path) => {
                Arc::new(LocalFileSystem::new_with_prefix(&path.path)?)
            }
            ObjectStoreConfig::S3(s3_config) => Arc::new(
                AmazonS3Builder::new()
                    .with_region(&s3_config.region)
                    .with_bucket_name(
                        config
                            .bucket
                            .clone()
                            .ok_or(Error::Anyhow(anyhow!("No bucket specified.")))?,
                    )
                    .with_access_key_id(&s3_config.access_key_id)
                    .with_secret_access_key(&s3_config.secret_access_key)
                    .build()?,
            ),
        };

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
        self.config.bucket.as_deref()
    }

    fn refresh_image(&self) -> &str {
        "ghcr.io/dashbook/refresh-iceberg-datafusion:sql"
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

        for (secret, map) in &self.config.secrets {
            for (key, value) in map {
                builder.env(vec![EnvVarBuilder::default()
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
                    .build()?]);
            }
        }

        Ok(Some(vec![builder.build()?]))
    }
    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error> {
        Ok(None)
    }
}
