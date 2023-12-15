use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    SerdeJSON(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeYAML(#[from] serde_yaml::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FuturesChannel(#[from] futures::channel::mpsc::SendError),
    #[error(transparent)]
    ObjectStore(#[from] object_store::Error),
    #[error(transparent)]
    Iceberg(#[from] iceberg_rust::error::Error),
    #[error(transparent)]
    IcebergSpec(#[from] iceberg_rust_spec::error::Error),
    #[error(transparent)]
    IcebergSqlCatalog(#[from] iceberg_catalog_sql::error::Error),
    #[error(transparent)]
    Git(#[from] git2::Error),
    #[error(transparent)]
    OIDCDiscovery(
        #[from] openidconnect::DiscoveryError<openidconnect::reqwest::Error<reqwest::Error>>,
    ),
    #[error(transparent)]
    OIDCConfiguration(#[from] openidconnect::ConfigurationError),
    #[error(transparent)]
    OIDCReqestToken(
        #[from]
        openidconnect::RequestTokenError<
            openidconnect::reqwest::Error<reqwest::Error>,
            openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    OIDCReqestTokenDevice(
        #[from]
        openidconnect::RequestTokenError<
            openidconnect::reqwest::Error<reqwest::Error>,
            openidconnect::StandardErrorResponse<openidconnect::DeviceCodeErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error(transparent)]
    SQLParser(#[from] sqlparser::parser::ParserError),
    #[error(transparent)]
    DashbookCatalog(#[from] dashbook_catalog::error::Error),
    #[error(transparent)]
    Datafusion(#[from] datafusion_common::DataFusionError),
    #[error(transparent)]
    ArgoDagTask(#[from] argo_workflow::schema::DagTaskBuilderError),
    #[error(transparent)]
    ArgoTemplate(#[from] argo_workflow::schema::TemplateBuilderError),
    #[error(transparent)]
    ArgoDagTemplate(#[from] argo_workflow::schema::DagTemplateBuilderError),
    #[error(transparent)]
    ArgoVolumeMount(#[from] argo_workflow::schema::VolumeMountBuilderError),
    #[error(transparent)]
    ArgoVolume(#[from] argo_workflow::schema::VolumeBuilderError),
    #[error(transparent)]
    ArgoSecretVolumeSource(#[from] argo_workflow::schema::SecretVolumeSourceBuilderError),
    #[error(transparent)]
    ArgoConfigMapVolumeSource(#[from] argo_workflow::schema::ConfigMapVolumeSourceBuilderError),
    #[error(transparent)]
    ArgoContainer(#[from] argo_workflow::schema::ContainerBuilderError),
    #[error(transparent)]
    ArgoUserContainer(#[from] argo_workflow::schema::UserContainerBuilderError),
    #[error(transparent)]
    ArgoEnvVar(#[from] argo_workflow::schema::EnvVarBuilderError),
    #[error(transparent)]
    ArgoInputs(#[from] argo_workflow::schema::InputsBuilderError),
    #[error(transparent)]
    ArgoParameter(#[from] argo_workflow::schema::ParameterBuilderError),
    #[error(transparent)]
    ArgoArguments(#[from] argo_workflow::schema::ArgumentsBuilderError),
    #[error(transparent)]
    ArgoEnvVarSource(#[from] argo_workflow::schema::EnvVarSourceBuilderError),
    #[error(transparent)]
    ArgoSecretKeySelector(#[from] argo_workflow::schema::SecretKeySelectorBuilderError),
    #[error(transparent)]
    ArgoCronWorkflowSpec(#[from] argo_workflow::schema::CronWorkflowSpecBuilderError),
    #[error(transparent)]
    ArgoCronWorkflow(#[from] argo_workflow::schema::CronWorkflowBuilderError),
    #[error(transparent)]
    ArgoWorkflow(#[from] argo_workflow::schema::WorkflowSpecBuilderError),
    #[error("No {0} token revieved.")]
    NoToken(String),
    #[error("{0}")]
    Text(String),
}
