use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(
    from = "Option<ObjectStoreConfigSerde>",
    into = "Option<ObjectStoreConfigSerde>"
)]
pub enum ObjectStoreConfig {
    S3(S3Config),
    Memory,
}

/// Config for the s3 object-store. The secret_access_key is read from the environment variable
/// AWS_SECRET_ACCESS_KEY
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct S3Config {
    pub aws_access_key_id: String,
    pub aws_region: String,
    pub aws_secret_access_key: Option<String>,
    pub aws_endpoint: Option<String>,
    pub aws_allow_http: Option<String>,
}

impl From<Option<ObjectStoreConfigSerde>> for ObjectStoreConfig {
    fn from(value: Option<ObjectStoreConfigSerde>) -> Self {
        match value {
            None => ObjectStoreConfig::Memory,
            Some(value) => match value {
                ObjectStoreConfigSerde::S3(value) => ObjectStoreConfig::S3(value),
            },
        }
    }
}

impl From<ObjectStoreConfig> for Option<ObjectStoreConfigSerde> {
    fn from(value: ObjectStoreConfig) -> Self {
        match value {
            ObjectStoreConfig::Memory => None,
            ObjectStoreConfig::S3(value) => Some(ObjectStoreConfigSerde::S3(value)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectStoreConfigSerde {
    S3(S3Config),
}
