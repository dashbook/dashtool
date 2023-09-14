use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FuturesChannel(#[from] futures::channel::mpsc::SendError),
    #[error(transparent)]
    ObjectStore(#[from] iceberg_rust::object_store::Error),
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
    #[error("No {0} token revieved.")]
    NoToken(String),
    #[error("{0}")]
    Text(String),
}
