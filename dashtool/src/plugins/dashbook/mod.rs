use std::{collections::HashMap, fs, sync::Arc};

use argo_workflow::schema::{
    ConfigMapVolumeSourceBuilder, IoArgoprojWorkflowV1alpha1UserContainer,
    IoK8sApiCoreV1EmptyDirVolumeSource, IoK8sApiCoreV1Volume, SecretVolumeSourceBuilder,
    UserContainerBuilder, VolumeBuilder, VolumeMountBuilder,
};
use async_trait::async_trait;
use dashbook_catalog::DashbookS3CatalogList;
use iceberg_rust::catalog::CatalogList;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use self::openid::{authorization, get_refresh_token};

use super::Plugin;

mod openid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// A map from catalog_name to bucket
    pub buckets: HashMap<String, String>,
    pub issuer: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Debug)]
pub struct DashbookPlugin {
    config: Config,
    catalog_list: Arc<dyn CatalogList>,
}

impl DashbookPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_json)?;

        let refresh_token = get_refresh_token(&config).await?;

        let issuer = config
            .issuer
            .as_deref()
            .unwrap_or("https://auth.dashbook.dev/realms/dashbook");

        let client_id = config.issuer.as_deref().unwrap_or("dashbook");

        let (access_token, id_token) = authorization(issuer, client_id, &refresh_token).await?;

        let catalog_list = Arc::new(DashbookS3CatalogList::new(&access_token, &id_token).await?);

        Ok(DashbookPlugin {
            config,
            catalog_list,
        })
    }
}

#[async_trait]
impl Plugin for DashbookPlugin {
    async fn catalog_list(&self) -> Result<Arc<dyn CatalogList>, Error> {
        Ok(self.catalog_list.clone())
    }

    fn bucket(&self, catalog_name: &str) -> &str {
        self.config
            .buckets
            .get(catalog_name)
            .map(|x| x.as_str())
            .unwrap()
    }

    fn refresh_image(&self) -> &str {
        "dashbook/refresh-iceberg-datafusion:dashbook"
    }

    fn refresh_config(&self, _identifier: &str, _branch: &str) -> Result<String, Error> {
        Ok("dashbook/refresh-iceberg-datafusion:dashbook".to_owned())
    }

    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error> {
        Ok(Some(vec![
            UserContainerBuilder::default()
                .name("authorization".to_string())
                .image(Some(
                    "dashbook/dashtool-authorization".to_string(),
                ))
                .volume_mounts(vec![
                    VolumeMountBuilder::default()
                        .name("authorization".to_string())
                        .mount_path("/tmp/authorization".to_string())
                        .build()?,
                    VolumeMountBuilder::default()
                        .name("authentication".to_string())
                        .mount_path("/tmp/authentication".to_string())
                        .build()?,
                ])
                .build()?,
            UserContainerBuilder::default()
                .name("envsubst".to_string())
                .image(Some("dibi/envsubst".to_string()))
                .command(vec!["/bin/sh".to_string()])
                .args(vec!["-c".to_string(),"export ACCESS_TOKEN=$(cat /tmp/authorization/access.jwt) && export ID_TOKEN=$(cat /tmp/authorization/id.jwt) && /envsubst-file.sh".to_string()])
                .volume_mounts(vec![
                    VolumeMountBuilder::default()
                        .name("authorization".to_string())
                        .mount_path("/tmp/authorization".to_string())
                        .build()?,
                    VolumeMountBuilder::default()
                        .name("config_template".to_string())
                        .mount_path("/workdir".to_string())
                        .build()?,
                    VolumeMountBuilder::default()
                        .name("config".to_string())
                        .mount_path("/processed".to_string())
                        .build()?,
                ])
                .build()?,
        ]))
    }

    fn volumes(&self) -> Result<Option<Vec<IoK8sApiCoreV1Volume>>, Error> {
        Ok(Some(vec![
            VolumeBuilder::default()
                .name("authorization".to_string())
                .empty_dir(Some(IoK8sApiCoreV1EmptyDirVolumeSource::default()))
                .build()?,
            VolumeBuilder::default()
                .name("authentication".to_string())
                .secret(Some(
                    SecretVolumeSourceBuilder::default()
                        .secret_name(Some("dashtool-authentication".to_string()))
                        .build()?,
                ))
                .build()?,
            VolumeBuilder::default()
                .name("config".to_string())
                .empty_dir(Some(IoK8sApiCoreV1EmptyDirVolumeSource::default()))
                .build()?,
            VolumeBuilder::default()
                .name("config_template".to_string())
                .config_map(Some(ConfigMapVolumeSourceBuilder::default().build()?))
                .build()?,
        ]))
    }
}
