use crate::{dag::Singer, error::Error, plugins::Plugin};

use argo_workflow::schema::{
    ConfigMapVolumeSourceBuilder, ContainerBuilder, IoArgoprojWorkflowV1alpha1Template,
    IoK8sApiCoreV1EmptyDirVolumeSource, TemplateBuilder, UserContainerBuilder, VolumeBuilder,
    VolumeMountBuilder,
};

pub(crate) fn singer_template(
    node: &Singer,
    plugin: &dyn Plugin,
) -> Result<IoArgoprojWorkflowV1alpha1Template, Error> {
    let template =
        TemplateBuilder::default()
            .name(Some(serde_json::from_value::<String>(
                node.target["image"].clone(),
            )?))
            .container(Some(
                ContainerBuilder::default()
                    .image(serde_json::from_value::<String>(
                        node.target["image"].clone(),
                    )?)
                    .volume_mounts(vec![VolumeMountBuilder::default()
                        .name("config".to_string())
                        .mount_path("/tmp/config".to_string())
                        .build()?])
                    .build()?,
            ))
            .init_containers(plugin.init_containters()?.unwrap_or(
                vec![UserContainerBuilder::default()
                    .name("envsubst".to_string())
                    .image(Some("dibi/envsubst".to_string()))
                    .volume_mounts(vec![
                        VolumeMountBuilder::default()
                            .name("config_template".to_string())
                            .mount_path("/workdir".to_string())
                            .build()?,
                        VolumeMountBuilder::default()
                            .name("config".to_string())
                            .mount_path("/processed".to_string())
                            .build()?,
                    ])
                    .build()?],
            ))
            .volumes(plugin.volumes()?.unwrap_or(vec![
            VolumeBuilder::default()
                .name("config".to_string())
                .empty_dir(Some(IoK8sApiCoreV1EmptyDirVolumeSource::default()))
                .build()?,
            VolumeBuilder::default()
                .name("config_template".to_string())
                .config_map(Some(ConfigMapVolumeSourceBuilder::default().build()?))
                .build()?,
        ]))
            .build()
            .unwrap();
    Ok(template)
}

pub(crate) fn iceberg_template(
    plugin: &dyn Plugin,
) -> Result<IoArgoprojWorkflowV1alpha1Template, Error> {
    let template = TemplateBuilder::default()
        .name(Some("iceberg".to_string()))
        .container(Some(
            ContainerBuilder::default()
                .image("ghcr.io/dashbook/refresh".to_string())
                .volume_mounts(vec![VolumeMountBuilder::default()
                    .name("config".to_string())
                    .mount_path("/tmp/config".to_string())
                    .build()?])
                .build()?,
        ))
        .init_containers(plugin.init_containters()?.unwrap_or(vec![
            UserContainerBuilder::default()
                .name("envsubst".to_string())
                .image(Some("dibi/envsubst".to_string()))
                .volume_mounts(vec![
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
        .volumes(plugin.volumes()?.unwrap_or(vec![
            VolumeBuilder::default()
                .name("config".to_string())
                .empty_dir(Some(IoK8sApiCoreV1EmptyDirVolumeSource::default()))
                .build()?,
            VolumeBuilder::default()
                .name("config_template".to_string())
                .config_map(Some(ConfigMapVolumeSourceBuilder::default().build()?))
                .build()?,
        ]))
        .build()
        .unwrap();
    Ok(template)
}
