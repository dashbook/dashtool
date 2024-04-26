use crate::{dag::Singer, error::Error, plugins::Plugin};

use argo_workflow::schema::{
    ConfigMapVolumeSourceBuilder, ContainerBuilder, InputsBuilder,
    IoArgoprojWorkflowV1alpha1Template, IoK8sApiCoreV1EmptyDirVolumeSource, ParameterBuilder,
    TemplateBuilder, UserContainerBuilder, VolumeBuilder, VolumeMountBuilder,
};

pub(crate) fn singer_template(
    node: &Singer,
    plugin: &dyn Plugin,
) -> Result<IoArgoprojWorkflowV1alpha1Template, Error> {
    let template =
        TemplateBuilder::default()
            .name(Some(node.image.clone()))
            .inputs(Some(
                InputsBuilder::default()
                    .parameters(vec![{
                        let mut builder: ParameterBuilder = Default::default();
                        builder.name("identifier".to_string()).build()?
                    }])
                    .build()?,
            ))
            .container(Some(
                ContainerBuilder::default()
                    .image(node.image.clone())
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
                            .name("config-template".to_string())
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
                        .name("config-template".to_string())
                        .config_map(Some(
                            ConfigMapVolumeSourceBuilder::default()
                                .name(Some(
                                    "{{inputs.parameters.identifier}}-config-template".to_string(),
                                ))
                                .build()?,
                        ))
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
        .name(Some("refresh".to_string()))
        .inputs(Some(
            InputsBuilder::default()
                .parameters(vec![{
                    let mut builder: ParameterBuilder = Default::default();
                    builder.name("identifier".to_string()).build()?
                }])
                .build()?,
        ))
        .container(Some(
            ContainerBuilder::default()
                .image(plugin.refresh_image().to_owned())
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
                        .name("config-template".to_string())
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
                .name("config-template".to_string())
                .config_map(Some(
                    ConfigMapVolumeSourceBuilder::default()
                        .name(Some(
                            "{{inputs.parameters.identifier}}-config-template".to_string(),
                        ))
                        .build()?,
                ))
                .build()?,
        ]))
        .build()
        .unwrap();
    Ok(template)
}
