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
    IcebergSqlCatalog(#[from] iceberg_sql_catalog::error::Error),
    #[error(transparent)]
    GitDiscover(#[from] gix::discover::Error),
    #[error(transparent)]
    GitReference(#[from] gix::reference::find::Error),
    #[error(transparent)]
    GitReferenceExisting(#[from] gix::reference::find::existing::Error),
    #[error(transparent)]
    GitObject(#[from] gix::object::find::Error),
    #[error(transparent)]
    GitChanges(#[from] gix::diff::tree::changes::Error),
    #[error(transparent)]
    GitObjectDecode(#[from] gix::objs::decode::Error),
    #[error(transparent)]
    GitObjectTryInto(#[from] gix::object::try_into::Error),
    #[error(transparent)]
    GitRecorder(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
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
    Envsubst(#[from] shellexpand::LookupError<std::env::VarError>),
    #[error(transparent)]
    StrParse(#[from] std::str::ParseBoolError),
    #[error(transparent)]
    SQLParser(#[from] sqlparser::parser::ParserError),
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
    ArgoObjectMeta(#[from] argo_workflow::schema::ObjectMetaBuilderError),
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
