use std::{collections::HashMap, fs, sync::Arc};

use argo_workflow::schema::{
    ConfigMapVolumeSourceBuilder, IoArgoprojWorkflowV1alpha1UserContainer,
    IoK8sApiCoreV1EmptyDirVolumeSource, IoK8sApiCoreV1Volume, SecretVolumeSourceBuilder,
    UserContainerBuilder, VolumeBuilder, VolumeMountBuilder,
};
use async_trait::async_trait;
use dashbook_catalog::{get_catalog, get_role};
use futures::lock::Mutex;
use iceberg_rust::catalog::Catalog;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use self::openid::{authorization, get_refresh_token};

use super::Plugin;

mod openid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub catalog: String,
    pub bucket: String,
    pub includes: Vec<[String; 2]>,
    pub issuer: String,
    pub client_id: String,
}

pub struct DashbookPlugin {
    config: Config,
    access_token: String,
    id_token: String,
    catalogs: Arc<Mutex<HashMap<(String, String), Arc<dyn Catalog>>>>,
}

impl DashbookPlugin {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config_json = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_json)?;

        let refresh_token = get_refresh_token(&config).await?;

        let (access_token, id_token) =
            authorization(&config.issuer, &config.client_id, &refresh_token).await?;

        let catalogs: Arc<Mutex<HashMap<(String, String), Arc<dyn Catalog>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        Ok(DashbookPlugin {
            config,
            access_token,
            id_token,
            catalogs,
        })
    }
}

#[async_trait]
impl Plugin for DashbookPlugin {
    async fn catalog(
        &self,
        catalog_name: &str,
        table_namespace: &str,
        table_name: &str,
    ) -> Result<Arc<dyn Catalog>, Error> {
        let catalog_name = catalog_name.to_owned();
        let role = get_role(
            &self.access_token,
            &catalog_name,
            table_namespace,
            table_name,
            "write",
        )
        .await?;

        let catalog = {
            let mut catalogs = self.catalogs.lock().await;
            catalogs
                .entry((catalog_name.clone(), role.clone()))
                .or_insert(
                    get_catalog(
                        &self.config.catalog,
                        &self.access_token,
                        &self.id_token,
                        &table_namespace,
                        &table_name,
                        &role,
                    )
                    .await?,
                )
                .clone()
        };
        Ok(catalog)
    }
    fn bucket(&self) -> &str {
        &self.config.bucket
    }
    fn init_containters(
        &self,
    ) -> Result<Option<Vec<IoArgoprojWorkflowV1alpha1UserContainer>>, Error> {
        Ok(Some(vec![
            UserContainerBuilder::default()
                .name("authorization".to_string())
                .image(Some(
                    "ghcr.io/dashbook/dashtool-authorization".to_string(),
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
