#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventsourceCreateEventSourceRequest {
    #[serde(
        rename = "eventSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source: Option<IoArgoprojEventsV1alpha1EventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&EventsourceCreateEventSourceRequest> for EventsourceCreateEventSourceRequest {
    fn from(value: &EventsourceCreateEventSourceRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventsourceEventSourceDeletedResponse(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for EventsourceEventSourceDeletedResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<EventsourceEventSourceDeletedResponse> for serde_json::Map<String, serde_json::Value> {
    fn from(value: EventsourceEventSourceDeletedResponse) -> Self {
        value.0
    }
}
impl From<&EventsourceEventSourceDeletedResponse> for EventsourceEventSourceDeletedResponse {
    fn from(value: &EventsourceEventSourceDeletedResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for EventsourceEventSourceDeletedResponse {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventsourceEventSourceWatchEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<IoArgoprojEventsV1alpha1EventSource>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&EventsourceEventSourceWatchEvent> for EventsourceEventSourceWatchEvent {
    fn from(value: &EventsourceEventSourceWatchEvent) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventsourceLogEntry {
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(
        rename = "eventSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source_name: Option<String>,
    #[serde(
        rename = "eventSourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
}
impl From<&EventsourceLogEntry> for EventsourceLogEntry {
    fn from(value: &EventsourceLogEntry) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventsourceUpdateEventSourceRequest {
    #[serde(
        rename = "eventSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source: Option<IoArgoprojEventsV1alpha1EventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&EventsourceUpdateEventSourceRequest> for EventsourceUpdateEventSourceRequest {
    fn from(value: &EventsourceUpdateEventSourceRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GoogleProtobufAny {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&GoogleProtobufAny> for GoogleProtobufAny {
    fn from(value: &GoogleProtobufAny) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrpcGatewayRuntimeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<GoogleProtobufAny>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl From<&GrpcGatewayRuntimeError> for GrpcGatewayRuntimeError {
    fn from(value: &GrpcGatewayRuntimeError) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrpcGatewayRuntimeStreamError {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<GoogleProtobufAny>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl From<&GrpcGatewayRuntimeStreamError> for GrpcGatewayRuntimeStreamError {
    fn from(value: &GrpcGatewayRuntimeStreamError) -> Self {
        value.clone()
    }
}
#[doc = "Amount represent a numeric amount."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Amount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1Amount> for IoArgoprojEventsV1alpha1Amount {
    fn from(value: &IoArgoprojEventsV1alpha1Amount) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AmqpConsumeConfig {
    #[serde(rename = "autoAck", default, skip_serializing_if = "Option::is_none")]
    pub auto_ack: Option<bool>,
    #[serde(
        rename = "consumerTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    #[serde(rename = "noLocal", default, skip_serializing_if = "Option::is_none")]
    pub no_local: Option<bool>,
    #[serde(rename = "noWait", default, skip_serializing_if = "Option::is_none")]
    pub no_wait: Option<bool>,
}
impl From<&IoArgoprojEventsV1alpha1AmqpConsumeConfig>
    for IoArgoprojEventsV1alpha1AmqpConsumeConfig
{
    fn from(value: &IoArgoprojEventsV1alpha1AmqpConsumeConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AmqpEventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<IoArgoprojEventsV1alpha1BasicAuth>,
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consume: Option<IoArgoprojEventsV1alpha1AmqpConsumeConfig>,
    #[serde(
        rename = "exchangeDeclare",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_declare: Option<IoArgoprojEventsV1alpha1AmqpExchangeDeclareConfig>,
    #[serde(
        rename = "exchangeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_name: Option<String>,
    #[serde(
        rename = "exchangeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(rename = "queueBind", default, skip_serializing_if = "Option::is_none")]
    pub queue_bind: Option<IoArgoprojEventsV1alpha1AmqpQueueBindConfig>,
    #[serde(
        rename = "queueDeclare",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_declare: Option<IoArgoprojEventsV1alpha1AmqpQueueDeclareConfig>,
    #[serde(
        rename = "routingKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub routing_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "urlSecret", default, skip_serializing_if = "Option::is_none")]
    pub url_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1AmqpEventSource> for IoArgoprojEventsV1alpha1AmqpEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1AmqpEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AmqpExchangeDeclareConfig {
    #[serde(
        rename = "autoDelete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "noWait", default, skip_serializing_if = "Option::is_none")]
    pub no_wait: Option<bool>,
}
impl From<&IoArgoprojEventsV1alpha1AmqpExchangeDeclareConfig>
    for IoArgoprojEventsV1alpha1AmqpExchangeDeclareConfig
{
    fn from(value: &IoArgoprojEventsV1alpha1AmqpExchangeDeclareConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AmqpQueueBindConfig {
    #[serde(rename = "noWait", default, skip_serializing_if = "Option::is_none")]
    pub no_wait: Option<bool>,
}
impl From<&IoArgoprojEventsV1alpha1AmqpQueueBindConfig>
    for IoArgoprojEventsV1alpha1AmqpQueueBindConfig
{
    fn from(value: &IoArgoprojEventsV1alpha1AmqpQueueBindConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AmqpQueueDeclareConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(
        rename = "autoDelete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "noWait", default, skip_serializing_if = "Option::is_none")]
    pub no_wait: Option<bool>,
}
impl From<&IoArgoprojEventsV1alpha1AmqpQueueDeclareConfig>
    for IoArgoprojEventsV1alpha1AmqpQueueDeclareConfig
{
    fn from(value: &IoArgoprojEventsV1alpha1AmqpQueueDeclareConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ArgoWorkflowTrigger {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<IoArgoprojEventsV1alpha1ArtifactLocation>,
}
impl From<&IoArgoprojEventsV1alpha1ArgoWorkflowTrigger>
    for IoArgoprojEventsV1alpha1ArgoWorkflowTrigger
{
    fn from(value: &IoArgoprojEventsV1alpha1ArgoWorkflowTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ArtifactLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configmap: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<IoArgoprojEventsV1alpha1FileArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<IoArgoprojEventsV1alpha1GitArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<IoArgoprojEventsV1alpha1Resource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<IoArgoprojEventsV1alpha1S3Artifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<IoArgoprojEventsV1alpha1UrlArtifact>,
}
impl From<&IoArgoprojEventsV1alpha1ArtifactLocation> for IoArgoprojEventsV1alpha1ArtifactLocation {
    fn from(value: &IoArgoprojEventsV1alpha1ArtifactLocation) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AwsLambdaTrigger {
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "FunctionName refers to the name of the function to invoke."]
    #[serde(
        rename = "functionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_name: Option<String>,
    #[doc = "Choose from the following options.\n\n   * RequestResponse (default) - Invoke the function synchronously. Keep\n   the connection open until the function returns a response or times out.\n   The API response includes the function response and additional data.\n\n   * Event - Invoke the function asynchronously. Send events that fail multiple\n   times to the function's dead-letter queue (if it's configured). The API\n   response only includes a status code.\n\n   * DryRun - Validate parameter values and verify that the user or role\n   has permission to invoke the function.\n+optional"]
    #[serde(
        rename = "invocationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invocation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Payload is the list of key-value extracted from an event payload to construct the request payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1AwsLambdaTrigger> for IoArgoprojEventsV1alpha1AwsLambdaTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1AwsLambdaTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AzureEventHubsTrigger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Payload is the list of key-value extracted from an event payload to construct the request payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(
        rename = "sharedAccessKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_access_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "sharedAccessKeyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_access_key_name: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1AzureEventHubsTrigger>
    for IoArgoprojEventsV1alpha1AzureEventHubsTrigger
{
    fn from(value: &IoArgoprojEventsV1alpha1AzureEventHubsTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1AzureEventsHubEventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "hubName", default, skip_serializing_if = "Option::is_none")]
    pub hub_name: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(
        rename = "sharedAccessKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_access_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "sharedAccessKeyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_access_key_name: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1AzureEventsHubEventSource>
    for IoArgoprojEventsV1alpha1AzureEventsHubEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1AzureEventsHubEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Backoff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<IoArgoprojEventsV1alpha1Int64OrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<IoArgoprojEventsV1alpha1Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter: Option<IoArgoprojEventsV1alpha1Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
}
impl From<&IoArgoprojEventsV1alpha1Backoff> for IoArgoprojEventsV1alpha1Backoff {
    fn from(value: &IoArgoprojEventsV1alpha1Backoff) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BasicAuth {
    #[doc = "Password refers to the Kubernetes secret that holds the password required for basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Username refers to the Kubernetes secret that holds the username required for basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1BasicAuth> for IoArgoprojEventsV1alpha1BasicAuth {
    fn from(value: &IoArgoprojEventsV1alpha1BasicAuth) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BitbucketAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic: Option<IoArgoprojEventsV1alpha1BitbucketBasicAuth>,
    #[serde(
        rename = "oauthToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub oauth_token: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1BitbucketAuth> for IoArgoprojEventsV1alpha1BitbucketAuth {
    fn from(value: &IoArgoprojEventsV1alpha1BitbucketAuth) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BitbucketBasicAuth {
    #[doc = "Password refers to the K8s secret that holds the password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Username refers to the K8s secret that holds the username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1BitbucketBasicAuth>
    for IoArgoprojEventsV1alpha1BitbucketBasicAuth
{
    fn from(value: &IoArgoprojEventsV1alpha1BitbucketBasicAuth) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BitbucketEventSource {
    #[doc = "Auth information required to connect to Bitbucket."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<IoArgoprojEventsV1alpha1BitbucketAuth>,
    #[serde(
        rename = "deleteHookOnFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_hook_on_finish: Option<bool>,
    #[doc = "Events this webhook is subscribed to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(
        rename = "projectKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_key: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<IoArgoprojEventsV1alpha1BitbucketRepository>,
    #[serde(
        rename = "repositorySlug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1BitbucketEventSource>
    for IoArgoprojEventsV1alpha1BitbucketEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1BitbucketEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BitbucketRepository {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(
        rename = "repositorySlug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_slug: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1BitbucketRepository>
    for IoArgoprojEventsV1alpha1BitbucketRepository
{
    fn from(value: &IoArgoprojEventsV1alpha1BitbucketRepository) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BitbucketServerEventSource {
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "bitbucketserverBaseURL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bitbucketserver_base_url: Option<String>,
    #[serde(
        rename = "deleteHookOnFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_hook_on_finish: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(
        rename = "projectKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_key: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<IoArgoprojEventsV1alpha1BitbucketServerRepository>,
    #[serde(
        rename = "repositorySlug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
    #[serde(
        rename = "webhookSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1BitbucketServerEventSource>
    for IoArgoprojEventsV1alpha1BitbucketServerEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1BitbucketServerEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1BitbucketServerRepository {
    #[serde(
        rename = "projectKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_key: Option<String>,
    #[serde(
        rename = "repositorySlug",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_slug: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1BitbucketServerRepository>
    for IoArgoprojEventsV1alpha1BitbucketServerRepository
{
    fn from(value: &IoArgoprojEventsV1alpha1BitbucketServerRepository) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1CalendarEventSource {
    #[doc = "ExclusionDates defines the list of DATE-TIME exceptions for recurring events."]
    #[serde(
        rename = "exclusionDates",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exclusion_dates: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistence: Option<IoArgoprojEventsV1alpha1EventPersistence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1CalendarEventSource>
    for IoArgoprojEventsV1alpha1CalendarEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1CalendarEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1CatchupConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "maxDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_duration: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1CatchupConfiguration>
    for IoArgoprojEventsV1alpha1CatchupConfiguration
{
    fn from(value: &IoArgoprojEventsV1alpha1CatchupConfiguration) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Condition {
    #[serde(
        rename = "lastTransitionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transition_time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1Condition> for IoArgoprojEventsV1alpha1Condition {
    fn from(value: &IoArgoprojEventsV1alpha1Condition) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ConditionsResetByTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1ConditionsResetByTime>
    for IoArgoprojEventsV1alpha1ConditionsResetByTime
{
    fn from(value: &IoArgoprojEventsV1alpha1ConditionsResetByTime) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ConditionsResetCriteria {
    #[serde(rename = "byTime", default, skip_serializing_if = "Option::is_none")]
    pub by_time: Option<IoArgoprojEventsV1alpha1ConditionsResetByTime>,
}
impl From<&IoArgoprojEventsV1alpha1ConditionsResetCriteria>
    for IoArgoprojEventsV1alpha1ConditionsResetCriteria
{
    fn from(value: &IoArgoprojEventsV1alpha1ConditionsResetCriteria) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ConfigMapPersistence {
    #[serde(
        rename = "createIfNotExist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_if_not_exist: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1ConfigMapPersistence>
    for IoArgoprojEventsV1alpha1ConfigMapPersistence
{
    fn from(value: &IoArgoprojEventsV1alpha1ConfigMapPersistence) -> Self {
        value.clone()
    }
}
#[doc = "CustomTrigger refers to the specification of the custom trigger."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1CustomTrigger {
    #[doc = "CertSecret refers to the secret that contains cert for secure connection between sensor and custom trigger gRPC server."]
    #[serde(
        rename = "certSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cert_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Parameters is the list of parameters that is applied to resolved custom trigger trigger object."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Payload is the list of key-value extracted from an event payload to construct the request payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[doc = "ServerNameOverride for the secure connection between sensor and custom trigger gRPC server."]
    #[serde(
        rename = "serverNameOverride",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name_override: Option<String>,
    #[serde(rename = "serverURL", default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    #[doc = "Spec is the custom trigger resource specification that custom trigger gRPC server knows how to interpret."]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub spec: std::collections::HashMap<String, String>,
}
impl From<&IoArgoprojEventsV1alpha1CustomTrigger> for IoArgoprojEventsV1alpha1CustomTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1CustomTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1DataFilter {
    #[doc = "Comparator compares the event data with a user given value.\nCan be \">=\", \">\", \"=\", \"!=\", \"<\", or \"<=\".\nIs optional, and if left blank treated as equality \"=\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[doc = "Path is the JSONPath of the event's (JSON decoded) data key\nPath is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'.\nTo access an array value use the index as the key. The dot and wildcard characters can be escaped with '\\\\'.\nSee https://github.com/tidwall/gjson#path-syntax for more information on how to use this."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl From<&IoArgoprojEventsV1alpha1DataFilter> for IoArgoprojEventsV1alpha1DataFilter {
    fn from(value: &IoArgoprojEventsV1alpha1DataFilter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EmitterEventSource {
    #[doc = "Broker URI to connect to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broker: Option<String>,
    #[serde(
        rename = "channelKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_key: Option<String>,
    #[serde(
        rename = "channelName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1EmitterEventSource>
    for IoArgoprojEventsV1alpha1EmitterEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1EmitterEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventContext {
    #[doc = "DataContentType - A MIME (RFC2046) string describing the media type of `data`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacontenttype: Option<String>,
    #[doc = "ID of the event; must be non-empty and unique within the scope of the producer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Source - A URI describing the event producer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "SpecVersion - The version of the CloudEvents specification used by the io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specversion: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Time - A Timestamp when the event happened."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "Type - The type of the occurrence which has happened."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1EventContext> for IoArgoprojEventsV1alpha1EventContext {
    fn from(value: &IoArgoprojEventsV1alpha1EventContext) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventDependency {
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(
        rename = "eventSourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_source_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<IoArgoprojEventsV1alpha1EventDependencyFilter>,
    #[doc = "FiltersLogicalOperator defines how different filters are evaluated together.\nAvailable values: and (&&), or (||)\nIs optional and if left blank treated as and (&&)."]
    #[serde(
        rename = "filtersLogicalOperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filters_logical_operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<IoArgoprojEventsV1alpha1EventDependencyTransformer>,
}
impl From<&IoArgoprojEventsV1alpha1EventDependency> for IoArgoprojEventsV1alpha1EventDependency {
    fn from(value: &IoArgoprojEventsV1alpha1EventDependency) -> Self {
        value.clone()
    }
}
#[doc = "EventDependencyFilter defines filters and constraints for a io.argoproj.workflow.v1alpha1."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventDependencyFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<IoArgoprojEventsV1alpha1EventContext>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<IoArgoprojEventsV1alpha1DataFilter>,
    #[doc = "DataLogicalOperator defines how multiple Data filters (if defined) are evaluated together.\nAvailable values: and (&&), or (||)\nIs optional and if left blank treated as and (&&)."]
    #[serde(
        rename = "dataLogicalOperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_logical_operator: Option<String>,
    #[doc = "ExprLogicalOperator defines how multiple Exprs filters (if defined) are evaluated together.\nAvailable values: and (&&), or (||)\nIs optional and if left blank treated as and (&&)."]
    #[serde(
        rename = "exprLogicalOperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expr_logical_operator: Option<String>,
    #[doc = "Exprs contains the list of expressions evaluated against the event payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exprs: Vec<IoArgoprojEventsV1alpha1ExprFilter>,
    #[doc = "Script refers to a Lua script evaluated to determine the validity of an io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<IoArgoprojEventsV1alpha1TimeFilter>,
}
impl From<&IoArgoprojEventsV1alpha1EventDependencyFilter>
    for IoArgoprojEventsV1alpha1EventDependencyFilter
{
    fn from(value: &IoArgoprojEventsV1alpha1EventDependencyFilter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventDependencyTransformer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jq: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1EventDependencyTransformer>
    for IoArgoprojEventsV1alpha1EventDependencyTransformer
{
    fn from(value: &IoArgoprojEventsV1alpha1EventDependencyTransformer) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventPersistence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catchup: Option<IoArgoprojEventsV1alpha1CatchupConfiguration>,
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<IoArgoprojEventsV1alpha1ConfigMapPersistence>,
}
impl From<&IoArgoprojEventsV1alpha1EventPersistence> for IoArgoprojEventsV1alpha1EventPersistence {
    fn from(value: &IoArgoprojEventsV1alpha1EventPersistence) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IoArgoprojEventsV1alpha1EventSourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojEventsV1alpha1EventSourceStatus>,
}
impl From<&IoArgoprojEventsV1alpha1EventSource> for IoArgoprojEventsV1alpha1EventSource {
    fn from(value: &IoArgoprojEventsV1alpha1EventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventSourceFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1EventSourceFilter>
    for IoArgoprojEventsV1alpha1EventSourceFilter
{
    fn from(value: &IoArgoprojEventsV1alpha1EventSourceFilter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventSourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoArgoprojEventsV1alpha1EventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ListMeta>,
}
impl From<&IoArgoprojEventsV1alpha1EventSourceList> for IoArgoprojEventsV1alpha1EventSourceList {
    fn from(value: &IoArgoprojEventsV1alpha1EventSourceList) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventSourceSpec {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub amqp: std::collections::HashMap<String, IoArgoprojEventsV1alpha1AmqpEventSource>,
    #[serde(
        rename = "azureEventsHub",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub azure_events_hub:
        std::collections::HashMap<String, IoArgoprojEventsV1alpha1AzureEventsHubEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub bitbucket: std::collections::HashMap<String, IoArgoprojEventsV1alpha1BitbucketEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub bitbucketserver:
        std::collections::HashMap<String, IoArgoprojEventsV1alpha1BitbucketServerEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub calendar: std::collections::HashMap<String, IoArgoprojEventsV1alpha1CalendarEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub emitter: std::collections::HashMap<String, IoArgoprojEventsV1alpha1EmitterEventSource>,
    #[serde(
        rename = "eventBusName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_bus_name: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub file: std::collections::HashMap<String, IoArgoprojEventsV1alpha1FileEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub generic: std::collections::HashMap<String, IoArgoprojEventsV1alpha1GenericEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub github: std::collections::HashMap<String, IoArgoprojEventsV1alpha1GithubEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub gitlab: std::collections::HashMap<String, IoArgoprojEventsV1alpha1GitlabEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub hdfs: std::collections::HashMap<String, IoArgoprojEventsV1alpha1HdfsEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub kafka: std::collections::HashMap<String, IoArgoprojEventsV1alpha1KafkaEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub minio: std::collections::HashMap<String, IoArgoprojEventsV1alpha1S3Artifact>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mqtt: std::collections::HashMap<String, IoArgoprojEventsV1alpha1MqttEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub nats: std::collections::HashMap<String, IoArgoprojEventsV1alpha1NatsEventsSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub nsq: std::collections::HashMap<String, IoArgoprojEventsV1alpha1NsqEventSource>,
    #[serde(
        rename = "pubSub",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub pub_sub: std::collections::HashMap<String, IoArgoprojEventsV1alpha1PubSubEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub pulsar: std::collections::HashMap<String, IoArgoprojEventsV1alpha1PulsarEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub redis: std::collections::HashMap<String, IoArgoprojEventsV1alpha1RedisEventSource>,
    #[serde(
        rename = "redisStream",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub redis_stream:
        std::collections::HashMap<String, IoArgoprojEventsV1alpha1RedisStreamEventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub resource: std::collections::HashMap<String, IoArgoprojEventsV1alpha1ResourceEventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<IoArgoprojEventsV1alpha1Service>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub slack: std::collections::HashMap<String, IoArgoprojEventsV1alpha1SlackEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub sns: std::collections::HashMap<String, IoArgoprojEventsV1alpha1SnsEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub sqs: std::collections::HashMap<String, IoArgoprojEventsV1alpha1SqsEventSource>,
    #[serde(
        rename = "storageGrid",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub storage_grid:
        std::collections::HashMap<String, IoArgoprojEventsV1alpha1StorageGridEventSource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub stripe: std::collections::HashMap<String, IoArgoprojEventsV1alpha1StripeEventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojEventsV1alpha1Template>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub webhook: std::collections::HashMap<String, IoArgoprojEventsV1alpha1WebhookEventSource>,
}
impl From<&IoArgoprojEventsV1alpha1EventSourceSpec> for IoArgoprojEventsV1alpha1EventSourceSpec {
    fn from(value: &IoArgoprojEventsV1alpha1EventSourceSpec) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1EventSourceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojEventsV1alpha1Status>,
}
impl From<&IoArgoprojEventsV1alpha1EventSourceStatus>
    for IoArgoprojEventsV1alpha1EventSourceStatus
{
    fn from(value: &IoArgoprojEventsV1alpha1EventSourceStatus) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ExprFilter {
    #[doc = "Expr refers to the expression that determines the outcome of the filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[doc = "Fields refers to set of keys that refer to the paths within event payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<IoArgoprojEventsV1alpha1PayloadField>,
}
impl From<&IoArgoprojEventsV1alpha1ExprFilter> for IoArgoprojEventsV1alpha1ExprFilter {
    fn from(value: &IoArgoprojEventsV1alpha1ExprFilter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1FileArtifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1FileArtifact> for IoArgoprojEventsV1alpha1FileArtifact {
    fn from(value: &IoArgoprojEventsV1alpha1FileArtifact) -> Self {
        value.clone()
    }
}
#[doc = "FileEventSource describes an event-source for file related events."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1FileEventSource {
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polling: Option<bool>,
    #[serde(
        rename = "watchPathConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub watch_path_config: Option<IoArgoprojEventsV1alpha1WatchPathConfig>,
}
impl From<&IoArgoprojEventsV1alpha1FileEventSource> for IoArgoprojEventsV1alpha1FileEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1FileEventSource) -> Self {
        value.clone()
    }
}
#[doc = "GenericEventSource refers to a generic event source. It can be used to implement a custom event source."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GenericEventSource {
    #[serde(
        rename = "authSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[doc = "Insecure determines the type of connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "URL of the gRPC server that implements the event source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1GenericEventSource>
    for IoArgoprojEventsV1alpha1GenericEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1GenericEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GitArtifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "Directory to clone the repository. We clone complete directory because GitArtifact is not limited to any specific Git service providers.\nHence we don't use any specific git provider client."]
    #[serde(
        rename = "cloneDirectory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clone_directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creds: Option<IoArgoprojEventsV1alpha1GitCreds>,
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(
        rename = "insecureIgnoreHostKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insecure_ignore_host_key: Option<bool>,
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<IoArgoprojEventsV1alpha1GitRemoteConfig>,
    #[serde(
        rename = "sshKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1GitArtifact> for IoArgoprojEventsV1alpha1GitArtifact {
    fn from(value: &IoArgoprojEventsV1alpha1GitArtifact) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GitCreds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1GitCreds> for IoArgoprojEventsV1alpha1GitCreds {
    fn from(value: &IoArgoprojEventsV1alpha1GitCreds) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GitRemoteConfig {
    #[doc = "Name of the remote to fetch from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "URLs the URLs of a remote repository. It must be non-empty. Fetch will\nalways use the first URL, while push will use all of them."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
}
impl From<&IoArgoprojEventsV1alpha1GitRemoteConfig> for IoArgoprojEventsV1alpha1GitRemoteConfig {
    fn from(value: &IoArgoprojEventsV1alpha1GitRemoteConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GithubAppCreds {
    #[serde(rename = "appID", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(
        rename = "installationID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub installation_id: Option<String>,
    #[serde(
        rename = "privateKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_key: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1GithubAppCreds> for IoArgoprojEventsV1alpha1GithubAppCreds {
    fn from(value: &IoArgoprojEventsV1alpha1GithubAppCreds) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GithubEventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "apiToken", default, skip_serializing_if = "Option::is_none")]
    pub api_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(
        rename = "deleteHookOnFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_hook_on_finish: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "githubApp", default, skip_serializing_if = "Option::is_none")]
    pub github_app: Option<IoArgoprojEventsV1alpha1GithubAppCreds>,
    #[serde(
        rename = "githubBaseURL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub github_base_url: Option<String>,
    #[serde(
        rename = "githubUploadURL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub github_upload_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Organizations holds the names of organizations (used for organization level webhooks). Not required if Repositories is set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Repositories holds the information of repositories, which uses repo owner as the key,\nand list of repo names as the value. Not required if Organizations is set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<IoArgoprojEventsV1alpha1OwnedRepositories>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
    #[serde(
        rename = "webhookSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1GithubEventSource>
    for IoArgoprojEventsV1alpha1GithubEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1GithubEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1GitlabEventSource {
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "deleteHookOnFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_hook_on_finish: Option<bool>,
    #[serde(
        rename = "enableSSLVerification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_ssl_verification: Option<bool>,
    #[doc = "Events are gitlab event to listen to.\nRefer https://github.com/xanzy/go-gitlab/blob/bf34eca5d13a9f4c3f501d8a97b8ac226d55e4d9/projects.go#L794."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(
        rename = "gitlabBaseURL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gitlab_base_url: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(rename = "projectID", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<String>,
    #[serde(
        rename = "secretToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1GitlabEventSource>
    for IoArgoprojEventsV1alpha1GitlabEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1GitlabEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1HdfsEventSource {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<String>,
    #[serde(
        rename = "checkInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[doc = "HDFSUser is the user to access HDFS file system.\nIt is ignored if either ccache or keytab is used."]
    #[serde(rename = "hdfsUser", default, skip_serializing_if = "Option::is_none")]
    pub hdfs_user: Option<String>,
    #[doc = "KrbCCacheSecret is the secret selector for Kerberos ccache\nEither ccache or keytab can be set to use Kerberos."]
    #[serde(
        rename = "krbCCacheSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_c_cache_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "KrbConfig is the configmap selector for Kerberos config as string\nIt must be set if either ccache or keytab is used."]
    #[serde(
        rename = "krbConfigConfigMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_config_config_map: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[doc = "KrbKeytabSecret is the secret selector for Kerberos keytab\nEither ccache or keytab can be set to use Kerberos."]
    #[serde(
        rename = "krbKeytabSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_keytab_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "KrbRealm is the Kerberos realm used with Kerberos keytab\nIt must be set if keytab is used."]
    #[serde(rename = "krbRealm", default, skip_serializing_if = "Option::is_none")]
    pub krb_realm: Option<String>,
    #[doc = "KrbServicePrincipalName is the principal name of Kerberos service\nIt must be set if either ccache or keytab is used."]
    #[serde(
        rename = "krbServicePrincipalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_service_principal_name: Option<String>,
    #[doc = "KrbUsername is the Kerberos username used with Kerberos keytab\nIt must be set if keytab is used."]
    #[serde(
        rename = "krbUsername",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_username: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(
        rename = "watchPathConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub watch_path_config: Option<IoArgoprojEventsV1alpha1WatchPathConfig>,
}
impl From<&IoArgoprojEventsV1alpha1HdfsEventSource> for IoArgoprojEventsV1alpha1HdfsEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1HdfsEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1HttpTrigger {
    #[serde(rename = "basicAuth", default, skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<IoArgoprojEventsV1alpha1BasicAuth>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub headers: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "Parameters is the list of key-value extracted from event's payload that are applied to\nthe HTTP trigger resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(
        rename = "secureHeaders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub secure_headers: Vec<IoArgoprojEventsV1alpha1SecureHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[doc = "URL refers to the URL to send HTTP request to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1HttpTrigger> for IoArgoprojEventsV1alpha1HttpTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1HttpTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Int64OrString {
    #[serde(rename = "int64Val", default, skip_serializing_if = "Option::is_none")]
    pub int64_val: Option<String>,
    #[serde(rename = "strVal", default, skip_serializing_if = "Option::is_none")]
    pub str_val: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1Int64OrString> for IoArgoprojEventsV1alpha1Int64OrString {
    fn from(value: &IoArgoprojEventsV1alpha1Int64OrString) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1K8sResourcePolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(
        rename = "errorOnBackoffTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_on_backoff_timeout: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,
}
impl From<&IoArgoprojEventsV1alpha1K8sResourcePolicy>
    for IoArgoprojEventsV1alpha1K8sResourcePolicy
{
    fn from(value: &IoArgoprojEventsV1alpha1K8sResourcePolicy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1KafkaConsumerGroup {
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest: Option<bool>,
    #[serde(
        rename = "rebalanceStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rebalance_strategy: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1KafkaConsumerGroup>
    for IoArgoprojEventsV1alpha1KafkaConsumerGroup
{
    fn from(value: &IoArgoprojEventsV1alpha1KafkaConsumerGroup) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1KafkaEventSource {
    #[doc = "Yaml format Sarama config for Kafka connection.\nIt follows the struct of sarama.Config. See https://github.com/Shopify/sarama/blob/main/config.go\ne.g.\n\nconsumer:\n  fetch:\n    min: 1\nnet:\n  MaxOpenRequests: 5\n\n+optional"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[doc = "Backoff holds parameters applied to connection."]
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(
        rename = "consumerGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_group: Option<IoArgoprojEventsV1alpha1KafkaConsumerGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(
        rename = "limitEventsPerSecond",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_events_per_second: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sasl: Option<IoArgoprojEventsV1alpha1SaslConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1KafkaEventSource> for IoArgoprojEventsV1alpha1KafkaEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1KafkaEventSource) -> Self {
        value.clone()
    }
}
#[doc = "KafkaTrigger refers to the specification of the Kafka trigger."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1KafkaTrigger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(
        rename = "flushFrequency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flush_frequency: Option<i64>,
    #[doc = "Parameters is the list of parameters that is applied to resolved Kafka trigger object."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Partition to write data to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i64>,
    #[doc = "The partitioning key for the messages put on the Kafka topic.\nDefaults to broker url.\n+optional."]
    #[serde(
        rename = "partitioningKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub partitioning_key: Option<String>,
    #[doc = "Payload is the list of key-value extracted from an event payload to construct the request payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "RequiredAcks used in producer to tell the broker how many replica acknowledgements\nDefaults to 1 (Only wait for the leader to ack).\n+optional."]
    #[serde(
        rename = "requiredAcks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_acks: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sasl: Option<IoArgoprojEventsV1alpha1SaslConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[doc = "URL of the Kafka broker, multiple URLs separated by comma."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1KafkaTrigger> for IoArgoprojEventsV1alpha1KafkaTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1KafkaTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1LogTrigger {
    #[serde(
        rename = "intervalSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_seconds: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1LogTrigger> for IoArgoprojEventsV1alpha1LogTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1LogTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Metadata {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub annotations: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,
}
impl From<&IoArgoprojEventsV1alpha1Metadata> for IoArgoprojEventsV1alpha1Metadata {
    fn from(value: &IoArgoprojEventsV1alpha1Metadata) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1MqttEventSource {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "ConnectionBackoff holds backoff applied to connection."]
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1MqttEventSource> for IoArgoprojEventsV1alpha1MqttEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1MqttEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1NatsAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic: Option<IoArgoprojEventsV1alpha1BasicAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nkey: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1NatsAuth> for IoArgoprojEventsV1alpha1NatsAuth {
    fn from(value: &IoArgoprojEventsV1alpha1NatsAuth) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1NatsEventsSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<IoArgoprojEventsV1alpha1NatsAuth>,
    #[doc = "ConnectionBackoff holds backoff applied to connection."]
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1NatsEventsSource> for IoArgoprojEventsV1alpha1NatsEventsSource {
    fn from(value: &IoArgoprojEventsV1alpha1NatsEventsSource) -> Self {
        value.clone()
    }
}
#[doc = "NATSTrigger refers to the specification of the NATS trigger."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1NatsTrigger {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Name of the subject to put message on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[doc = "URL of the NATS cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1NatsTrigger> for IoArgoprojEventsV1alpha1NatsTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1NatsTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1NsqEventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(
        rename = "hostAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_address: Option<String>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[doc = "Topic to subscribe to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1NsqEventSource> for IoArgoprojEventsV1alpha1NsqEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1NsqEventSource) -> Self {
        value.clone()
    }
}
#[doc = "OpenWhiskTrigger refers to the specification of the OpenWhisk trigger."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1OpenWhiskTrigger {
    #[doc = "Name of the action/function."]
    #[serde(
        rename = "actionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_name: Option<String>,
    #[serde(rename = "authToken", default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Host URL of the OpenWhisk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "Namespace for the action.\nDefaults to \"_\".\n+optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Payload is the list of key-value extracted from an event payload to construct the request payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1OpenWhiskTrigger> for IoArgoprojEventsV1alpha1OpenWhiskTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1OpenWhiskTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1OwnedRepositories {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1OwnedRepositories>
    for IoArgoprojEventsV1alpha1OwnedRepositories
{
    fn from(value: &IoArgoprojEventsV1alpha1OwnedRepositories) -> Self {
        value.clone()
    }
}
#[doc = "PayloadField binds a value at path within the event payload against a name."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1PayloadField {
    #[doc = "Name acts as key that holds the value at the path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Path is the JSONPath of the event's (JSON decoded) data key\nPath is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'.\nTo access an array value use the index as the key. The dot and wildcard characters can be escaped with '\\\\'.\nSee https://github.com/tidwall/gjson#path-syntax for more information on how to use this."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1PayloadField> for IoArgoprojEventsV1alpha1PayloadField {
    fn from(value: &IoArgoprojEventsV1alpha1PayloadField) -> Self {
        value.clone()
    }
}
#[doc = "PubSubEventSource refers to event-source for GCP PubSub related events."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1PubSubEventSource {
    #[serde(
        rename = "credentialSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "deleteSubscriptionOnFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_subscription_on_finish: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(rename = "projectID", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "subscriptionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(
        rename = "topicProjectID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub topic_project_id: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1PubSubEventSource>
    for IoArgoprojEventsV1alpha1PubSubEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1PubSubEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1PulsarEventSource {
    #[serde(
        rename = "authTokenSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_token_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(
        rename = "tlsAllowInsecureConnection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_allow_insecure_connection: Option<bool>,
    #[serde(
        rename = "tlsTrustCertsSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_trust_certs_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "tlsValidateHostname",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_validate_hostname: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1PulsarEventSource>
    for IoArgoprojEventsV1alpha1PulsarEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1PulsarEventSource) -> Self {
        value.clone()
    }
}
#[doc = "PulsarTrigger refers to the specification of the Pulsar trigger."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1PulsarTrigger {
    #[serde(
        rename = "authTokenSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_token_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "connectionBackoff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_backoff: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[doc = "Parameters is the list of parameters that is applied to resolved Kafka trigger object."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "Payload is the list of key-value extracted from an event payload to construct the request payload."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(
        rename = "tlsAllowInsecureConnection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_allow_insecure_connection: Option<bool>,
    #[serde(
        rename = "tlsTrustCertsSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_trust_certs_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "tlsValidateHostname",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_validate_hostname: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1PulsarTrigger> for IoArgoprojEventsV1alpha1PulsarTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1PulsarTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1RateLimit {
    #[serde(
        rename = "requestsPerUnit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requests_per_unit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1RateLimit> for IoArgoprojEventsV1alpha1RateLimit {
    fn from(value: &IoArgoprojEventsV1alpha1RateLimit) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1RedisEventSource {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(
        rename = "hostAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_address: Option<String>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1RedisEventSource> for IoArgoprojEventsV1alpha1RedisEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1RedisEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1RedisStreamEventSource {
    #[serde(
        rename = "consumerGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consumer_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(
        rename = "hostAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_address: Option<String>,
    #[serde(
        rename = "maxMsgCountPerRead",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_msg_count_per_read: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Streams to look for entries. XREADGROUP is used on all streams using a single consumer group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<IoArgoprojEventsV1alpha1TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1RedisStreamEventSource>
    for IoArgoprojEventsV1alpha1RedisStreamEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1RedisStreamEventSource) -> Self {
        value.clone()
    }
}
#[doc = "Resource represent arbitrary structured data."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1Resource> for IoArgoprojEventsV1alpha1Resource {
    fn from(value: &IoArgoprojEventsV1alpha1Resource) -> Self {
        value.clone()
    }
}
#[doc = "ResourceEventSource refers to a event-source for K8s resource related events."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ResourceEventSource {
    #[doc = "EventTypes is the list of event type to watch.\nPossible values are - ADD, UPDATE and DELETE."]
    #[serde(rename = "eventTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub event_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1ResourceFilter>,
    #[serde(
        rename = "groupVersionResource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_version_resource: Option<IoK8sApimachineryPkgApisMetaV1GroupVersionResource>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1ResourceEventSource>
    for IoArgoprojEventsV1alpha1ResourceEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1ResourceEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ResourceFilter {
    #[serde(
        rename = "afterStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_start: Option<bool>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<IoArgoprojEventsV1alpha1Selector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<IoArgoprojEventsV1alpha1Selector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1ResourceFilter> for IoArgoprojEventsV1alpha1ResourceFilter {
    fn from(value: &IoArgoprojEventsV1alpha1ResourceFilter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1S3Artifact {
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<IoArgoprojEventsV1alpha1S3Bucket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1S3Filter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1S3Artifact> for IoArgoprojEventsV1alpha1S3Artifact {
    fn from(value: &IoArgoprojEventsV1alpha1S3Artifact) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1S3Bucket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1S3Bucket> for IoArgoprojEventsV1alpha1S3Bucket {
    fn from(value: &IoArgoprojEventsV1alpha1S3Bucket) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1S3Filter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1S3Filter> for IoArgoprojEventsV1alpha1S3Filter {
    fn from(value: &IoArgoprojEventsV1alpha1S3Filter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SaslConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1SaslConfig> for IoArgoprojEventsV1alpha1SaslConfig {
    fn from(value: &IoArgoprojEventsV1alpha1SaslConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SecureHeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "valueFrom", default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<IoArgoprojEventsV1alpha1ValueFromSource>,
}
impl From<&IoArgoprojEventsV1alpha1SecureHeader> for IoArgoprojEventsV1alpha1SecureHeader {
    fn from(value: &IoArgoprojEventsV1alpha1SecureHeader) -> Self {
        value.clone()
    }
}
#[doc = "Selector represents conditional operation to select K8s objects."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Selector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1Selector> for IoArgoprojEventsV1alpha1Selector {
    fn from(value: &IoArgoprojEventsV1alpha1Selector) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Sensor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IoArgoprojEventsV1alpha1SensorSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojEventsV1alpha1SensorStatus>,
}
impl From<&IoArgoprojEventsV1alpha1Sensor> for IoArgoprojEventsV1alpha1Sensor {
    fn from(value: &IoArgoprojEventsV1alpha1Sensor) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SensorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoArgoprojEventsV1alpha1Sensor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ListMeta>,
}
impl From<&IoArgoprojEventsV1alpha1SensorList> for IoArgoprojEventsV1alpha1SensorList {
    fn from(value: &IoArgoprojEventsV1alpha1SensorList) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SensorSpec {
    #[doc = "Dependencies is a list of the events that this sensor is dependent on."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<IoArgoprojEventsV1alpha1EventDependency>,
    #[doc = "ErrorOnFailedRound if set to true, marks sensor state as `error` if the previous trigger round fails.\nOnce sensor state is set to `error`, no further triggers will be processed."]
    #[serde(
        rename = "errorOnFailedRound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_on_failed_round: Option<bool>,
    #[serde(
        rename = "eventBusName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_bus_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojEventsV1alpha1Template>,
    #[doc = "Triggers is a list of the things that this sensor evokes. These are the outputs from this sensor."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub triggers: Vec<IoArgoprojEventsV1alpha1Trigger>,
}
impl From<&IoArgoprojEventsV1alpha1SensorSpec> for IoArgoprojEventsV1alpha1SensorSpec {
    fn from(value: &IoArgoprojEventsV1alpha1SensorSpec) -> Self {
        value.clone()
    }
}
#[doc = "SensorStatus contains information about the status of a sensor."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SensorStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojEventsV1alpha1Status>,
}
impl From<&IoArgoprojEventsV1alpha1SensorStatus> for IoArgoprojEventsV1alpha1SensorStatus {
    fn from(value: &IoArgoprojEventsV1alpha1SensorStatus) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Service {
    #[serde(rename = "clusterIP", default, skip_serializing_if = "Option::is_none")]
    pub cluster_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IoK8sApiCoreV1ServicePort>,
}
impl From<&IoArgoprojEventsV1alpha1Service> for IoArgoprojEventsV1alpha1Service {
    fn from(value: &IoArgoprojEventsV1alpha1Service) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SlackEventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(
        rename = "signingSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1SlackEventSource> for IoArgoprojEventsV1alpha1SlackEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1SlackEventSource) -> Self {
        value.clone()
    }
}
#[doc = "SlackTrigger refers to the specification of the slack notification trigger."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SlackTrigger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[doc = "SlackToken refers to the Kubernetes secret that holds the slack token required to send messages."]
    #[serde(
        rename = "slackToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slack_token: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1SlackTrigger> for IoArgoprojEventsV1alpha1SlackTrigger {
    fn from(value: &IoArgoprojEventsV1alpha1SlackTrigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SnsEventSource {
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(rename = "topicArn", default, skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(
        rename = "validateSignature",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub validate_signature: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1SnsEventSource> for IoArgoprojEventsV1alpha1SnsEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1SnsEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1SqsEventSource {
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dlq: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(rename = "jsonBody", default, skip_serializing_if = "Option::is_none")]
    pub json_body: Option<bool>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(
        rename = "queueAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "sessionToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "WaitTimeSeconds is The duration (in seconds) for which the call waits for a message to arrive\nin the queue before returning."]
    #[serde(
        rename = "waitTimeSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wait_time_seconds: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1SqsEventSource> for IoArgoprojEventsV1alpha1SqsEventSource {
    fn from(value: &IoArgoprojEventsV1alpha1SqsEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1StandardK8sTrigger {
    #[serde(
        rename = "liveObject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_object: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Parameters is the list of parameters that is applied to resolved K8s trigger object."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(
        rename = "patchStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patch_strategy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<IoArgoprojEventsV1alpha1ArtifactLocation>,
}
impl From<&IoArgoprojEventsV1alpha1StandardK8sTrigger>
    for IoArgoprojEventsV1alpha1StandardK8sTrigger
{
    fn from(value: &IoArgoprojEventsV1alpha1StandardK8sTrigger) -> Self {
        value.clone()
    }
}
#[doc = "Status is a common structure which can be used for Status field."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Status {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<IoArgoprojEventsV1alpha1Condition>,
}
impl From<&IoArgoprojEventsV1alpha1Status> for IoArgoprojEventsV1alpha1Status {
    fn from(value: &IoArgoprojEventsV1alpha1Status) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1StatusPolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<i32>,
}
impl From<&IoArgoprojEventsV1alpha1StatusPolicy> for IoArgoprojEventsV1alpha1StatusPolicy {
    fn from(value: &IoArgoprojEventsV1alpha1StatusPolicy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1StorageGridEventSource {
    #[doc = "APIURL is the url of the storagegrid api."]
    #[serde(rename = "apiURL", default, skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(rename = "authToken", default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Name of the bucket to register notifications for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[doc = "Filter on object key which caused the notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1StorageGridFilter>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "topicArn", default, skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1StorageGridEventSource>
    for IoArgoprojEventsV1alpha1StorageGridEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1StorageGridEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1StorageGridFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1StorageGridFilter>
    for IoArgoprojEventsV1alpha1StorageGridFilter
{
    fn from(value: &IoArgoprojEventsV1alpha1StorageGridFilter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1StripeEventSource {
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "createWebhook",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_webhook: Option<bool>,
    #[serde(rename = "eventFilter", default, skip_serializing_if = "Vec::is_empty")]
    pub event_filter: Vec<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1StripeEventSource>
    for IoArgoprojEventsV1alpha1StripeEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1StripeEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Template {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<IoK8sApiCoreV1Affinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<IoK8sApiCoreV1Container>,
    #[serde(
        rename = "imagePullSecrets",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_pull_secrets: Vec<IoK8sApiCoreV1LocalObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoArgoprojEventsV1alpha1Metadata>,
    #[serde(
        rename = "nodeSelector",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub node_selector: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(
        rename = "priorityClassName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub priority_class_name: Option<String>,
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1PodSecurityContext>,
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<IoK8sApiCoreV1Toleration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<IoK8sApiCoreV1Volume>,
}
impl From<&IoArgoprojEventsV1alpha1Template> for IoArgoprojEventsV1alpha1Template {
    fn from(value: &IoArgoprojEventsV1alpha1Template) -> Self {
        value.clone()
    }
}
#[doc = "TimeFilter describes a window in time.\nIt filters out events that occur outside the time limits.\nIn other words, only events that occur after Start and before Stop\nwill pass this filter."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1TimeFilter {
    #[doc = "Start is the beginning of a time window in UTC.\nBefore this time, events for this dependency are ignored.\nFormat is hh:mm:ss."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[doc = "Stop is the end of a time window in UTC.\nAfter or equal to this time, events for this dependency are ignored and\nFormat is hh:mm:ss.\nIf it is smaller than Start, it is treated as next day of Start\n(e.g.: 22:00:00-01:00:00 means 22:00:00-25:00:00)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1TimeFilter> for IoArgoprojEventsV1alpha1TimeFilter {
    fn from(value: &IoArgoprojEventsV1alpha1TimeFilter) -> Self {
        value.clone()
    }
}
#[doc = "TLSConfig refers to TLS configuration for a client."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1TlsConfig {
    #[serde(
        rename = "caCertSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ca_cert_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "clientCertSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_cert_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "clientKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "insecureSkipVerify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insecure_skip_verify: Option<bool>,
}
impl From<&IoArgoprojEventsV1alpha1TlsConfig> for IoArgoprojEventsV1alpha1TlsConfig {
    fn from(value: &IoArgoprojEventsV1alpha1TlsConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1Trigger {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojEventsV1alpha1TriggerParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<IoArgoprojEventsV1alpha1TriggerPolicy>,
    #[serde(rename = "rateLimit", default, skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<IoArgoprojEventsV1alpha1RateLimit>,
    #[serde(
        rename = "retryStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_strategy: Option<IoArgoprojEventsV1alpha1Backoff>,
    #[doc = "Template describes the trigger specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojEventsV1alpha1TriggerTemplate>,
}
impl From<&IoArgoprojEventsV1alpha1Trigger> for IoArgoprojEventsV1alpha1Trigger {
    fn from(value: &IoArgoprojEventsV1alpha1Trigger) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1TriggerParameter {
    #[doc = "Dest is the JSONPath of a resource key.\nA path is a series of keys separated by a dot. The colon character can be escaped with '.'\nThe -1 key can be used to append a value to an existing array.\nSee https://github.com/tidwall/sjson#path-syntax for more information about how this is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    #[doc = "Operation is what to do with the existing value at Dest, whether to\n'prepend', 'overwrite', or 'append' it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<IoArgoprojEventsV1alpha1TriggerParameterSource>,
}
impl From<&IoArgoprojEventsV1alpha1TriggerParameter> for IoArgoprojEventsV1alpha1TriggerParameter {
    fn from(value: &IoArgoprojEventsV1alpha1TriggerParameter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1TriggerParameterSource {
    #[doc = "ContextKey is the JSONPath of the event's (JSON decoded) context key\nContextKey is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'.\nTo access an array value use the index as the key. The dot and wildcard characters can be escaped with '\\\\'.\nSee https://github.com/tidwall/gjson#path-syntax for more information on how to use this."]
    #[serde(
        rename = "contextKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_key: Option<String>,
    #[serde(
        rename = "contextTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_template: Option<String>,
    #[doc = "DataKey is the JSONPath of the event's (JSON decoded) data key\nDataKey is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'.\nTo access an array value use the index as the key. The dot and wildcard characters can be escaped with '\\\\'.\nSee https://github.com/tidwall/gjson#path-syntax for more information on how to use this."]
    #[serde(rename = "dataKey", default, skip_serializing_if = "Option::is_none")]
    pub data_key: Option<String>,
    #[serde(
        rename = "dataTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_template: Option<String>,
    #[doc = "DependencyName refers to the name of the dependency. The event which is stored for this dependency is used as payload\nfor the parameterization. Make sure to refer to one of the dependencies you have defined under Dependencies list."]
    #[serde(
        rename = "dependencyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dependency_name: Option<String>,
    #[doc = "Value is the default literal value to use for this parameter source\nThis is only used if the DataKey is invalid.\nIf the DataKey is invalid and this is not defined, this param source will produce an error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1TriggerParameterSource>
    for IoArgoprojEventsV1alpha1TriggerParameterSource
{
    fn from(value: &IoArgoprojEventsV1alpha1TriggerParameterSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1TriggerPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k8s: Option<IoArgoprojEventsV1alpha1K8sResourcePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojEventsV1alpha1StatusPolicy>,
}
impl From<&IoArgoprojEventsV1alpha1TriggerPolicy> for IoArgoprojEventsV1alpha1TriggerPolicy {
    fn from(value: &IoArgoprojEventsV1alpha1TriggerPolicy) -> Self {
        value.clone()
    }
}
#[doc = "TriggerTemplate is the template that describes trigger specification."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1TriggerTemplate {
    #[serde(
        rename = "argoWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub argo_workflow: Option<IoArgoprojEventsV1alpha1ArgoWorkflowTrigger>,
    #[serde(rename = "awsLambda", default, skip_serializing_if = "Option::is_none")]
    pub aws_lambda: Option<IoArgoprojEventsV1alpha1AwsLambdaTrigger>,
    #[serde(
        rename = "azureEventHubs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_event_hubs: Option<IoArgoprojEventsV1alpha1AzureEventHubsTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    #[serde(
        rename = "conditionsReset",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions_reset: Vec<IoArgoprojEventsV1alpha1ConditionsResetCriteria>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<IoArgoprojEventsV1alpha1CustomTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<IoArgoprojEventsV1alpha1HttpTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k8s: Option<IoArgoprojEventsV1alpha1StandardK8sTrigger>,
    #[doc = "Kafka refers to the trigger designed to place messages on Kafka topic.\n+optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kafka: Option<IoArgoprojEventsV1alpha1KafkaTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<IoArgoprojEventsV1alpha1LogTrigger>,
    #[doc = "Name is a unique name of the action to take."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "NATS refers to the trigger designed to place message on NATS subject.\n+optional."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nats: Option<IoArgoprojEventsV1alpha1NatsTrigger>,
    #[serde(rename = "openWhisk", default, skip_serializing_if = "Option::is_none")]
    pub open_whisk: Option<IoArgoprojEventsV1alpha1OpenWhiskTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulsar: Option<IoArgoprojEventsV1alpha1PulsarTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slack: Option<IoArgoprojEventsV1alpha1SlackTrigger>,
}
impl From<&IoArgoprojEventsV1alpha1TriggerTemplate> for IoArgoprojEventsV1alpha1TriggerTemplate {
    fn from(value: &IoArgoprojEventsV1alpha1TriggerTemplate) -> Self {
        value.clone()
    }
}
#[doc = "URLArtifact contains information about an artifact at an http endpoint."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1UrlArtifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "verifyCert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verify_cert: Option<bool>,
}
impl From<&IoArgoprojEventsV1alpha1UrlArtifact> for IoArgoprojEventsV1alpha1UrlArtifact {
    fn from(value: &IoArgoprojEventsV1alpha1UrlArtifact) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1ValueFromSource {
    #[serde(
        rename = "configMapKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_map_key_ref: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[serde(
        rename = "secretKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_ref: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojEventsV1alpha1ValueFromSource> for IoArgoprojEventsV1alpha1ValueFromSource {
    fn from(value: &IoArgoprojEventsV1alpha1ValueFromSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1WatchPathConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "pathRegexp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_regexp: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1WatchPathConfig> for IoArgoprojEventsV1alpha1WatchPathConfig {
    fn from(value: &IoArgoprojEventsV1alpha1WatchPathConfig) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1WebhookContext {
    #[serde(
        rename = "authSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(
        rename = "maxPayloadSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_payload_size: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "Port on which HTTP server is listening for incoming events."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "ServerCertPath refers the file that contains the cert."]
    #[serde(
        rename = "serverCertSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_cert_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "serverKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "URL is the url of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IoArgoprojEventsV1alpha1WebhookContext> for IoArgoprojEventsV1alpha1WebhookContext {
    fn from(value: &IoArgoprojEventsV1alpha1WebhookContext) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojEventsV1alpha1WebhookEventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<IoArgoprojEventsV1alpha1EventSourceFilter>,
    #[serde(
        rename = "webhookContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_context: Option<IoArgoprojEventsV1alpha1WebhookContext>,
}
impl From<&IoArgoprojEventsV1alpha1WebhookEventSource>
    for IoArgoprojEventsV1alpha1WebhookEventSource
{
    fn from(value: &IoArgoprojEventsV1alpha1WebhookEventSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Amount(pub f64);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1Amount {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1Amount> for f64 {
    fn from(value: IoArgoprojWorkflowV1alpha1Amount) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1Amount> for IoArgoprojWorkflowV1alpha1Amount {
    fn from(value: &IoArgoprojWorkflowV1alpha1Amount) -> Self {
        value.clone()
    }
}
impl From<f64> for IoArgoprojWorkflowV1alpha1Amount {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IoArgoprojWorkflowV1alpha1Amount {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for IoArgoprojWorkflowV1alpha1Amount {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoArgoprojWorkflowV1alpha1Amount {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoArgoprojWorkflowV1alpha1Amount {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for IoArgoprojWorkflowV1alpha1Amount {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "ArchiveStrategy describes how to archive files/directory when saving artifacts"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArchiveStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub none: Option<IoArgoprojWorkflowV1alpha1NoneStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tar: Option<IoArgoprojWorkflowV1alpha1TarStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<IoArgoprojWorkflowV1alpha1ZipStrategy>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArchiveStrategy>
    for IoArgoprojWorkflowV1alpha1ArchiveStrategy
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArchiveStrategy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse>
    for IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1ArchivedWorkflowDeletedResponse
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}

pub use IoArgoprojWorkflowV1alpha1ArgumentsBuilder as ArgumentsBuilder;
pub use IoArgoprojWorkflowV1alpha1ArgumentsBuilderError as ArgumentsBuilderError;

#[doc = "Arguments to a template"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
pub struct IoArgoprojWorkflowV1alpha1Arguments {
    #[builder(default)]
    #[doc = "Artifacts is the list of artifacts to pass to the template or workflow"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<IoArgoprojWorkflowV1alpha1Artifact>,
    #[doc = "Parameters is the list of parameters to pass to the template or workflow"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojWorkflowV1alpha1Parameter>,
}
impl From<&IoArgoprojWorkflowV1alpha1Arguments> for IoArgoprojWorkflowV1alpha1Arguments {
    fn from(value: &IoArgoprojWorkflowV1alpha1Arguments) -> Self {
        value.clone()
    }
}
#[doc = "ArtGCStatus maintains state related to ArtifactGC"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtGcStatus {
    #[doc = "if this is true, we already checked to see if we need to do it and we don't"]
    #[serde(
        rename = "notSpecified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_specified: Option<bool>,
    #[doc = "have completed Pods been processed? (mapped by Pod name) used to prevent re-processing the Status of a Pod more than once"]
    #[serde(
        rename = "podsRecouped",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub pods_recouped: std::collections::HashMap<String, bool>,
    #[doc = "have Pods been started to perform this strategy? (enables us not to re-process what we've already done)"]
    #[serde(
        rename = "strategiesProcessed",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub strategies_processed: std::collections::HashMap<String, bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtGcStatus> for IoArgoprojWorkflowV1alpha1ArtGcStatus {
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtGcStatus) -> Self {
        value.clone()
    }
}
#[doc = "Artifact indicates an artifact to place at a specified path"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Artifact {
    #[doc = "Archive controls how the artifact will be saved to the artifact repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<IoArgoprojWorkflowV1alpha1ArchiveStrategy>,
    #[doc = "ArchiveLogs indicates if the container logs should be archived"]
    #[serde(
        rename = "archiveLogs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_logs: Option<bool>,
    #[doc = "ArtifactGC describes the strategy to use when to deleting an artifact from completed or deleted workflows"]
    #[serde(
        rename = "artifactGC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_gc: Option<IoArgoprojWorkflowV1alpha1ArtifactGc>,
    #[doc = "Artifactory contains artifactory artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<IoArgoprojWorkflowV1alpha1ArtifactoryArtifact>,
    #[doc = "Azure contains Azure Storage artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<IoArgoprojWorkflowV1alpha1AzureArtifact>,
    #[doc = "Has this been deleted?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "From allows an artifact to reference an artifact from a previous step"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "FromExpression, if defined, is evaluated to specify the value for the artifact"]
    #[serde(
        rename = "fromExpression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_expression: Option<String>,
    #[doc = "GCS contains GCS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<IoArgoprojWorkflowV1alpha1GcsArtifact>,
    #[doc = "Git contains git artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<IoArgoprojWorkflowV1alpha1GitArtifact>,
    #[doc = "GlobalName exports an output artifact to the global scope, making it available as '{{io.argoproj.workflow.v1alpha1.outputs.artifacts.XXXX}} and in workflow.status.outputs.artifacts"]
    #[serde(
        rename = "globalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_name: Option<String>,
    #[doc = "HDFS contains HDFS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<IoArgoprojWorkflowV1alpha1HdfsArtifact>,
    #[doc = "HTTP contains HTTP artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<IoArgoprojWorkflowV1alpha1HttpArtifact>,
    #[doc = "mode bits to use on this file, must be a value between 0 and 0777 set when loading input artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[doc = "name of the artifact. must be unique within a template's inputs/outputs."]
    pub name: String,
    #[doc = "Make Artifacts optional, if Artifacts doesn't generate or exist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[doc = "OSS contains OSS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oss: Option<IoArgoprojWorkflowV1alpha1OssArtifact>,
    #[doc = "Path is the container path to the artifact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Raw contains raw artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<IoArgoprojWorkflowV1alpha1RawArtifact>,
    #[doc = "If mode is set, apply the permission recursively into the artifact if it is a folder"]
    #[serde(
        rename = "recurseMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recurse_mode: Option<bool>,
    #[doc = "S3 contains S3 artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<IoArgoprojWorkflowV1alpha1S3Artifact>,
    #[doc = "SubPath allows an artifact to be sourced from a subpath within the specified source"]
    #[serde(rename = "subPath", default, skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Artifact> for IoArgoprojWorkflowV1alpha1Artifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1Artifact) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactGC describes how to delete artifacts from completed Workflows - this is embedded into the WorkflowLevelArtifactGC, and also used for individual Artifacts to override that as needed"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactGc {
    #[doc = "PodMetadata is an optional field for specifying the Labels and Annotations that should be assigned to the Pod doing the deletion"]
    #[serde(
        rename = "podMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_metadata: Option<IoArgoprojWorkflowV1alpha1Metadata>,
    #[doc = "ServiceAccountName is an optional field for specifying the Service Account that should be assigned to the Pod doing the deletion"]
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
    #[doc = "Strategy is the strategy to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactGc> for IoArgoprojWorkflowV1alpha1ArtifactGc {
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactGc) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactGCSpec specifies the Artifacts that need to be deleted"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactGcSpec {
    #[doc = "ArtifactsByNode maps Node name to information pertaining to Artifacts on that Node"]
    #[serde(
        rename = "artifactsByNode",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub artifacts_by_node:
        std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1ArtifactNodeSpec>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactGcSpec> for IoArgoprojWorkflowV1alpha1ArtifactGcSpec {
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactGcSpec) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactGCStatus describes the result of the deletion"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactGcStatus {
    #[doc = "ArtifactResultsByNode maps Node name to result"]
    #[serde(
        rename = "artifactResultsByNode",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub artifact_results_by_node:
        std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1ArtifactResultNodeStatus>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactGcStatus>
    for IoArgoprojWorkflowV1alpha1ArtifactGcStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactGcStatus) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactLocation describes a location for a single or multiple artifacts. It is used as single artifact in the context of inputs/outputs (e.g. outputs.artifacts.artname). It is also used to describe the location of multiple artifacts such as the archive location of a single workflow step, which the executor will use as a default location to store its files."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactLocation {
    #[doc = "ArchiveLogs indicates if the container logs should be archived"]
    #[serde(
        rename = "archiveLogs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_logs: Option<bool>,
    #[doc = "Artifactory contains artifactory artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<IoArgoprojWorkflowV1alpha1ArtifactoryArtifact>,
    #[doc = "Azure contains Azure Storage artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<IoArgoprojWorkflowV1alpha1AzureArtifact>,
    #[doc = "GCS contains GCS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<IoArgoprojWorkflowV1alpha1GcsArtifact>,
    #[doc = "Git contains git artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<IoArgoprojWorkflowV1alpha1GitArtifact>,
    #[doc = "HDFS contains HDFS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<IoArgoprojWorkflowV1alpha1HdfsArtifact>,
    #[doc = "HTTP contains HTTP artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<IoArgoprojWorkflowV1alpha1HttpArtifact>,
    #[doc = "OSS contains OSS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oss: Option<IoArgoprojWorkflowV1alpha1OssArtifact>,
    #[doc = "Raw contains raw artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<IoArgoprojWorkflowV1alpha1RawArtifact>,
    #[doc = "S3 contains S3 artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<IoArgoprojWorkflowV1alpha1S3Artifact>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactLocation>
    for IoArgoprojWorkflowV1alpha1ArtifactLocation
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactLocation) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactNodeSpec specifies the Artifacts that need to be deleted for a given Node"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactNodeSpec {
    #[doc = "ArchiveLocation is the template-level Artifact location specification"]
    #[serde(
        rename = "archiveLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_location: Option<IoArgoprojWorkflowV1alpha1ArtifactLocation>,
    #[doc = "Artifacts maps artifact name to Artifact description"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub artifacts: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1Artifact>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactNodeSpec>
    for IoArgoprojWorkflowV1alpha1ArtifactNodeSpec
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactNodeSpec) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactPaths expands a step from a collection of artifacts"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactPaths {
    #[doc = "Archive controls how the artifact will be saved to the artifact repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<IoArgoprojWorkflowV1alpha1ArchiveStrategy>,
    #[doc = "ArchiveLogs indicates if the container logs should be archived"]
    #[serde(
        rename = "archiveLogs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_logs: Option<bool>,
    #[doc = "ArtifactGC describes the strategy to use when to deleting an artifact from completed or deleted workflows"]
    #[serde(
        rename = "artifactGC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_gc: Option<IoArgoprojWorkflowV1alpha1ArtifactGc>,
    #[doc = "Artifactory contains artifactory artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<IoArgoprojWorkflowV1alpha1ArtifactoryArtifact>,
    #[doc = "Azure contains Azure Storage artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<IoArgoprojWorkflowV1alpha1AzureArtifact>,
    #[doc = "Has this been deleted?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "From allows an artifact to reference an artifact from a previous step"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "FromExpression, if defined, is evaluated to specify the value for the artifact"]
    #[serde(
        rename = "fromExpression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_expression: Option<String>,
    #[doc = "GCS contains GCS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<IoArgoprojWorkflowV1alpha1GcsArtifact>,
    #[doc = "Git contains git artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<IoArgoprojWorkflowV1alpha1GitArtifact>,
    #[doc = "GlobalName exports an output artifact to the global scope, making it available as '{{io.argoproj.workflow.v1alpha1.outputs.artifacts.XXXX}} and in workflow.status.outputs.artifacts"]
    #[serde(
        rename = "globalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_name: Option<String>,
    #[doc = "HDFS contains HDFS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<IoArgoprojWorkflowV1alpha1HdfsArtifact>,
    #[doc = "HTTP contains HTTP artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<IoArgoprojWorkflowV1alpha1HttpArtifact>,
    #[doc = "mode bits to use on this file, must be a value between 0 and 0777 set when loading input artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[doc = "name of the artifact. must be unique within a template's inputs/outputs."]
    pub name: String,
    #[doc = "Make Artifacts optional, if Artifacts doesn't generate or exist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[doc = "OSS contains OSS artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oss: Option<IoArgoprojWorkflowV1alpha1OssArtifact>,
    #[doc = "Path is the container path to the artifact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Raw contains raw artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<IoArgoprojWorkflowV1alpha1RawArtifact>,
    #[doc = "If mode is set, apply the permission recursively into the artifact if it is a folder"]
    #[serde(
        rename = "recurseMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recurse_mode: Option<bool>,
    #[doc = "S3 contains S3 artifact location details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<IoArgoprojWorkflowV1alpha1S3Artifact>,
    #[doc = "SubPath allows an artifact to be sourced from a subpath within the specified source"]
    #[serde(rename = "subPath", default, skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactPaths> for IoArgoprojWorkflowV1alpha1ArtifactPaths {
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactPaths) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactRepository represents an artifact repository in which a controller will store its artifacts"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactRepository {
    #[doc = "ArchiveLogs enables log archiving"]
    #[serde(
        rename = "archiveLogs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_logs: Option<bool>,
    #[doc = "Artifactory stores artifacts to JFrog Artifactory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<IoArgoprojWorkflowV1alpha1ArtifactoryArtifactRepository>,
    #[doc = "Azure stores artifact in an Azure Storage account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<IoArgoprojWorkflowV1alpha1AzureArtifactRepository>,
    #[doc = "GCS stores artifact in a GCS object store"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<IoArgoprojWorkflowV1alpha1GcsArtifactRepository>,
    #[doc = "HDFS stores artifacts in HDFS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<IoArgoprojWorkflowV1alpha1HdfsArtifactRepository>,
    #[doc = "OSS stores artifact in a OSS-compliant object store"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oss: Option<IoArgoprojWorkflowV1alpha1OssArtifactRepository>,
    #[doc = "S3 stores artifact in a S3-compliant object store"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<IoArgoprojWorkflowV1alpha1S3ArtifactRepository>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactRepository>
    for IoArgoprojWorkflowV1alpha1ArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactRepository) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactRepositoryRef {
    #[doc = "The name of the config map. Defaults to \"artifact-repositories\"."]
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<String>,
    #[doc = "The config map key. Defaults to the value of the \"workflows.argoproj.io/default-artifact-repository\" annotation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactRepositoryRef>
    for IoArgoprojWorkflowV1alpha1ArtifactRepositoryRef
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactRepositoryRef) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactRepositoryRefStatus {
    #[doc = "The repository the workflow will use. This maybe empty before v3.1."]
    #[serde(
        rename = "artifactRepository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_repository: Option<IoArgoprojWorkflowV1alpha1ArtifactRepository>,
    #[doc = "The name of the config map. Defaults to \"artifact-repositories\"."]
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<String>,
    #[doc = "If this ref represents the default artifact repository, rather than a config map."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[doc = "The config map key. Defaults to the value of the \"workflows.argoproj.io/default-artifact-repository\" annotation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "The namespace of the config map. Defaults to the workflow's namespace, or the controller's namespace (if found)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactRepositoryRefStatus>
    for IoArgoprojWorkflowV1alpha1ArtifactRepositoryRefStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactRepositoryRefStatus) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactResult describes the result of attempting to delete a given Artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactResult {
    #[doc = "Error is an optional error message which should be set if Success==false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Name is the name of the Artifact"]
    pub name: String,
    #[doc = "Success describes whether the deletion succeeded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactResult> for IoArgoprojWorkflowV1alpha1ArtifactResult {
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactResult) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactResultNodeStatus describes the result of the deletion on a given node"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactResultNodeStatus {
    #[doc = "ArtifactResults maps Artifact name to result of the deletion"]
    #[serde(
        rename = "artifactResults",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub artifact_results:
        std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1ArtifactResult>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactResultNodeStatus>
    for IoArgoprojWorkflowV1alpha1ArtifactResultNodeStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactResultNodeStatus) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactoryArtifact is the location of an artifactory artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactoryArtifact {
    #[doc = "PasswordSecret is the secret selector to the repository password"]
    #[serde(
        rename = "passwordSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "URL of the artifact"]
    pub url: String,
    #[doc = "UsernameSecret is the secret selector to the repository username"]
    #[serde(
        rename = "usernameSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactoryArtifact>
    for IoArgoprojWorkflowV1alpha1ArtifactoryArtifact
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactoryArtifact) -> Self {
        value.clone()
    }
}
#[doc = "ArtifactoryArtifactRepository defines the controller configuration for an artifactory artifact repository"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ArtifactoryArtifactRepository {
    #[doc = "KeyFormat defines the format of how to store keys and can reference workflow variables."]
    #[serde(rename = "keyFormat", default, skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[doc = "PasswordSecret is the secret selector to the repository password"]
    #[serde(
        rename = "passwordSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "RepoURL is the url for artifactory repo."]
    #[serde(rename = "repoURL", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "UsernameSecret is the secret selector to the repository username"]
    #[serde(
        rename = "usernameSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1ArtifactoryArtifactRepository>
    for IoArgoprojWorkflowV1alpha1ArtifactoryArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ArtifactoryArtifactRepository) -> Self {
        value.clone()
    }
}
#[doc = "AzureArtifact is the location of a an Azure Storage artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1AzureArtifact {
    #[doc = "AccountKeySecret is the secret selector to the Azure Blob Storage account access key"]
    #[serde(
        rename = "accountKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Blob is the blob name (i.e., path) in the container where the artifact resides"]
    pub blob: String,
    #[doc = "Container is the container where resources will be stored"]
    pub container: String,
    #[doc = "Endpoint is the service url associated with an account. It is most likely \"https://<ACCOUNT_NAME>.blob.core.windows.net\""]
    pub endpoint: String,
    #[doc = "UseSDKCreds tells the driver to figure out credentials based on sdk defaults."]
    #[serde(
        rename = "useSDKCreds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sdk_creds: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1AzureArtifact> for IoArgoprojWorkflowV1alpha1AzureArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1AzureArtifact) -> Self {
        value.clone()
    }
}
#[doc = "AzureArtifactRepository defines the controller configuration for an Azure Blob Storage artifact repository"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1AzureArtifactRepository {
    #[doc = "AccountKeySecret is the secret selector to the Azure Blob Storage account access key"]
    #[serde(
        rename = "accountKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "BlobNameFormat is defines the format of how to store blob names. Can reference workflow variables"]
    #[serde(
        rename = "blobNameFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_name_format: Option<String>,
    #[doc = "Container is the container where resources will be stored"]
    pub container: String,
    #[doc = "Endpoint is the service url associated with an account. It is most likely \"https://<ACCOUNT_NAME>.blob.core.windows.net\""]
    pub endpoint: String,
    #[doc = "UseSDKCreds tells the driver to figure out credentials based on sdk defaults."]
    #[serde(
        rename = "useSDKCreds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sdk_creds: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1AzureArtifactRepository>
    for IoArgoprojWorkflowV1alpha1AzureArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1AzureArtifactRepository) -> Self {
        value.clone()
    }
}
#[doc = "Backoff is a backoff strategy to use within retryStrategy"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Backoff {
    #[doc = "Duration is the amount to back off. Default unit is seconds, but could also be a duration (e.g. \"2m\", \"1h\")"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Factor is a factor to multiply the base duration after each failed retry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "MaxDuration is the maximum amount of time allowed for a workflow in the backoff strategy"]
    #[serde(
        rename = "maxDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_duration: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Backoff> for IoArgoprojWorkflowV1alpha1Backoff {
    fn from(value: &IoArgoprojWorkflowV1alpha1Backoff) -> Self {
        value.clone()
    }
}
#[doc = "BasicAuth describes the secret selectors required for basic authentication"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1BasicAuth {
    #[doc = "PasswordSecret is the secret selector to the repository password"]
    #[serde(
        rename = "passwordSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "UsernameSecret is the secret selector to the repository username"]
    #[serde(
        rename = "usernameSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1BasicAuth> for IoArgoprojWorkflowV1alpha1BasicAuth {
    fn from(value: &IoArgoprojWorkflowV1alpha1BasicAuth) -> Self {
        value.clone()
    }
}
#[doc = "Cache is the configuration for the type of cache to be used"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Cache {
    #[doc = "ConfigMap sets a ConfigMap-based cache"]
    #[serde(rename = "configMap")]
    pub config_map: IoK8sApiCoreV1ConfigMapKeySelector,
}
impl From<&IoArgoprojWorkflowV1alpha1Cache> for IoArgoprojWorkflowV1alpha1Cache {
    fn from(value: &IoArgoprojWorkflowV1alpha1Cache) -> Self {
        value.clone()
    }
}
#[doc = "ClientCertAuth holds necessary information for client authentication via certificates"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClientCertAuth {
    #[serde(
        rename = "clientCertSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_cert_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "clientKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1ClientCertAuth> for IoArgoprojWorkflowV1alpha1ClientCertAuth {
    fn from(value: &IoArgoprojWorkflowV1alpha1ClientCertAuth) -> Self {
        value.clone()
    }
}
#[doc = "ClusterWorkflowTemplate is the definition of a workflow template resource in cluster scope"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    pub spec: IoArgoprojWorkflowV1alpha1WorkflowSpec,
}
impl From<&IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateCreateRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate>,
}
impl From<&IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateCreateRequest>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateCreateRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateCreateRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateDeleteResponse
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateLintRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate>,
}
impl From<&IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateLintRequest>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateLintRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateLintRequest) -> Self {
        value.clone()
    }
}
#[doc = "ClusterWorkflowTemplateList is list of ClusterWorkflowTemplate resources"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateList {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    pub items: Vec<IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ListMeta,
}
impl From<&IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateList>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateList
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateList) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateUpdateRequest {
    #[doc = "DEPRECATED: This field is ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplate>,
}
impl From<&IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateUpdateRequest>
    for IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateUpdateRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ClusterWorkflowTemplateUpdateRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CollectEventRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1CollectEventRequest>
    for IoArgoprojWorkflowV1alpha1CollectEventRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CollectEventRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CollectEventResponse(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1CollectEventResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1CollectEventResponse>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1CollectEventResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1CollectEventResponse>
    for IoArgoprojWorkflowV1alpha1CollectEventResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CollectEventResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1CollectEventResponse
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Column is a custom column that will be exposed in the Workflow List View."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Column {
    #[doc = "The key of the label or annotation, e.g., \"workflows.argoproj.io/completed\"."]
    pub key: String,
    #[doc = "The name of this column, e.g., \"Workflow Completed\"."]
    pub name: String,
    #[doc = "The type of this column, \"label\" or \"annotation\"."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Column> for IoArgoprojWorkflowV1alpha1Column {
    fn from(value: &IoArgoprojWorkflowV1alpha1Column) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Condition {
    #[doc = "Message is the condition message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Status is the status of the condition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Type is the type of condition"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Condition> for IoArgoprojWorkflowV1alpha1Condition {
    fn from(value: &IoArgoprojWorkflowV1alpha1Condition) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ContainerNode {
    #[doc = "Arguments to the entrypoint. The container image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[doc = "Entrypoint array. Not executed within a shell. The container image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[doc = "List of environment variables to set in the container. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<IoK8sApiCoreV1EnvVar>,
    #[doc = "List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated."]
    #[serde(rename = "envFrom", default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<IoK8sApiCoreV1EnvFromSource>,
    #[doc = "Container image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images"]
    #[serde(
        rename = "imagePullPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_pull_policy: Option<String>,
    #[doc = "Actions that the management system should take in response to container lifecycle events. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<IoK8sApiCoreV1Lifecycle>,
    #[doc = "Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "livenessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub liveness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated."]
    pub name: String,
    #[doc = "List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IoK8sApiCoreV1ContainerPort>,
    #[doc = "Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "readinessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readiness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<IoK8sApiCoreV1ResourceRequirements>,
    #[doc = "SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/"]
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1SecurityContext>,
    #[doc = "StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "startupProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    #[doc = "Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false"]
    #[serde(rename = "stdinOnce", default, skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    #[doc = "Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_path: Option<String>,
    #[doc = "Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_policy: Option<String>,
    #[doc = "Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[doc = "volumeDevices is the list of block devices to be used by the container."]
    #[serde(
        rename = "volumeDevices",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_devices: Vec<IoK8sApiCoreV1VolumeDevice>,
    #[doc = "Pod volumes to mount into the container's filesystem. Cannot be updated."]
    #[serde(
        rename = "volumeMounts",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_mounts: Vec<IoK8sApiCoreV1VolumeMount>,
    #[doc = "Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated."]
    #[serde(
        rename = "workingDir",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_dir: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ContainerNode> for IoArgoprojWorkflowV1alpha1ContainerNode {
    fn from(value: &IoArgoprojWorkflowV1alpha1ContainerNode) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ContainerSetRetryStrategy {
    #[doc = "Duration is the time between each retry, examples values are \"300ms\", \"1s\" or \"5m\". Valid time units are \"ns\", \"us\" (or \"µs\"), \"ms\", \"s\", \"m\", \"h\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Nbr of retries"]
    pub retries: IoK8sApimachineryPkgUtilIntstrIntOrString,
}
impl From<&IoArgoprojWorkflowV1alpha1ContainerSetRetryStrategy>
    for IoArgoprojWorkflowV1alpha1ContainerSetRetryStrategy
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ContainerSetRetryStrategy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ContainerSetTemplate {
    pub containers: Vec<IoArgoprojWorkflowV1alpha1ContainerNode>,
    #[doc = "RetryStrategy describes how to retry a container nodes in the container set if it fails. Nbr of retries(default 0) and sleep duration between retries(default 0s, instant retry) can be set."]
    #[serde(
        rename = "retryStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_strategy: Option<IoArgoprojWorkflowV1alpha1ContainerSetRetryStrategy>,
    #[serde(
        rename = "volumeMounts",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_mounts: Vec<IoK8sApiCoreV1VolumeMount>,
}
impl From<&IoArgoprojWorkflowV1alpha1ContainerSetTemplate>
    for IoArgoprojWorkflowV1alpha1ContainerSetTemplate
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ContainerSetTemplate) -> Self {
        value.clone()
    }
}
#[doc = "ContinueOn defines if a workflow should continue even if a task or step fails/errors. It can be specified if the workflow should continue when the pod errors, fails or both."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ContinueOn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1ContinueOn> for IoArgoprojWorkflowV1alpha1ContinueOn {
    fn from(value: &IoArgoprojWorkflowV1alpha1ContinueOn) -> Self {
        value.clone()
    }
}
#[doc = "Counter is a Counter prometheus metric"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Counter {
    #[doc = "Value is the value of the metric"]
    pub value: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Counter> for IoArgoprojWorkflowV1alpha1Counter {
    fn from(value: &IoArgoprojWorkflowV1alpha1Counter) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CreateCronWorkflowRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[serde(
        rename = "cronWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cron_workflow: Option<IoArgoprojWorkflowV1alpha1CronWorkflow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1CreateCronWorkflowRequest>
    for IoArgoprojWorkflowV1alpha1CreateCronWorkflowRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CreateCronWorkflowRequest) -> Self {
        value.clone()
    }
}
#[doc = "CreateS3BucketOptions options used to determine automatic automatic bucket-creation process"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CreateS3BucketOptions {
    #[doc = "ObjectLocking Enable object locking"]
    #[serde(
        rename = "objectLocking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub object_locking: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1CreateS3BucketOptions>
    for IoArgoprojWorkflowV1alpha1CreateS3BucketOptions
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CreateS3BucketOptions) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1CronWorkflowBuilder as CronWorkflowBuilder;
pub use IoArgoprojWorkflowV1alpha1CronWorkflowBuilderError as CronWorkflowBuilderError;

#[doc = "CronWorkflow is the definition of a scheduled workflow resource"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflow {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    pub spec: IoArgoprojWorkflowV1alpha1CronWorkflowSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojWorkflowV1alpha1CronWorkflowStatus>,
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflow> for IoArgoprojWorkflowV1alpha1CronWorkflow {
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflow) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse>
    for IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1CronWorkflowDeletedResponse
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "CronWorkflowList is list of CronWorkflow resources"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflowList {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    pub items: Vec<IoArgoprojWorkflowV1alpha1CronWorkflow>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ListMeta,
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflowList>
    for IoArgoprojWorkflowV1alpha1CronWorkflowList
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflowList) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflowResumeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflowResumeRequest>
    for IoArgoprojWorkflowV1alpha1CronWorkflowResumeRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflowResumeRequest) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1CronWorkflowSpecBuilder as CronWorkflowSpecBuilder;
pub use IoArgoprojWorkflowV1alpha1CronWorkflowSpecBuilderError as CronWorkflowSpecBuilderError;

#[doc = "CronWorkflowSpec is the specification of a CronWorkflow"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflowSpec {
    #[doc = "ConcurrencyPolicy is the K8s-style concurrency policy that will be used"]
    #[serde(
        rename = "concurrencyPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrency_policy: Option<String>,
    #[doc = "FailedJobsHistoryLimit is the number of failed jobs to be kept at a time"]
    #[serde(
        rename = "failedJobsHistoryLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_jobs_history_limit: Option<i64>,
    #[doc = "Schedule is a schedule to run the Workflow in Cron format"]
    pub schedule: String,
    #[doc = "StartingDeadlineSeconds is the K8s-style deadline that will limit the time a CronWorkflow will be run after its original scheduled time if it is missed."]
    #[serde(
        rename = "startingDeadlineSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub starting_deadline_seconds: Option<i64>,
    #[doc = "SuccessfulJobsHistoryLimit is the number of successful jobs to be kept at a time"]
    #[serde(
        rename = "successfulJobsHistoryLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub successful_jobs_history_limit: Option<i64>,
    #[doc = "Suspend is a flag that will stop new CronWorkflows from running if set to true"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    #[doc = "Timezone is the timezone against which the cron schedule will be calculated, e.g. \"Asia/Tokyo\". Default is machine's local time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[doc = "WorkflowMetadata contains some metadata of the workflow to be run"]
    #[serde(
        rename = "workflowMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_metadata: Option<IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
    #[doc = "WorkflowSpec is the spec of the workflow to be run"]
    #[serde(rename = "workflowSpec")]
    pub workflow_spec: IoArgoprojWorkflowV1alpha1WorkflowSpec,
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflowSpec>
    for IoArgoprojWorkflowV1alpha1CronWorkflowSpec
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflowSpec) -> Self {
        value.clone()
    }
}
#[doc = "CronWorkflowStatus is the status of a CronWorkflow"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflowStatus {
    #[doc = "Active is a list of active workflows stemming from this CronWorkflow"]
    pub active: Vec<IoK8sApiCoreV1ObjectReference>,
    #[doc = "Conditions is a list of conditions the CronWorkflow may have"]
    pub conditions: Vec<IoArgoprojWorkflowV1alpha1Condition>,
    #[doc = "LastScheduleTime is the last time the CronWorkflow was scheduled"]
    #[serde(rename = "lastScheduledTime")]
    pub last_scheduled_time: IoK8sApimachineryPkgApisMetaV1Time,
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflowStatus>
    for IoArgoprojWorkflowV1alpha1CronWorkflowStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflowStatus) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1CronWorkflowSuspendRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1CronWorkflowSuspendRequest>
    for IoArgoprojWorkflowV1alpha1CronWorkflowSuspendRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1CronWorkflowSuspendRequest) -> Self {
        value.clone()
    }
}
pub use IoArgoprojWorkflowV1alpha1DagTaskBuilder as DagTaskBuilder;
pub use IoArgoprojWorkflowV1alpha1DagTaskBuilderError as DagTaskBuilderError;

#[doc = "DAGTask represents a node in the graph during DAG execution"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1DagTask {
    #[doc = "Arguments are the parameter and artifact arguments to the template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<IoArgoprojWorkflowV1alpha1Arguments>,
    #[doc = "ContinueOn makes argo to proceed with the following step even if this step fails. Errors and Failed states can be specified"]
    #[serde(
        rename = "continueOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on: Option<IoArgoprojWorkflowV1alpha1ContinueOn>,
    #[doc = "Dependencies are name of other targets which this depends on"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[doc = "Depends are name of other targets which this depends on"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depends: Option<String>,
    #[doc = "Hooks hold the lifecycle hook which is invoked at lifecycle of task, irrespective of the success, failure, or error status of the primary task"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub hooks: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1LifecycleHook>,
    #[doc = "Inline is the template. Template must be empty if this is declared (and vice-versa)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<IoArgoprojWorkflowV1alpha1Template>,
    #[doc = "Name is the name of the target"]
    pub name: String,
    #[doc = "OnExit is a template reference which is invoked at the end of the template, irrespective of the success, failure, or error of the primary template. DEPRECATED: Use Hooks[exit].Template instead."]
    #[serde(rename = "onExit", default, skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<String>,
    #[doc = "Name of template to execute"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "TemplateRef is the reference to the template resource to execute."]
    #[serde(
        rename = "templateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_ref: Option<IoArgoprojWorkflowV1alpha1TemplateRef>,
    #[doc = "When is an expression in which the task should conditionally execute"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    #[doc = "WithItems expands a task into multiple parallel tasks from the items in the list"]
    #[serde(rename = "withItems", default, skip_serializing_if = "Vec::is_empty")]
    pub with_items: Vec<IoArgoprojWorkflowV1alpha1Item>,
    #[doc = "WithParam expands a task into multiple parallel tasks from the value in the parameter, which is expected to be a JSON list."]
    #[serde(rename = "withParam", default, skip_serializing_if = "Option::is_none")]
    pub with_param: Option<String>,
    #[doc = "WithSequence expands a task into a numeric sequence"]
    #[serde(
        rename = "withSequence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub with_sequence: Option<IoArgoprojWorkflowV1alpha1Sequence>,
}
impl From<&IoArgoprojWorkflowV1alpha1DagTask> for IoArgoprojWorkflowV1alpha1DagTask {
    fn from(value: &IoArgoprojWorkflowV1alpha1DagTask) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1DagTemplateBuilder as DagTemplateBuilder;
pub use IoArgoprojWorkflowV1alpha1DagTemplateBuilderError as DagTemplateBuilderError;

#[doc = "DAGTemplate is a template subtype for directed acyclic graph templates"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1DagTemplate {
    #[doc = "This flag is for DAG logic. The DAG logic has a built-in \"fail fast\" feature to stop scheduling new steps, as soon as it detects that one of the DAG nodes is failed. Then it waits until all DAG nodes are completed before failing the DAG itself. The FailFast flag default is true,  if set to false, it will allow a DAG to run all branches of the DAG to completion (either success or failure), regardless of the failed outcomes of branches in the DAG. More info and example about this feature at https://github.com/argoproj/argo-workflows/issues/1442"]
    #[serde(rename = "failFast", default, skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    #[doc = "Target are one or more names of targets to execute in a DAG"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Tasks are a list of DAG tasks"]
    pub tasks: Vec<IoArgoprojWorkflowV1alpha1DagTask>,
}
impl From<&IoArgoprojWorkflowV1alpha1DagTemplate> for IoArgoprojWorkflowV1alpha1DagTemplate {
    fn from(value: &IoArgoprojWorkflowV1alpha1DagTemplate) -> Self {
        value.clone()
    }
}
#[doc = "Data is a data template"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Data {
    #[doc = "Source sources external data into a data template"]
    pub source: IoArgoprojWorkflowV1alpha1DataSource,
    #[doc = "Transformation applies a set of transformations"]
    pub transformation: Vec<IoArgoprojWorkflowV1alpha1TransformationStep>,
}
impl From<&IoArgoprojWorkflowV1alpha1Data> for IoArgoprojWorkflowV1alpha1Data {
    fn from(value: &IoArgoprojWorkflowV1alpha1Data) -> Self {
        value.clone()
    }
}
#[doc = "DataSource sources external data into a data template"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1DataSource {
    #[doc = "ArtifactPaths is a data transformation that collects a list of artifact paths"]
    #[serde(
        rename = "artifactPaths",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_paths: Option<IoArgoprojWorkflowV1alpha1ArtifactPaths>,
}
impl From<&IoArgoprojWorkflowV1alpha1DataSource> for IoArgoprojWorkflowV1alpha1DataSource {
    fn from(value: &IoArgoprojWorkflowV1alpha1DataSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Event {
    #[doc = "Selector (https://github.com/antonmedv/expr) that we must must match the io.argoproj.workflow.v1alpha1. E.g. `payload.message == \"test\"`"]
    pub selector: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Event> for IoArgoprojWorkflowV1alpha1Event {
    fn from(value: &IoArgoprojWorkflowV1alpha1Event) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1EventResponse(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1EventResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1EventResponse> for serde_json::Map<String, serde_json::Value> {
    fn from(value: IoArgoprojWorkflowV1alpha1EventResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1EventResponse> for IoArgoprojWorkflowV1alpha1EventResponse {
    fn from(value: &IoArgoprojWorkflowV1alpha1EventResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for IoArgoprojWorkflowV1alpha1EventResponse {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "ExecutorConfig holds configurations of an executor container."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ExecutorConfig {
    #[doc = "ServiceAccountName specifies the service account name of the executor container."]
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ExecutorConfig> for IoArgoprojWorkflowV1alpha1ExecutorConfig {
    fn from(value: &IoArgoprojWorkflowV1alpha1ExecutorConfig) -> Self {
        value.clone()
    }
}
#[doc = "Gauge is a Gauge prometheus metric"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Gauge {
    #[doc = "Operation defines the operation to apply with value and the metrics' current value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Realtime emits this metric in real time if applicable"]
    pub realtime: bool,
    #[doc = "Value is the value to be used in the operation with the metric's current value. If no operation is set, value is the value of the metric"]
    pub value: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Gauge> for IoArgoprojWorkflowV1alpha1Gauge {
    fn from(value: &IoArgoprojWorkflowV1alpha1Gauge) -> Self {
        value.clone()
    }
}
#[doc = "GCSArtifact is the location of a GCS artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1GcsArtifact {
    #[doc = "Bucket is the name of the bucket"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[doc = "Key is the path in the bucket where the artifact resides"]
    pub key: String,
    #[doc = "ServiceAccountKeySecret is the secret selector to the bucket's service account key"]
    #[serde(
        rename = "serviceAccountKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1GcsArtifact> for IoArgoprojWorkflowV1alpha1GcsArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1GcsArtifact) -> Self {
        value.clone()
    }
}
#[doc = "GCSArtifactRepository defines the controller configuration for a GCS artifact repository"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1GcsArtifactRepository {
    #[doc = "Bucket is the name of the bucket"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[doc = "KeyFormat defines the format of how to store keys and can reference workflow variables."]
    #[serde(rename = "keyFormat", default, skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[doc = "ServiceAccountKeySecret is the secret selector to the bucket's service account key"]
    #[serde(
        rename = "serviceAccountKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1GcsArtifactRepository>
    for IoArgoprojWorkflowV1alpha1GcsArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1GcsArtifactRepository) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1GetUserInfoResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(
        rename = "emailVerified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
    #[serde(
        rename = "serviceAccountNamespace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1GetUserInfoResponse>
    for IoArgoprojWorkflowV1alpha1GetUserInfoResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1GetUserInfoResponse) -> Self {
        value.clone()
    }
}
#[doc = "GitArtifact is the location of an git artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1GitArtifact {
    #[doc = "Branch is the branch to fetch when `SingleBranch` is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "Depth specifies clones/fetches should be shallow and include the given number of commits from the branch tip"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    #[doc = "DisableSubmodules disables submodules during git clone"]
    #[serde(
        rename = "disableSubmodules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_submodules: Option<bool>,
    #[doc = "Fetch specifies a number of refs that should be fetched before checkout"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fetch: Vec<String>,
    #[doc = "InsecureIgnoreHostKey disables SSH strict host key checking during git clone"]
    #[serde(
        rename = "insecureIgnoreHostKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insecure_ignore_host_key: Option<bool>,
    #[doc = "PasswordSecret is the secret selector to the repository password"]
    #[serde(
        rename = "passwordSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Repo is the git repository"]
    pub repo: String,
    #[doc = "Revision is the git commit, tag, branch to checkout"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[doc = "SingleBranch enables single branch clone, using the `branch` parameter"]
    #[serde(
        rename = "singleBranch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub single_branch: Option<bool>,
    #[doc = "SSHPrivateKeySecret is the secret selector to the repository ssh private key"]
    #[serde(
        rename = "sshPrivateKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_private_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "UsernameSecret is the secret selector to the repository username"]
    #[serde(
        rename = "usernameSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1GitArtifact> for IoArgoprojWorkflowV1alpha1GitArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1GitArtifact) -> Self {
        value.clone()
    }
}
#[doc = "HDFSArtifact is the location of an HDFS artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HdfsArtifact {
    #[doc = "Addresses is accessible addresses of HDFS name nodes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<String>,
    #[doc = "Force copies a file forcibly even if it exists"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[doc = "HDFSUser is the user to access HDFS file system. It is ignored if either ccache or keytab is used."]
    #[serde(rename = "hdfsUser", default, skip_serializing_if = "Option::is_none")]
    pub hdfs_user: Option<String>,
    #[doc = "KrbCCacheSecret is the secret selector for Kerberos ccache Either ccache or keytab can be set to use Kerberos."]
    #[serde(
        rename = "krbCCacheSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_c_cache_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "KrbConfig is the configmap selector for Kerberos config as string It must be set if either ccache or keytab is used."]
    #[serde(
        rename = "krbConfigConfigMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_config_config_map: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[doc = "KrbKeytabSecret is the secret selector for Kerberos keytab Either ccache or keytab can be set to use Kerberos."]
    #[serde(
        rename = "krbKeytabSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_keytab_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "KrbRealm is the Kerberos realm used with Kerberos keytab It must be set if keytab is used."]
    #[serde(rename = "krbRealm", default, skip_serializing_if = "Option::is_none")]
    pub krb_realm: Option<String>,
    #[doc = "KrbServicePrincipalName is the principal name of Kerberos service It must be set if either ccache or keytab is used."]
    #[serde(
        rename = "krbServicePrincipalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_service_principal_name: Option<String>,
    #[doc = "KrbUsername is the Kerberos username used with Kerberos keytab It must be set if keytab is used."]
    #[serde(
        rename = "krbUsername",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_username: Option<String>,
    #[doc = "Path is a file path in HDFS"]
    pub path: String,
}
impl From<&IoArgoprojWorkflowV1alpha1HdfsArtifact> for IoArgoprojWorkflowV1alpha1HdfsArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1HdfsArtifact) -> Self {
        value.clone()
    }
}
#[doc = "HDFSArtifactRepository defines the controller configuration for an HDFS artifact repository"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HdfsArtifactRepository {
    #[doc = "Addresses is accessible addresses of HDFS name nodes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<String>,
    #[doc = "Force copies a file forcibly even if it exists"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[doc = "HDFSUser is the user to access HDFS file system. It is ignored if either ccache or keytab is used."]
    #[serde(rename = "hdfsUser", default, skip_serializing_if = "Option::is_none")]
    pub hdfs_user: Option<String>,
    #[doc = "KrbCCacheSecret is the secret selector for Kerberos ccache Either ccache or keytab can be set to use Kerberos."]
    #[serde(
        rename = "krbCCacheSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_c_cache_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "KrbConfig is the configmap selector for Kerberos config as string It must be set if either ccache or keytab is used."]
    #[serde(
        rename = "krbConfigConfigMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_config_config_map: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[doc = "KrbKeytabSecret is the secret selector for Kerberos keytab Either ccache or keytab can be set to use Kerberos."]
    #[serde(
        rename = "krbKeytabSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_keytab_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "KrbRealm is the Kerberos realm used with Kerberos keytab It must be set if keytab is used."]
    #[serde(rename = "krbRealm", default, skip_serializing_if = "Option::is_none")]
    pub krb_realm: Option<String>,
    #[doc = "KrbServicePrincipalName is the principal name of Kerberos service It must be set if either ccache or keytab is used."]
    #[serde(
        rename = "krbServicePrincipalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_service_principal_name: Option<String>,
    #[doc = "KrbUsername is the Kerberos username used with Kerberos keytab It must be set if keytab is used."]
    #[serde(
        rename = "krbUsername",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krb_username: Option<String>,
    #[doc = "PathFormat is defines the format of path to store a file. Can reference workflow variables"]
    #[serde(
        rename = "pathFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_format: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1HdfsArtifactRepository>
    for IoArgoprojWorkflowV1alpha1HdfsArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1HdfsArtifactRepository) -> Self {
        value.clone()
    }
}
#[doc = "Header indicate a key-value request header to be used when fetching artifacts over HTTP"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Header {
    #[doc = "Name is the header name"]
    pub name: String,
    #[doc = "Value is the literal value to use for the header"]
    pub value: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Header> for IoArgoprojWorkflowV1alpha1Header {
    fn from(value: &IoArgoprojWorkflowV1alpha1Header) -> Self {
        value.clone()
    }
}
#[doc = "Histogram is a Histogram prometheus metric"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Histogram {
    #[doc = "Buckets is a list of bucket divisors for the histogram"]
    pub buckets: Vec<IoArgoprojWorkflowV1alpha1Amount>,
    #[doc = "Value is the value of the metric"]
    pub value: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Histogram> for IoArgoprojWorkflowV1alpha1Histogram {
    fn from(value: &IoArgoprojWorkflowV1alpha1Histogram) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Http {
    #[doc = "Body is content of the HTTP Request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "BodyFrom is  content of the HTTP Request as Bytes"]
    #[serde(rename = "bodyFrom", default, skip_serializing_if = "Option::is_none")]
    pub body_from: Option<IoArgoprojWorkflowV1alpha1HttpBodySource>,
    #[doc = "Headers are an optional list of headers to send with HTTP requests"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<IoArgoprojWorkflowV1alpha1HttpHeader>,
    #[doc = "InsecureSkipVerify is a bool when if set to true will skip TLS verification for the HTTP client"]
    #[serde(
        rename = "insecureSkipVerify",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insecure_skip_verify: Option<bool>,
    #[doc = "Method is HTTP methods for HTTP Request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "SuccessCondition is an expression if evaluated to true is considered successful"]
    #[serde(
        rename = "successCondition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub success_condition: Option<String>,
    #[doc = "TimeoutSeconds is request timeout for HTTP Request. Default is 30 seconds"]
    #[serde(
        rename = "timeoutSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_seconds: Option<i64>,
    #[doc = "URL of the HTTP Request"]
    pub url: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Http> for IoArgoprojWorkflowV1alpha1Http {
    fn from(value: &IoArgoprojWorkflowV1alpha1Http) -> Self {
        value.clone()
    }
}
#[doc = "HTTPArtifact allows a file served on HTTP to be placed as an input artifact in a container"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HttpArtifact {
    #[doc = "Auth contains information for client authentication"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<IoArgoprojWorkflowV1alpha1HttpAuth>,
    #[doc = "Headers are an optional list of headers to send with HTTP requests for artifacts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<IoArgoprojWorkflowV1alpha1Header>,
    #[doc = "URL of the artifact"]
    pub url: String,
}
impl From<&IoArgoprojWorkflowV1alpha1HttpArtifact> for IoArgoprojWorkflowV1alpha1HttpArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1HttpArtifact) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HttpAuth {
    #[serde(rename = "basicAuth", default, skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<IoArgoprojWorkflowV1alpha1BasicAuth>,
    #[serde(
        rename = "clientCert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_cert: Option<IoArgoprojWorkflowV1alpha1ClientCertAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<IoArgoprojWorkflowV1alpha1OAuth2Auth>,
}
impl From<&IoArgoprojWorkflowV1alpha1HttpAuth> for IoArgoprojWorkflowV1alpha1HttpAuth {
    fn from(value: &IoArgoprojWorkflowV1alpha1HttpAuth) -> Self {
        value.clone()
    }
}
#[doc = "HTTPBodySource contains the source of the HTTP body."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HttpBodySource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1HttpBodySource> for IoArgoprojWorkflowV1alpha1HttpBodySource {
    fn from(value: &IoArgoprojWorkflowV1alpha1HttpBodySource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HttpHeader {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "valueFrom", default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<IoArgoprojWorkflowV1alpha1HttpHeaderSource>,
}
impl From<&IoArgoprojWorkflowV1alpha1HttpHeader> for IoArgoprojWorkflowV1alpha1HttpHeader {
    fn from(value: &IoArgoprojWorkflowV1alpha1HttpHeader) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1HttpHeaderSource {
    #[serde(
        rename = "secretKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_ref: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1HttpHeaderSource>
    for IoArgoprojWorkflowV1alpha1HttpHeaderSource
{
    fn from(value: &IoArgoprojWorkflowV1alpha1HttpHeaderSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1InfoResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<IoArgoprojWorkflowV1alpha1Column>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<IoArgoprojWorkflowV1alpha1Link>,
    #[serde(
        rename = "managedNamespace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub modals: std::collections::HashMap<String, bool>,
    #[serde(rename = "navColor", default, skip_serializing_if = "Option::is_none")]
    pub nav_color: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1InfoResponse> for IoArgoprojWorkflowV1alpha1InfoResponse {
    fn from(value: &IoArgoprojWorkflowV1alpha1InfoResponse) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1InputsBuilder as InputsBuilder;
pub use IoArgoprojWorkflowV1alpha1InputsBuilderError as InputsBuilderError;
#[doc = "Inputs are the mechanism for passing parameters, artifacts, volumes from one template to another"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
pub struct IoArgoprojWorkflowV1alpha1Inputs {
    #[builder(default)]
    #[doc = "Artifact are a list of artifacts passed as inputs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<IoArgoprojWorkflowV1alpha1Artifact>,
    #[doc = "Parameters are a list of parameters passed as inputs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojWorkflowV1alpha1Parameter>,
}
impl From<&IoArgoprojWorkflowV1alpha1Inputs> for IoArgoprojWorkflowV1alpha1Inputs {
    fn from(value: &IoArgoprojWorkflowV1alpha1Inputs) -> Self {
        value.clone()
    }
}
#[doc = "Item expands a single workflow step into multiple parallel steps The value of Item can be a map, string, bool, or number"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Item(pub serde_json::Value);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1Item {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1Item> for serde_json::Value {
    fn from(value: IoArgoprojWorkflowV1alpha1Item) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1Item> for IoArgoprojWorkflowV1alpha1Item {
    fn from(value: &IoArgoprojWorkflowV1alpha1Item) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for IoArgoprojWorkflowV1alpha1Item {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = "LabelKeys is list of keys"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1LabelKeys {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1LabelKeys> for IoArgoprojWorkflowV1alpha1LabelKeys {
    fn from(value: &IoArgoprojWorkflowV1alpha1LabelKeys) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1LabelValueFrom {
    pub expression: String,
}
impl From<&IoArgoprojWorkflowV1alpha1LabelValueFrom> for IoArgoprojWorkflowV1alpha1LabelValueFrom {
    fn from(value: &IoArgoprojWorkflowV1alpha1LabelValueFrom) -> Self {
        value.clone()
    }
}
#[doc = "Labels is list of workflow labels"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1LabelValues {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1LabelValues> for IoArgoprojWorkflowV1alpha1LabelValues {
    fn from(value: &IoArgoprojWorkflowV1alpha1LabelValues) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1LifecycleHook {
    #[doc = "Arguments hold arguments to the template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<IoArgoprojWorkflowV1alpha1Arguments>,
    #[doc = "Expression is a condition expression for when a node will be retried. If it evaluates to false, the node will not be retried and the retry strategy will be ignored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "Template is the name of the template to execute by the hook"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "TemplateRef is the reference to the template resource to execute by the hook"]
    #[serde(
        rename = "templateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_ref: Option<IoArgoprojWorkflowV1alpha1TemplateRef>,
}
impl From<&IoArgoprojWorkflowV1alpha1LifecycleHook> for IoArgoprojWorkflowV1alpha1LifecycleHook {
    fn from(value: &IoArgoprojWorkflowV1alpha1LifecycleHook) -> Self {
        value.clone()
    }
}
#[doc = "A link to another app."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Link {
    #[doc = "The name of the link, E.g. \"Workflow Logs\" or \"Pod Logs\""]
    pub name: String,
    #[doc = "\"workflow\", \"pod\", \"pod-logs\", \"event-source-logs\", \"sensor-logs\", \"workflow-list\" or \"chat\""]
    pub scope: String,
    #[doc = "The URL. Can contain \"${metadata.namespace}\", \"${metadata.name}\", \"${status.startedAt}\", \"${status.finishedAt}\" or any other element in workflow yaml, e.g. \"${io.argoproj.workflow.v1alpha1.metadata.annotations.userDefinedKey}\""]
    pub url: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Link> for IoArgoprojWorkflowV1alpha1Link {
    fn from(value: &IoArgoprojWorkflowV1alpha1Link) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1LintCronWorkflowRequest {
    #[serde(
        rename = "cronWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cron_workflow: Option<IoArgoprojWorkflowV1alpha1CronWorkflow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1LintCronWorkflowRequest>
    for IoArgoprojWorkflowV1alpha1LintCronWorkflowRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1LintCronWorkflowRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1LogEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "podName", default, skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1LogEntry> for IoArgoprojWorkflowV1alpha1LogEntry {
    fn from(value: &IoArgoprojWorkflowV1alpha1LogEntry) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ManifestFrom {
    #[doc = "Artifact contains the artifact to use"]
    pub artifact: IoArgoprojWorkflowV1alpha1Artifact,
}
impl From<&IoArgoprojWorkflowV1alpha1ManifestFrom> for IoArgoprojWorkflowV1alpha1ManifestFrom {
    fn from(value: &IoArgoprojWorkflowV1alpha1ManifestFrom) -> Self {
        value.clone()
    }
}
#[doc = "MemoizationStatus is the status of this memoized node"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1MemoizationStatus {
    #[doc = "Cache is the name of the cache that was used"]
    #[serde(rename = "cacheName")]
    pub cache_name: String,
    #[doc = "Hit indicates whether this node was created from a cache entry"]
    pub hit: bool,
    #[doc = "Key is the name of the key used for this node's cache"]
    pub key: String,
}
impl From<&IoArgoprojWorkflowV1alpha1MemoizationStatus>
    for IoArgoprojWorkflowV1alpha1MemoizationStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1MemoizationStatus) -> Self {
        value.clone()
    }
}
#[doc = "Memoization enables caching for the Outputs of the template"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Memoize {
    #[doc = "Cache sets and configures the kind of cache"]
    pub cache: IoArgoprojWorkflowV1alpha1Cache,
    #[doc = "Key is the key to use as the caching key"]
    pub key: String,
    #[doc = "MaxAge is the maximum age (e.g. \"180s\", \"24h\") of an entry that is still considered valid. If an entry is older than the MaxAge, it will be ignored."]
    #[serde(rename = "maxAge")]
    pub max_age: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Memoize> for IoArgoprojWorkflowV1alpha1Memoize {
    fn from(value: &IoArgoprojWorkflowV1alpha1Memoize) -> Self {
        value.clone()
    }
}
#[doc = "Pod metdata"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Metadata {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub annotations: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Metadata> for IoArgoprojWorkflowV1alpha1Metadata {
    fn from(value: &IoArgoprojWorkflowV1alpha1Metadata) -> Self {
        value.clone()
    }
}
#[doc = "MetricLabel is a single label for a prometheus metric"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1MetricLabel {
    pub key: String,
    pub value: String,
}
impl From<&IoArgoprojWorkflowV1alpha1MetricLabel> for IoArgoprojWorkflowV1alpha1MetricLabel {
    fn from(value: &IoArgoprojWorkflowV1alpha1MetricLabel) -> Self {
        value.clone()
    }
}
#[doc = "Metrics are a list of metrics emitted from a Workflow/Template"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Metrics {
    #[doc = "Prometheus is a list of prometheus metrics to be emitted"]
    pub prometheus: Vec<IoArgoprojWorkflowV1alpha1Prometheus>,
}
impl From<&IoArgoprojWorkflowV1alpha1Metrics> for IoArgoprojWorkflowV1alpha1Metrics {
    fn from(value: &IoArgoprojWorkflowV1alpha1Metrics) -> Self {
        value.clone()
    }
}
#[doc = "Mutex holds Mutex configuration"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Mutex {
    #[doc = "name of the mutex"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Namespace is the namespace of the mutex, default: [namespace of workflow]"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Mutex> for IoArgoprojWorkflowV1alpha1Mutex {
    fn from(value: &IoArgoprojWorkflowV1alpha1Mutex) -> Self {
        value.clone()
    }
}
#[doc = "MutexHolding describes the mutex and the object which is holding it."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1MutexHolding {
    #[doc = "Holder is a reference to the object which holds the Mutex. Holding Scenario:\n  1. Current workflow's NodeID which is holding the lock.\n     e.g: ${NodeID}\nWaiting Scenario:\n  1. Current workflow or other workflow NodeID which is holding the lock.\n     e.g: ${WorkflowName}/${NodeID}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[doc = "Reference for the mutex e.g: ${namespace}/mutex/${mutexName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutex: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1MutexHolding> for IoArgoprojWorkflowV1alpha1MutexHolding {
    fn from(value: &IoArgoprojWorkflowV1alpha1MutexHolding) -> Self {
        value.clone()
    }
}
#[doc = "MutexStatus contains which objects hold  mutex locks, and which objects this workflow is waiting on to release locks."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1MutexStatus {
    #[doc = "Holding is a list of mutexes and their respective objects that are held by mutex lock for this io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding: Vec<IoArgoprojWorkflowV1alpha1MutexHolding>,
    #[doc = "Waiting is a list of mutexes and their respective objects this workflow is waiting for."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub waiting: Vec<IoArgoprojWorkflowV1alpha1MutexHolding>,
}
impl From<&IoArgoprojWorkflowV1alpha1MutexStatus> for IoArgoprojWorkflowV1alpha1MutexStatus {
    fn from(value: &IoArgoprojWorkflowV1alpha1MutexStatus) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1NodeResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IoArgoprojWorkflowV1alpha1Outputs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1NodeResult> for IoArgoprojWorkflowV1alpha1NodeResult {
    fn from(value: &IoArgoprojWorkflowV1alpha1NodeResult) -> Self {
        value.clone()
    }
}
#[doc = "NodeStatus contains status information about an individual node in the workflow"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1NodeStatus {
    #[doc = "BoundaryID indicates the node ID of the associated template root node in which this node belongs to"]
    #[serde(
        rename = "boundaryID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub boundary_id: Option<String>,
    #[doc = "Children is a list of child node IDs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,
    #[doc = "Daemoned tracks whether or not this node was daemoned and need to be terminated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemoned: Option<bool>,
    #[doc = "DisplayName is a human readable representation of the node. Unique within a template boundary"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "EstimatedDuration in seconds."]
    #[serde(
        rename = "estimatedDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_duration: Option<i64>,
    #[doc = "Time at which this node completed"]
    #[serde(
        rename = "finishedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub finished_at: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "HostNodeName name of the Kubernetes node on which the Pod is running, if applicable"]
    #[serde(
        rename = "hostNodeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_node_name: Option<String>,
    #[doc = "ID is a unique identifier of a node within the worklow It is implemented as a hash of the node name, which makes the ID deterministic"]
    pub id: String,
    #[doc = "Inputs captures input parameter values and artifact locations supplied to this template invocation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<IoArgoprojWorkflowV1alpha1Inputs>,
    #[doc = "MemoizationStatus holds information about cached nodes"]
    #[serde(
        rename = "memoizationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub memoization_status: Option<IoArgoprojWorkflowV1alpha1MemoizationStatus>,
    #[doc = "A human readable message indicating details about why the node is in this condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Name is unique name in the node tree used to generate the node ID"]
    pub name: String,
    #[doc = "OutboundNodes tracks the node IDs which are considered \"outbound\" nodes to a template invocation. For every invocation of a template, there are nodes which we considered as \"outbound\". Essentially, these are last nodes in the execution sequence to run, before the template is considered completed. These nodes are then connected as parents to a following step.\n\nIn the case of single pod steps (i.e. container, script, resource templates), this list will be nil since the pod itself is already considered the \"outbound\" node. In the case of DAGs, outbound nodes are the \"target\" tasks (tasks with no children). In the case of steps, outbound nodes are all the containers involved in the last step group. NOTE: since templates are composable, the list of outbound nodes are carried upwards when a DAG/steps template invokes another DAG/steps template. In other words, the outbound nodes of a template, will be a superset of the outbound nodes of its last children."]
    #[serde(
        rename = "outboundNodes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_nodes: Vec<String>,
    #[doc = "Outputs captures output parameter values and artifact locations produced by this template invocation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IoArgoprojWorkflowV1alpha1Outputs>,
    #[doc = "Phase a simple, high-level summary of where the node is in its lifecycle. Can be used as a state machine. Will be one of these values \"Pending\", \"Running\" before the node is completed, or \"Succeeded\", \"Skipped\", \"Failed\", \"Error\", or \"Omitted\" as a final state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[doc = "PodIP captures the IP of the pod for daemoned steps"]
    #[serde(rename = "podIP", default, skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,
    #[doc = "Progress to completion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[doc = "ResourcesDuration is indicative, but not accurate, resource duration. This is populated when the nodes completes."]
    #[serde(
        rename = "resourcesDuration",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub resources_duration: std::collections::HashMap<String, i64>,
    #[doc = "Time at which this node started"]
    #[serde(rename = "startedAt", default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "SynchronizationStatus is the synchronization status of the node"]
    #[serde(
        rename = "synchronizationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub synchronization_status: Option<IoArgoprojWorkflowV1alpha1NodeSynchronizationStatus>,
    #[doc = "TemplateName is the template name which this node corresponds to. Not applicable to virtual nodes (e.g. Retry, StepGroup)"]
    #[serde(
        rename = "templateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_name: Option<String>,
    #[doc = "TemplateRef is the reference to the template resource which this node corresponds to. Not applicable to virtual nodes (e.g. Retry, StepGroup)"]
    #[serde(
        rename = "templateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_ref: Option<IoArgoprojWorkflowV1alpha1TemplateRef>,
    #[doc = "TemplateScope is the template scope in which the template of this node was retrieved."]
    #[serde(
        rename = "templateScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_scope: Option<String>,
    #[doc = "Type indicates type of node"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&IoArgoprojWorkflowV1alpha1NodeStatus> for IoArgoprojWorkflowV1alpha1NodeStatus {
    fn from(value: &IoArgoprojWorkflowV1alpha1NodeStatus) -> Self {
        value.clone()
    }
}
#[doc = "NodeSynchronizationStatus stores the status of a node"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1NodeSynchronizationStatus {
    #[doc = "Waiting is the name of the lock that this node is waiting for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1NodeSynchronizationStatus>
    for IoArgoprojWorkflowV1alpha1NodeSynchronizationStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1NodeSynchronizationStatus) -> Self {
        value.clone()
    }
}
#[doc = "NoneStrategy indicates to skip tar process and upload the files or directory tree as independent files. Note that if the artifact is a directory, the artifact driver must support the ability to save/load the directory appropriately."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1NoneStrategy(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1NoneStrategy {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1NoneStrategy> for serde_json::Map<String, serde_json::Value> {
    fn from(value: IoArgoprojWorkflowV1alpha1NoneStrategy) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1NoneStrategy> for IoArgoprojWorkflowV1alpha1NoneStrategy {
    fn from(value: &IoArgoprojWorkflowV1alpha1NoneStrategy) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for IoArgoprojWorkflowV1alpha1NoneStrategy {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "OAuth2Auth holds all information for client authentication via OAuth2 tokens"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1OAuth2Auth {
    #[serde(
        rename = "clientIDSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_id_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "clientSecretSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_secret_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[serde(
        rename = "endpointParams",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_params: Vec<IoArgoprojWorkflowV1alpha1OAuth2EndpointParam>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(
        rename = "tokenURLSecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub token_url_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1OAuth2Auth> for IoArgoprojWorkflowV1alpha1OAuth2Auth {
    fn from(value: &IoArgoprojWorkflowV1alpha1OAuth2Auth) -> Self {
        value.clone()
    }
}
#[doc = "EndpointParam is for requesting optional fields that should be sent in the oauth request"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1OAuth2EndpointParam {
    #[doc = "Name is the header name"]
    pub key: String,
    #[doc = "Value is the literal value to use for the header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1OAuth2EndpointParam>
    for IoArgoprojWorkflowV1alpha1OAuth2EndpointParam
{
    fn from(value: &IoArgoprojWorkflowV1alpha1OAuth2EndpointParam) -> Self {
        value.clone()
    }
}
#[doc = "OSSArtifact is the location of an Alibaba Cloud OSS artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1OssArtifact {
    #[doc = "AccessKeySecret is the secret selector to the bucket's access key"]
    #[serde(
        rename = "accessKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Bucket is the name of the bucket"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[doc = "CreateBucketIfNotPresent tells the driver to attempt to create the OSS bucket for output artifacts, if it doesn't exist"]
    #[serde(
        rename = "createBucketIfNotPresent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_bucket_if_not_present: Option<bool>,
    #[doc = "Endpoint is the hostname of the bucket endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Key is the path in the bucket where the artifact resides"]
    pub key: String,
    #[doc = "LifecycleRule specifies how to manage bucket's lifecycle"]
    #[serde(
        rename = "lifecycleRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lifecycle_rule: Option<IoArgoprojWorkflowV1alpha1OssLifecycleRule>,
    #[doc = "SecretKeySecret is the secret selector to the bucket's secret key"]
    #[serde(
        rename = "secretKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "SecurityToken is the user's temporary security token. For more details, check out: https://www.alibabacloud.com/help/doc-detail/100624.htm"]
    #[serde(
        rename = "securityToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_token: Option<String>,
    #[doc = "UseSDKCreds tells the driver to figure out credentials based on sdk defaults."]
    #[serde(
        rename = "useSDKCreds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sdk_creds: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1OssArtifact> for IoArgoprojWorkflowV1alpha1OssArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1OssArtifact) -> Self {
        value.clone()
    }
}
#[doc = "OSSArtifactRepository defines the controller configuration for an OSS artifact repository"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1OssArtifactRepository {
    #[doc = "AccessKeySecret is the secret selector to the bucket's access key"]
    #[serde(
        rename = "accessKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Bucket is the name of the bucket"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[doc = "CreateBucketIfNotPresent tells the driver to attempt to create the OSS bucket for output artifacts, if it doesn't exist"]
    #[serde(
        rename = "createBucketIfNotPresent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_bucket_if_not_present: Option<bool>,
    #[doc = "Endpoint is the hostname of the bucket endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "KeyFormat defines the format of how to store keys and can reference workflow variables."]
    #[serde(rename = "keyFormat", default, skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[doc = "LifecycleRule specifies how to manage bucket's lifecycle"]
    #[serde(
        rename = "lifecycleRule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lifecycle_rule: Option<IoArgoprojWorkflowV1alpha1OssLifecycleRule>,
    #[doc = "SecretKeySecret is the secret selector to the bucket's secret key"]
    #[serde(
        rename = "secretKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "SecurityToken is the user's temporary security token. For more details, check out: https://www.alibabacloud.com/help/doc-detail/100624.htm"]
    #[serde(
        rename = "securityToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_token: Option<String>,
    #[doc = "UseSDKCreds tells the driver to figure out credentials based on sdk defaults."]
    #[serde(
        rename = "useSDKCreds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sdk_creds: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1OssArtifactRepository>
    for IoArgoprojWorkflowV1alpha1OssArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1OssArtifactRepository) -> Self {
        value.clone()
    }
}
#[doc = "OSSLifecycleRule specifies how to manage bucket's lifecycle"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1OssLifecycleRule {
    #[doc = "MarkDeletionAfterDays is the number of days before we delete objects in the bucket"]
    #[serde(
        rename = "markDeletionAfterDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_deletion_after_days: Option<i64>,
    #[doc = "MarkInfrequentAccessAfterDays is the number of days before we convert the objects in the bucket to Infrequent Access (IA) storage type"]
    #[serde(
        rename = "markInfrequentAccessAfterDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_infrequent_access_after_days: Option<i64>,
}
impl From<&IoArgoprojWorkflowV1alpha1OssLifecycleRule>
    for IoArgoprojWorkflowV1alpha1OssLifecycleRule
{
    fn from(value: &IoArgoprojWorkflowV1alpha1OssLifecycleRule) -> Self {
        value.clone()
    }
}
#[doc = "Outputs hold parameters, artifacts, and results from a step"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Outputs {
    #[doc = "Artifacts holds the list of output artifacts produced by a step"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<IoArgoprojWorkflowV1alpha1Artifact>,
    #[doc = "ExitCode holds the exit code of a script template"]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    #[doc = "Parameters holds the list of output parameters produced by a step"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<IoArgoprojWorkflowV1alpha1Parameter>,
    #[doc = "Result holds the result (stdout) of a script template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Outputs> for IoArgoprojWorkflowV1alpha1Outputs {
    fn from(value: &IoArgoprojWorkflowV1alpha1Outputs) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ParallelSteps(pub Vec<IoArgoprojWorkflowV1alpha1WorkflowStep>);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1ParallelSteps {
    type Target = Vec<IoArgoprojWorkflowV1alpha1WorkflowStep>;
    fn deref(&self) -> &Vec<IoArgoprojWorkflowV1alpha1WorkflowStep> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1ParallelSteps> for Vec<IoArgoprojWorkflowV1alpha1WorkflowStep> {
    fn from(value: IoArgoprojWorkflowV1alpha1ParallelSteps) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1ParallelSteps> for IoArgoprojWorkflowV1alpha1ParallelSteps {
    fn from(value: &IoArgoprojWorkflowV1alpha1ParallelSteps) -> Self {
        value.clone()
    }
}
impl From<Vec<IoArgoprojWorkflowV1alpha1WorkflowStep>> for IoArgoprojWorkflowV1alpha1ParallelSteps {
    fn from(value: Vec<IoArgoprojWorkflowV1alpha1WorkflowStep>) -> Self {
        Self(value)
    }
}

pub use IoArgoprojWorkflowV1alpha1ParameterBuilder as ParameterBuilder;
pub use IoArgoprojWorkflowV1alpha1ParameterBuilderError as ParameterBuilderError;
#[doc = "Parameter indicate a passed string parameter to a service template with an optional default value"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
pub struct IoArgoprojWorkflowV1alpha1Parameter {
    #[builder(default)]
    #[doc = "Default is the default value to use for an input parameter if a value was not supplied"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[builder(default)]
    #[doc = "Description is the parameter description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default)]
    #[doc = "Enum holds a list of string values to choose from, for the actual value of the parameter"]
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<String>,
    #[builder(default)]
    #[doc = "GlobalName exports an output parameter to the global scope, making it available as '{{io.argoproj.workflow.v1alpha1.outputs.parameters.XXXX}} and in workflow.status.outputs.parameters"]
    #[serde(
        rename = "globalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub global_name: Option<String>,
    #[doc = "Name is the parameter name"]
    pub name: String,
    #[builder(default)]
    #[doc = "Value is the literal value to use for the parameter. If specified in the context of an input parameter, the value takes precedence over any passed values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[builder(default)]
    #[doc = "ValueFrom is the source for the output parameter's value"]
    #[serde(rename = "valueFrom", default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<IoArgoprojWorkflowV1alpha1ValueFrom>,
}
impl From<&IoArgoprojWorkflowV1alpha1Parameter> for IoArgoprojWorkflowV1alpha1Parameter {
    fn from(value: &IoArgoprojWorkflowV1alpha1Parameter) -> Self {
        value.clone()
    }
}
#[doc = "Plugin is an Object with exactly one key"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Plugin(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1Plugin {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1Plugin> for serde_json::Map<String, serde_json::Value> {
    fn from(value: IoArgoprojWorkflowV1alpha1Plugin) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1Plugin> for IoArgoprojWorkflowV1alpha1Plugin {
    fn from(value: &IoArgoprojWorkflowV1alpha1Plugin) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for IoArgoprojWorkflowV1alpha1Plugin {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "PodGC describes how to delete completed pods as they complete"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1PodGc {
    #[doc = "DeleteDelayDuration specifies the duration before pods in the GC queue get deleted."]
    #[serde(
        rename = "deleteDelayDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_delay_duration: Option<IoK8sApimachineryPkgApisMetaV1Duration>,
    #[doc = "LabelSelector is the label selector to check if the pods match the labels before being added to the pod GC queue."]
    #[serde(
        rename = "labelSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_selector: Option<IoK8sApimachineryPkgApisMetaV1LabelSelector>,
    #[doc = "Strategy is the strategy to use. One of \"OnPodCompletion\", \"OnPodSuccess\", \"OnWorkflowCompletion\", \"OnWorkflowSuccess\". If unset, does not delete Pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1PodGc> for IoArgoprojWorkflowV1alpha1PodGc {
    fn from(value: &IoArgoprojWorkflowV1alpha1PodGc) -> Self {
        value.clone()
    }
}
#[doc = "Prometheus is a prometheus metric to be emitted"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Prometheus {
    #[doc = "Counter is a counter metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counter: Option<IoArgoprojWorkflowV1alpha1Counter>,
    #[doc = "Gauge is a gauge metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gauge: Option<IoArgoprojWorkflowV1alpha1Gauge>,
    #[doc = "Help is a string that describes the metric"]
    pub help: String,
    #[doc = "Histogram is a histogram metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histogram: Option<IoArgoprojWorkflowV1alpha1Histogram>,
    #[doc = "Labels is a list of metric labels"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<IoArgoprojWorkflowV1alpha1MetricLabel>,
    #[doc = "Name is the name of the metric"]
    pub name: String,
    #[doc = "When is a conditional statement that decides when to emit the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1Prometheus> for IoArgoprojWorkflowV1alpha1Prometheus {
    fn from(value: &IoArgoprojWorkflowV1alpha1Prometheus) -> Self {
        value.clone()
    }
}
#[doc = "RawArtifact allows raw string content to be placed as an artifact in a container"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1RawArtifact {
    #[doc = "Data is the string contents of the artifact"]
    pub data: String,
}
impl From<&IoArgoprojWorkflowV1alpha1RawArtifact> for IoArgoprojWorkflowV1alpha1RawArtifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1RawArtifact) -> Self {
        value.clone()
    }
}
#[doc = "ResourceTemplate is a template subtype to manipulate kubernetes resources"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ResourceTemplate {
    #[doc = "Action is the action to perform to the resource. Must be one of: get, create, apply, delete, replace, patch"]
    pub action: String,
    #[doc = "FailureCondition is a label selector expression which describes the conditions of the k8s resource in which the step was considered failed"]
    #[serde(
        rename = "failureCondition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_condition: Option<String>,
    #[doc = "Flags is a set of additional options passed to kubectl before submitting a resource I.e. to disable resource validation: flags: [\n\t\"--validate=false\"  # disable resource validation\n]"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<String>,
    #[doc = "Manifest contains the kubernetes manifest"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[doc = "ManifestFrom is the source for a single kubernetes manifest"]
    #[serde(
        rename = "manifestFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_from: Option<IoArgoprojWorkflowV1alpha1ManifestFrom>,
    #[doc = "MergeStrategy is the strategy used to merge a patch. It defaults to \"strategic\" Must be one of: strategic, merge, json"]
    #[serde(
        rename = "mergeStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merge_strategy: Option<String>,
    #[doc = "SetOwnerReference sets the reference to the workflow on the OwnerReference of generated resource."]
    #[serde(
        rename = "setOwnerReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub set_owner_reference: Option<bool>,
    #[doc = "SuccessCondition is a label selector expression which describes the conditions of the k8s resource in which it is acceptable to proceed to the following step"]
    #[serde(
        rename = "successCondition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub success_condition: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ResourceTemplate>
    for IoArgoprojWorkflowV1alpha1ResourceTemplate
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ResourceTemplate) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ResubmitArchivedWorkflowRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memoized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ResubmitArchivedWorkflowRequest>
    for IoArgoprojWorkflowV1alpha1ResubmitArchivedWorkflowRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1ResubmitArchivedWorkflowRequest) -> Self {
        value.clone()
    }
}
#[doc = "RetryAffinity prevents running steps on the same host."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1RetryAffinity {
    #[serde(
        rename = "nodeAntiAffinity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_anti_affinity: Option<IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity>,
}
impl From<&IoArgoprojWorkflowV1alpha1RetryAffinity> for IoArgoprojWorkflowV1alpha1RetryAffinity {
    fn from(value: &IoArgoprojWorkflowV1alpha1RetryAffinity) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1RetryArchivedWorkflowRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "nodeFieldSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_field_selector: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[serde(
        rename = "restartSuccessful",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub restart_successful: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1RetryArchivedWorkflowRequest>
    for IoArgoprojWorkflowV1alpha1RetryArchivedWorkflowRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1RetryArchivedWorkflowRequest) -> Self {
        value.clone()
    }
}
#[doc = "RetryNodeAntiAffinity is a placeholder for future expansion, only empty nodeAntiAffinity is allowed. In order to prevent running steps on the same host, it uses \"kubernetes.io/hostname\"."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity>
    for IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity
{
    fn from(value: &IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1RetryNodeAntiAffinity
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "RetryStrategy provides controls on how to retry a workflow step"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1RetryStrategy {
    #[doc = "Affinity prevents running workflow's step on the same host"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<IoArgoprojWorkflowV1alpha1RetryAffinity>,
    #[doc = "Backoff is a backoff strategy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<IoArgoprojWorkflowV1alpha1Backoff>,
    #[doc = "Expression is a condition expression for when a node will be retried. If it evaluates to false, the node will not be retried and the retry strategy will be ignored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "Limit is the maximum number of retry attempts when retrying a container. It does not include the original container; the maximum number of total attempts will be `limit + 1`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "RetryPolicy is a policy of NodePhase statuses that will be retried"]
    #[serde(
        rename = "retryPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_policy: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1RetryStrategy> for IoArgoprojWorkflowV1alpha1RetryStrategy {
    fn from(value: &IoArgoprojWorkflowV1alpha1RetryStrategy) -> Self {
        value.clone()
    }
}
#[doc = "S3Artifact is the location of an S3 artifact"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1S3Artifact {
    #[doc = "AccessKeySecret is the secret selector to the bucket's access key"]
    #[serde(
        rename = "accessKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Bucket is the name of the bucket"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[doc = "CASecret specifies the secret that contains the CA, used to verify the TLS connection"]
    #[serde(rename = "caSecret", default, skip_serializing_if = "Option::is_none")]
    pub ca_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "CreateBucketIfNotPresent tells the driver to attempt to create the S3 bucket for output artifacts, if it doesn't exist. Setting Enabled Encryption will apply either SSE-S3 to the bucket if KmsKeyId is not set or SSE-KMS if it is."]
    #[serde(
        rename = "createBucketIfNotPresent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_bucket_if_not_present: Option<IoArgoprojWorkflowV1alpha1CreateS3BucketOptions>,
    #[serde(
        rename = "encryptionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_options: Option<IoArgoprojWorkflowV1alpha1S3EncryptionOptions>,
    #[doc = "Endpoint is the hostname of the bucket endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Insecure will connect to the service with TLS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[doc = "Key is the key in the bucket where the artifact resides"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Region contains the optional bucket region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "RoleARN is the Amazon Resource Name (ARN) of the role to assume."]
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[doc = "SecretKeySecret is the secret selector to the bucket's secret key"]
    #[serde(
        rename = "secretKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "UseSDKCreds tells the driver to figure out credentials based on sdk defaults."]
    #[serde(
        rename = "useSDKCreds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sdk_creds: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1S3Artifact> for IoArgoprojWorkflowV1alpha1S3Artifact {
    fn from(value: &IoArgoprojWorkflowV1alpha1S3Artifact) -> Self {
        value.clone()
    }
}
#[doc = "S3ArtifactRepository defines the controller configuration for an S3 artifact repository"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1S3ArtifactRepository {
    #[doc = "AccessKeySecret is the secret selector to the bucket's access key"]
    #[serde(
        rename = "accessKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "Bucket is the name of the bucket"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[doc = "CASecret specifies the secret that contains the CA, used to verify the TLS connection"]
    #[serde(rename = "caSecret", default, skip_serializing_if = "Option::is_none")]
    pub ca_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "CreateBucketIfNotPresent tells the driver to attempt to create the S3 bucket for output artifacts, if it doesn't exist. Setting Enabled Encryption will apply either SSE-S3 to the bucket if KmsKeyId is not set or SSE-KMS if it is."]
    #[serde(
        rename = "createBucketIfNotPresent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_bucket_if_not_present: Option<IoArgoprojWorkflowV1alpha1CreateS3BucketOptions>,
    #[serde(
        rename = "encryptionOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_options: Option<IoArgoprojWorkflowV1alpha1S3EncryptionOptions>,
    #[doc = "Endpoint is the hostname of the bucket endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Insecure will connect to the service with TLS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[doc = "KeyFormat defines the format of how to store keys and can reference workflow variables."]
    #[serde(rename = "keyFormat", default, skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[doc = "KeyPrefix is prefix used as part of the bucket key in which the controller will store artifacts. DEPRECATED. Use KeyFormat instead"]
    #[serde(rename = "keyPrefix", default, skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[doc = "Region contains the optional bucket region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "RoleARN is the Amazon Resource Name (ARN) of the role to assume."]
    #[serde(rename = "roleARN", default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[doc = "SecretKeySecret is the secret selector to the bucket's secret key"]
    #[serde(
        rename = "secretKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
    #[doc = "UseSDKCreds tells the driver to figure out credentials based on sdk defaults."]
    #[serde(
        rename = "useSDKCreds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_sdk_creds: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1S3ArtifactRepository>
    for IoArgoprojWorkflowV1alpha1S3ArtifactRepository
{
    fn from(value: &IoArgoprojWorkflowV1alpha1S3ArtifactRepository) -> Self {
        value.clone()
    }
}
#[doc = "S3EncryptionOptions used to determine encryption options during s3 operations"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1S3EncryptionOptions {
    #[doc = "EnableEncryption tells the driver to encrypt objects if set to true. If kmsKeyId and serverSideCustomerKeySecret are not set, SSE-S3 will be used"]
    #[serde(
        rename = "enableEncryption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_encryption: Option<bool>,
    #[doc = "KmsEncryptionContext is a json blob that contains an encryption context. See https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context for more information"]
    #[serde(
        rename = "kmsEncryptionContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_encryption_context: Option<String>,
    #[doc = "KMSKeyId tells the driver to encrypt the object using the specified KMS Key."]
    #[serde(rename = "kmsKeyId", default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[doc = "ServerSideCustomerKeySecret tells the driver to encrypt the output artifacts using SSE-C with the specified secret."]
    #[serde(
        rename = "serverSideCustomerKeySecret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_side_customer_key_secret: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoArgoprojWorkflowV1alpha1S3EncryptionOptions>
    for IoArgoprojWorkflowV1alpha1S3EncryptionOptions
{
    fn from(value: &IoArgoprojWorkflowV1alpha1S3EncryptionOptions) -> Self {
        value.clone()
    }
}
#[doc = "ScriptTemplate is a template subtype to enable scripting through code steps"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ScriptTemplate {
    #[doc = "Arguments to the entrypoint. The container image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[doc = "Entrypoint array. Not executed within a shell. The container image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "List of environment variables to set in the container. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<IoK8sApiCoreV1EnvVar>,
    #[doc = "List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated."]
    #[serde(rename = "envFrom", default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<IoK8sApiCoreV1EnvFromSource>,
    #[doc = "Container image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets."]
    pub image: String,
    #[doc = "Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images"]
    #[serde(
        rename = "imagePullPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_pull_policy: Option<String>,
    #[doc = "Actions that the management system should take in response to container lifecycle events. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<IoK8sApiCoreV1Lifecycle>,
    #[doc = "Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "livenessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub liveness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IoK8sApiCoreV1ContainerPort>,
    #[doc = "Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "readinessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readiness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<IoK8sApiCoreV1ResourceRequirements>,
    #[doc = "SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/"]
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1SecurityContext>,
    #[doc = "Source contains the source code of the script to execute"]
    pub source: String,
    #[doc = "StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "startupProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    #[doc = "Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false"]
    #[serde(rename = "stdinOnce", default, skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    #[doc = "Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_path: Option<String>,
    #[doc = "Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_policy: Option<String>,
    #[doc = "Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[doc = "volumeDevices is the list of block devices to be used by the container."]
    #[serde(
        rename = "volumeDevices",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_devices: Vec<IoK8sApiCoreV1VolumeDevice>,
    #[doc = "Pod volumes to mount into the container's filesystem. Cannot be updated."]
    #[serde(
        rename = "volumeMounts",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_mounts: Vec<IoK8sApiCoreV1VolumeMount>,
    #[doc = "Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated."]
    #[serde(
        rename = "workingDir",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_dir: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1ScriptTemplate> for IoArgoprojWorkflowV1alpha1ScriptTemplate {
    fn from(value: &IoArgoprojWorkflowV1alpha1ScriptTemplate) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SemaphoreHolding {
    #[doc = "Holders stores the list of current holder names in the io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holders: Vec<String>,
    #[doc = "Semaphore stores the semaphore name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1SemaphoreHolding>
    for IoArgoprojWorkflowV1alpha1SemaphoreHolding
{
    fn from(value: &IoArgoprojWorkflowV1alpha1SemaphoreHolding) -> Self {
        value.clone()
    }
}
#[doc = "SemaphoreRef is a reference of Semaphore"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SemaphoreRef {
    #[doc = "ConfigMapKeyRef is configmap selector for Semaphore configuration"]
    #[serde(
        rename = "configMapKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_map_key_ref: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[doc = "Namespace is the namespace of the configmap, default: [namespace of workflow]"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1SemaphoreRef> for IoArgoprojWorkflowV1alpha1SemaphoreRef {
    fn from(value: &IoArgoprojWorkflowV1alpha1SemaphoreRef) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SemaphoreStatus {
    #[doc = "Holding stores the list of resource acquired synchronization lock for workflows."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding: Vec<IoArgoprojWorkflowV1alpha1SemaphoreHolding>,
    #[doc = "Waiting indicates the list of current synchronization lock holders."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub waiting: Vec<IoArgoprojWorkflowV1alpha1SemaphoreHolding>,
}
impl From<&IoArgoprojWorkflowV1alpha1SemaphoreStatus>
    for IoArgoprojWorkflowV1alpha1SemaphoreStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1SemaphoreStatus) -> Self {
        value.clone()
    }
}
#[doc = "Sequence expands a workflow step into numeric range"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Sequence {
    #[doc = "Count is number of elements in the sequence (default: 0). Not to be used with end"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "Number at which to end the sequence (default: 0). Not to be used with Count"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "Format is a printf format string to format the value in the sequence"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "Number at which to start the sequence (default: 0)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
}
impl From<&IoArgoprojWorkflowV1alpha1Sequence> for IoArgoprojWorkflowV1alpha1Sequence {
    fn from(value: &IoArgoprojWorkflowV1alpha1Sequence) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Submit {
    #[doc = "Arguments extracted from the event and then set as arguments to the workflow created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<IoArgoprojWorkflowV1alpha1Arguments>,
    #[doc = "Metadata optional means to customize select fields of the workflow metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
    #[doc = "WorkflowTemplateRef the workflow template to submit"]
    #[serde(rename = "workflowTemplateRef")]
    pub workflow_template_ref: IoArgoprojWorkflowV1alpha1WorkflowTemplateRef,
}
impl From<&IoArgoprojWorkflowV1alpha1Submit> for IoArgoprojWorkflowV1alpha1Submit {
    fn from(value: &IoArgoprojWorkflowV1alpha1Submit) -> Self {
        value.clone()
    }
}
#[doc = "SubmitOpts are workflow submission options"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SubmitOpts {
    #[doc = "Annotations adds to metadata.labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,
    #[doc = "DryRun validates the workflow on the client-side without creating it. This option is not supported in API"]
    #[serde(rename = "dryRun", default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[doc = "Entrypoint overrides spec.entrypoint"]
    #[serde(
        rename = "entryPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_point: Option<String>,
    #[doc = "GenerateName overrides metadata.generateName"]
    #[serde(
        rename = "generateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_name: Option<String>,
    #[doc = "Labels adds to metadata.labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[doc = "Name overrides metadata.name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "OwnerReference creates a metadata.ownerReference"]
    #[serde(
        rename = "ownerReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_reference: Option<IoK8sApimachineryPkgApisMetaV1OwnerReference>,
    #[doc = "Parameters passes input parameters to workflow"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[doc = "Set the podPriorityClassName of the workflow"]
    #[serde(
        rename = "podPriorityClassName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_priority_class_name: Option<String>,
    #[doc = "Priority is used if controller is configured to process limited number of workflows in parallel, higher priority workflows are processed first."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "ServerDryRun validates the workflow on the server-side without creating it"]
    #[serde(
        rename = "serverDryRun",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_dry_run: Option<bool>,
    #[doc = "ServiceAccount runs all pods in the workflow using specified ServiceAccount."]
    #[serde(
        rename = "serviceAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1SubmitOpts> for IoArgoprojWorkflowV1alpha1SubmitOpts {
    fn from(value: &IoArgoprojWorkflowV1alpha1SubmitOpts) -> Self {
        value.clone()
    }
}
#[doc = "SuppliedValueFrom is a placeholder for a value to be filled in directly, either through the CLI, API, etc."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SuppliedValueFrom(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1SuppliedValueFrom {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1SuppliedValueFrom>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1SuppliedValueFrom) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1SuppliedValueFrom>
    for IoArgoprojWorkflowV1alpha1SuppliedValueFrom
{
    fn from(value: &IoArgoprojWorkflowV1alpha1SuppliedValueFrom) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1SuppliedValueFrom
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "SuspendTemplate is a template subtype to suspend a workflow at a predetermined point in time"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SuspendTemplate {
    #[doc = "Duration is the seconds to wait before automatically resuming a template. Must be a string. Default unit is seconds. Could also be a Duration, e.g.: \"2m\", \"6h\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1SuspendTemplate>
    for IoArgoprojWorkflowV1alpha1SuspendTemplate
{
    fn from(value: &IoArgoprojWorkflowV1alpha1SuspendTemplate) -> Self {
        value.clone()
    }
}
#[doc = "Synchronization holds synchronization lock configuration"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Synchronization {
    #[doc = "Mutex holds the Mutex lock details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutex: Option<IoArgoprojWorkflowV1alpha1Mutex>,
    #[doc = "Semaphore holds the Semaphore configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<IoArgoprojWorkflowV1alpha1SemaphoreRef>,
}
impl From<&IoArgoprojWorkflowV1alpha1Synchronization>
    for IoArgoprojWorkflowV1alpha1Synchronization
{
    fn from(value: &IoArgoprojWorkflowV1alpha1Synchronization) -> Self {
        value.clone()
    }
}
#[doc = "SynchronizationStatus stores the status of semaphore and mutex."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1SynchronizationStatus {
    #[doc = "Mutex stores this workflow's mutex holder details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutex: Option<IoArgoprojWorkflowV1alpha1MutexStatus>,
    #[doc = "Semaphore stores this workflow's Semaphore holder details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<IoArgoprojWorkflowV1alpha1SemaphoreStatus>,
}
impl From<&IoArgoprojWorkflowV1alpha1SynchronizationStatus>
    for IoArgoprojWorkflowV1alpha1SynchronizationStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1SynchronizationStatus) -> Self {
        value.clone()
    }
}
#[doc = "TarStrategy will tar and gzip the file or directory when saving"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1TarStrategy {
    #[doc = "CompressionLevel specifies the gzip compression level to use for the artifact. Defaults to gzip.DefaultCompression."]
    #[serde(
        rename = "compressionLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compression_level: Option<i64>,
}
impl From<&IoArgoprojWorkflowV1alpha1TarStrategy> for IoArgoprojWorkflowV1alpha1TarStrategy {
    fn from(value: &IoArgoprojWorkflowV1alpha1TarStrategy) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1TemplateBuilder as TemplateBuilder;
pub use IoArgoprojWorkflowV1alpha1TemplateBuilderError as TemplateBuilderError;

#[doc = "Template is a reusable and composable unit of execution in a workflow"]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1Template {
    #[doc = "Optional duration in seconds relative to the StartTime that the pod may be active on a node before the system actively tries to terminate the pod; value must be positive integer This field is only applicable to container and script templates."]
    #[serde(
        rename = "activeDeadlineSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_deadline_seconds: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "Affinity sets the pod's scheduling constraints Overrides the affinity set at the workflow level (if any)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<IoK8sApiCoreV1Affinity>,
    #[doc = "Location in which all files related to the step will be stored (logs, artifacts, etc...). Can be overridden by individual items in Outputs. If omitted, will use the default artifact repository location configured in the controller, appended with the <workflowname>/<nodename> in the key."]
    #[serde(
        rename = "archiveLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_location: Option<IoArgoprojWorkflowV1alpha1ArtifactLocation>,
    #[doc = "AutomountServiceAccountToken indicates whether a service account token should be automatically mounted in pods. ServiceAccountName of ExecutorConfig must be specified if this value is false."]
    #[serde(
        rename = "automountServiceAccountToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automount_service_account_token: Option<bool>,
    #[doc = "Container is the main container image to run in the pod"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<IoK8sApiCoreV1Container>,
    #[doc = "ContainerSet groups multiple containers within a single pod."]
    #[serde(
        rename = "containerSet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_set: Option<IoArgoprojWorkflowV1alpha1ContainerSetTemplate>,
    #[doc = "Daemon will allow a workflow to proceed to the next step so long as the container reaches readiness"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemon: Option<bool>,
    #[doc = "DAG template subtype which runs a DAG"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dag: Option<IoArgoprojWorkflowV1alpha1DagTemplate>,
    #[doc = "Data is a data template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<IoArgoprojWorkflowV1alpha1Data>,
    #[doc = "Executor holds configurations of the executor container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executor: Option<IoArgoprojWorkflowV1alpha1ExecutorConfig>,
    #[doc = "FailFast, if specified, will fail this template if any of its child pods has failed. This is useful for when this template is expanded with `withItems`, etc."]
    #[serde(rename = "failFast", default, skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    #[doc = "HostAliases is an optional list of hosts and IPs that will be injected into the pod spec"]
    #[serde(rename = "hostAliases", default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<IoK8sApiCoreV1HostAlias>,
    #[doc = "HTTP makes a HTTP request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<IoArgoprojWorkflowV1alpha1Http>,
    #[doc = "InitContainers is a list of containers which run before the main container."]
    #[serde(
        rename = "initContainers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub init_containers: Vec<IoArgoprojWorkflowV1alpha1UserContainer>,
    #[doc = "Inputs describe what inputs parameters and artifacts are supplied to this template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<IoArgoprojWorkflowV1alpha1Inputs>,
    #[doc = "Memoize allows templates to use outputs generated from already executed templates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memoize: Option<IoArgoprojWorkflowV1alpha1Memoize>,
    #[doc = "Metdata sets the pods's metadata, i.e. annotations and labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoArgoprojWorkflowV1alpha1Metadata>,
    #[doc = "Metrics are a list of metrics emitted from this template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<IoArgoprojWorkflowV1alpha1Metrics>,
    #[doc = "Name is the name of the template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "NodeSelector is a selector to schedule this step of the workflow to be run on the selected node(s). Overrides the selector set at the workflow level."]
    #[serde(
        rename = "nodeSelector",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub node_selector: std::collections::HashMap<String, String>,
    #[doc = "Outputs describe the parameters and artifacts that this template produces"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IoArgoprojWorkflowV1alpha1Outputs>,
    #[doc = "Parallelism limits the max total parallel pods that can execute at the same time within the boundaries of this template invocation. If additional steps/dag templates are invoked, the pods created by those templates will not be counted towards this total."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    #[doc = "Plugin is a plugin template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<IoArgoprojWorkflowV1alpha1Plugin>,
    #[doc = "PodSpecPatch holds strategic merge patch to apply against the pod spec. Allows parameterization of container fields which are not strings (e.g. resource limits)."]
    #[serde(
        rename = "podSpecPatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_spec_patch: Option<String>,
    #[doc = "Priority to apply to workflow pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "PriorityClassName to apply to workflow pods."]
    #[serde(
        rename = "priorityClassName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub priority_class_name: Option<String>,
    #[doc = "Resource template subtype which can run k8s resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<IoArgoprojWorkflowV1alpha1ResourceTemplate>,
    #[doc = "RetryStrategy describes how to retry a template when it fails"]
    #[serde(
        rename = "retryStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_strategy: Option<IoArgoprojWorkflowV1alpha1RetryStrategy>,
    #[doc = "If specified, the pod will be dispatched by specified scheduler. Or it will be dispatched by workflow scope scheduler if specified. If neither specified, the pod will be dispatched by default scheduler."]
    #[serde(
        rename = "schedulerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduler_name: Option<String>,
    #[doc = "Script runs a portion of code against an interpreter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<IoArgoprojWorkflowV1alpha1ScriptTemplate>,
    #[doc = "SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field."]
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1PodSecurityContext>,
    #[doc = "ServiceAccountName to apply to workflow pods"]
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
    #[doc = "Sidecars is a list of containers which run alongside the main container Sidecars are automatically killed when the main container completes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sidecars: Vec<IoArgoprojWorkflowV1alpha1UserContainer>,
    #[doc = "Steps define a series of sequential/parallel workflow steps"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<IoArgoprojWorkflowV1alpha1ParallelSteps>,
    #[doc = "Suspend template subtype which can suspend a workflow when reaching the step"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<IoArgoprojWorkflowV1alpha1SuspendTemplate>,
    #[doc = "Synchronization holds synchronization lock configuration for this template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<IoArgoprojWorkflowV1alpha1Synchronization>,
    #[doc = "Timeout allows to set the total node execution timeout duration counting from the node's start time. This duration also includes time in which the node spends in Pending state. This duration may not be applied to Step or DAG templates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "Tolerations to apply to workflow pods."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<IoK8sApiCoreV1Toleration>,
    #[doc = "Volumes is a list of volumes that can be mounted by containers in a template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<IoK8sApiCoreV1Volume>,
}
impl From<&IoArgoprojWorkflowV1alpha1Template> for IoArgoprojWorkflowV1alpha1Template {
    fn from(value: &IoArgoprojWorkflowV1alpha1Template) -> Self {
        value.clone()
    }
}
#[doc = "TemplateRef is a reference of template resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1TemplateRef {
    #[doc = "ClusterScope indicates the referred template is cluster scoped (i.e. a ClusterWorkflowTemplate)."]
    #[serde(
        rename = "clusterScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_scope: Option<bool>,
    #[doc = "Name is the resource name of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Template is the name of referred template in the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1TemplateRef> for IoArgoprojWorkflowV1alpha1TemplateRef {
    fn from(value: &IoArgoprojWorkflowV1alpha1TemplateRef) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1TransformationStep {
    #[doc = "Expression defines an expr expression to apply"]
    pub expression: String,
}
impl From<&IoArgoprojWorkflowV1alpha1TransformationStep>
    for IoArgoprojWorkflowV1alpha1TransformationStep
{
    fn from(value: &IoArgoprojWorkflowV1alpha1TransformationStep) -> Self {
        value.clone()
    }
}
#[doc = "TTLStrategy is the strategy for the time to live depending on if the workflow succeeded or failed"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1TtlStrategy {
    #[doc = "SecondsAfterCompletion is the number of seconds to live after completion"]
    #[serde(
        rename = "secondsAfterCompletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_after_completion: Option<i64>,
    #[doc = "SecondsAfterFailure is the number of seconds to live after failure"]
    #[serde(
        rename = "secondsAfterFailure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_after_failure: Option<i64>,
    #[doc = "SecondsAfterSuccess is the number of seconds to live after success"]
    #[serde(
        rename = "secondsAfterSuccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_after_success: Option<i64>,
}
impl From<&IoArgoprojWorkflowV1alpha1TtlStrategy> for IoArgoprojWorkflowV1alpha1TtlStrategy {
    fn from(value: &IoArgoprojWorkflowV1alpha1TtlStrategy) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1UpdateCronWorkflowRequest {
    #[serde(
        rename = "cronWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cron_workflow: Option<IoArgoprojWorkflowV1alpha1CronWorkflow>,
    #[doc = "DEPRECATED: This field is ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1UpdateCronWorkflowRequest>
    for IoArgoprojWorkflowV1alpha1UpdateCronWorkflowRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1UpdateCronWorkflowRequest) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1UserContainerBuilder as UserContainerBuilder;
pub use IoArgoprojWorkflowV1alpha1UserContainerBuilderError as UserContainerBuilderError;

#[doc = "UserContainer is a container specified by a user."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1UserContainer {
    #[doc = "Arguments to the entrypoint. The container image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[doc = "Entrypoint array. Not executed within a shell. The container image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "List of environment variables to set in the container. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<IoK8sApiCoreV1EnvVar>,
    #[doc = "List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated."]
    #[serde(rename = "envFrom", default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<IoK8sApiCoreV1EnvFromSource>,
    #[doc = "Container image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images"]
    #[serde(
        rename = "imagePullPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_pull_policy: Option<String>,
    #[doc = "Actions that the management system should take in response to container lifecycle events. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<IoK8sApiCoreV1Lifecycle>,
    #[doc = "Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "livenessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub liveness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "MirrorVolumeMounts will mount the same volumes specified in the main container to the container (including artifacts), at the same mountPaths. This enables dind daemon to partially see the same filesystem as the main container in order to use features such as docker volume binding"]
    #[serde(
        rename = "mirrorVolumeMounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_volume_mounts: Option<bool>,
    #[doc = "Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated."]
    pub name: String,
    #[doc = "List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IoK8sApiCoreV1ContainerPort>,
    #[doc = "Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "readinessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readiness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<IoK8sApiCoreV1ResourceRequirements>,
    #[doc = "SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/"]
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1SecurityContext>,
    #[doc = "StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "startupProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    #[doc = "Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false"]
    #[serde(rename = "stdinOnce", default, skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    #[doc = "Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_path: Option<String>,
    #[doc = "Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_policy: Option<String>,
    #[doc = "Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[doc = "volumeDevices is the list of block devices to be used by the container."]
    #[serde(
        rename = "volumeDevices",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_devices: Vec<IoK8sApiCoreV1VolumeDevice>,
    #[doc = "Pod volumes to mount into the container's filesystem. Cannot be updated."]
    #[serde(
        rename = "volumeMounts",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_mounts: Vec<IoK8sApiCoreV1VolumeMount>,
    #[doc = "Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated."]
    #[serde(
        rename = "workingDir",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_dir: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1UserContainer> for IoArgoprojWorkflowV1alpha1UserContainer {
    fn from(value: &IoArgoprojWorkflowV1alpha1UserContainer) -> Self {
        value.clone()
    }
}
#[doc = "ValueFrom describes a location in which to obtain the value to a parameter"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ValueFrom {
    #[doc = "ConfigMapKeyRef is configmap selector for input parameter configuration"]
    #[serde(
        rename = "configMapKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_map_key_ref: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[doc = "Default specifies a value to be used if retrieving the value from the specified source fails"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[doc = "Selector (https://github.com/antonmedv/expr) that is evaluated against the event to get the value of the parameter. E.g. `payload.message`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[doc = "Expression, if defined, is evaluated to specify the value for the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "JQFilter expression against the resource object in resource templates"]
    #[serde(rename = "jqFilter", default, skip_serializing_if = "Option::is_none")]
    pub jq_filter: Option<String>,
    #[doc = "JSONPath of a resource to retrieve an output parameter value from in resource templates"]
    #[serde(rename = "jsonPath", default, skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[doc = "Parameter reference to a step or dag task in which to retrieve an output parameter value from (e.g. '{{steps.mystep.outputs.myparam}}')"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[doc = "Path in the container to retrieve an output parameter value from in container templates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Supplied value to be filled in directly, either through the CLI, API, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplied: Option<IoArgoprojWorkflowV1alpha1SuppliedValueFrom>,
}
impl From<&IoArgoprojWorkflowV1alpha1ValueFrom> for IoArgoprojWorkflowV1alpha1ValueFrom {
    fn from(value: &IoArgoprojWorkflowV1alpha1ValueFrom) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Version {
    #[serde(rename = "buildDate")]
    pub build_date: String,
    pub compiler: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    #[serde(rename = "gitTag")]
    pub git_tag: String,
    #[serde(rename = "gitTreeState")]
    pub git_tree_state: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
    pub platform: String,
    pub version: String,
}
impl From<&IoArgoprojWorkflowV1alpha1Version> for IoArgoprojWorkflowV1alpha1Version {
    fn from(value: &IoArgoprojWorkflowV1alpha1Version) -> Self {
        value.clone()
    }
}
#[doc = "VolumeClaimGC describes how to delete volumes from completed Workflows"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1VolumeClaimGc {
    #[doc = "Strategy is the strategy to use. One of \"OnWorkflowCompletion\", \"OnWorkflowSuccess\". Defaults to \"OnWorkflowSuccess\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1VolumeClaimGc> for IoArgoprojWorkflowV1alpha1VolumeClaimGc {
    fn from(value: &IoArgoprojWorkflowV1alpha1VolumeClaimGc) -> Self {
        value.clone()
    }
}
#[doc = "Workflow is the definition of a workflow resource"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1Workflow {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    pub spec: IoArgoprojWorkflowV1alpha1WorkflowSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoArgoprojWorkflowV1alpha1WorkflowStatus>,
}
impl From<&IoArgoprojWorkflowV1alpha1Workflow> for IoArgoprojWorkflowV1alpha1Workflow {
    fn from(value: &IoArgoprojWorkflowV1alpha1Workflow) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowCreateRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[doc = "This field is no longer used."]
    #[serde(
        rename = "instanceID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "serverDryRun",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_dry_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<IoArgoprojWorkflowV1alpha1Workflow>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowCreateRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowCreateRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowCreateRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse>
    for IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1WorkflowDeleteResponse
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "WorkflowEventBinding is the definition of an event resource"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowEventBinding {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    pub spec: IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowEventBinding>
    for IoArgoprojWorkflowV1alpha1WorkflowEventBinding
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowEventBinding) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowEventBindingList is list of event resources"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowEventBindingList {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    pub items: Vec<IoArgoprojWorkflowV1alpha1WorkflowEventBinding>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ListMeta,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowEventBindingList>
    for IoArgoprojWorkflowV1alpha1WorkflowEventBindingList
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowEventBindingList) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec {
    #[doc = "Event is the event to bind to"]
    pub event: IoArgoprojWorkflowV1alpha1Event,
    #[doc = "Submit is the workflow template to submit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit: Option<IoArgoprojWorkflowV1alpha1Submit>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec>
    for IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowEventBindingSpec) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowLevelArtifactGC describes how to delete artifacts from completed Workflows - this spec is used on the Workflow level"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowLevelArtifactGc {
    #[doc = "ForceFinalizerRemoval: if set to true, the finalizer will be removed in the case that Artifact GC fails"]
    #[serde(
        rename = "forceFinalizerRemoval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_finalizer_removal: Option<bool>,
    #[doc = "PodMetadata is an optional field for specifying the Labels and Annotations that should be assigned to the Pod doing the deletion"]
    #[serde(
        rename = "podMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_metadata: Option<IoArgoprojWorkflowV1alpha1Metadata>,
    #[doc = "PodSpecPatch holds strategic merge patch to apply against the artgc pod spec."]
    #[serde(
        rename = "podSpecPatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_spec_patch: Option<String>,
    #[doc = "ServiceAccountName is an optional field for specifying the Service Account that should be assigned to the Pod doing the deletion"]
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
    #[doc = "Strategy is the strategy to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowLevelArtifactGc>
    for IoArgoprojWorkflowV1alpha1WorkflowLevelArtifactGc
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowLevelArtifactGc) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowLintRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<IoArgoprojWorkflowV1alpha1Workflow>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowLintRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowLintRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowLintRequest) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowList is list of Workflow resources"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowList {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    pub items: Vec<IoArgoprojWorkflowV1alpha1Workflow>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ListMeta,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowList> for IoArgoprojWorkflowV1alpha1WorkflowList {
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowList) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowMetadata {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub annotations: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,
    #[serde(
        rename = "labelsFrom",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub labels_from: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1LabelValueFrom>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowMetadata>
    for IoArgoprojWorkflowV1alpha1WorkflowMetadata
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowMetadata) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowResubmitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memoized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowResubmitRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowResubmitRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowResubmitRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowResumeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "nodeFieldSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_field_selector: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowResumeRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowResumeRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowResumeRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowRetryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "nodeFieldSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_field_selector: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[serde(
        rename = "restartSuccessful",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub restart_successful: Option<bool>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowRetryRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowRetryRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowRetryRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowSetRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "nodeFieldSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_field_selector: Option<String>,
    #[serde(
        rename = "outputParameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub output_parameters: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowSetRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowSetRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowSetRequest) -> Self {
        value.clone()
    }
}

pub use IoArgoprojWorkflowV1alpha1WorkflowSpecBuilder as WorkflowSpecBuilder;
pub use IoArgoprojWorkflowV1alpha1WorkflowSpecBuilderError as WorkflowSpecBuilderError;

#[doc = "WorkflowSpec is the specification of a Workflow."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowSpec {
    #[doc = "Optional duration in seconds relative to the workflow start time which the workflow is allowed to run before the controller terminates the io.argoproj.workflow.v1alpha1. A value of zero is used to terminate a Running workflow"]
    #[serde(
        rename = "activeDeadlineSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_deadline_seconds: Option<i64>,
    #[doc = "Affinity sets the scheduling constraints for all pods in the io.argoproj.workflow.v1alpha1. Can be overridden by an affinity specified in the template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<IoK8sApiCoreV1Affinity>,
    #[doc = "ArchiveLogs indicates if the container logs should be archived"]
    #[serde(
        rename = "archiveLogs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_logs: Option<bool>,
    #[doc = "Arguments contain the parameters and artifacts sent to the workflow entrypoint Parameters are referencable globally using the 'workflow' variable prefix. e.g. {{io.argoproj.workflow.v1alpha1.parameters.myparam}}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<IoArgoprojWorkflowV1alpha1Arguments>,
    #[doc = "ArtifactGC describes the strategy to use when deleting artifacts from completed or deleted workflows (applies to all output Artifacts unless Artifact.ArtifactGC is specified, which overrides this)"]
    #[serde(
        rename = "artifactGC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_gc: Option<IoArgoprojWorkflowV1alpha1WorkflowLevelArtifactGc>,
    #[doc = "ArtifactRepositoryRef specifies the configMap name and key containing the artifact repository config."]
    #[serde(
        rename = "artifactRepositoryRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_repository_ref: Option<IoArgoprojWorkflowV1alpha1ArtifactRepositoryRef>,
    #[doc = "AutomountServiceAccountToken indicates whether a service account token should be automatically mounted in pods. ServiceAccountName of ExecutorConfig must be specified if this value is false."]
    #[serde(
        rename = "automountServiceAccountToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automount_service_account_token: Option<bool>,
    #[doc = "PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy."]
    #[serde(rename = "dnsConfig", default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<IoK8sApiCoreV1PodDnsConfig>,
    #[doc = "Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'."]
    #[serde(rename = "dnsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
    #[doc = "Entrypoint is a template reference to the starting point of the io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[doc = "Executor holds configurations of executor containers of the io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executor: Option<IoArgoprojWorkflowV1alpha1ExecutorConfig>,
    #[doc = "Hooks holds the lifecycle hook which is invoked at lifecycle of step, irrespective of the success, failure, or error status of the primary step"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub hooks: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1LifecycleHook>,
    #[serde(rename = "hostAliases", default, skip_serializing_if = "Vec::is_empty")]
    pub host_aliases: Vec<IoK8sApiCoreV1HostAlias>,
    #[doc = "Host networking requested for this workflow pod. Default to false."]
    #[serde(
        rename = "hostNetwork",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_network: Option<bool>,
    #[doc = "ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod"]
    #[serde(
        rename = "imagePullSecrets",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_pull_secrets: Vec<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "Metrics are a list of metrics emitted from this Workflow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<IoArgoprojWorkflowV1alpha1Metrics>,
    #[doc = "NodeSelector is a selector which will result in all pods of the workflow to be scheduled on the selected node(s). This is able to be overridden by a nodeSelector specified in the template."]
    #[serde(
        rename = "nodeSelector",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub node_selector: std::collections::HashMap<String, String>,
    #[doc = "OnExit is a template reference which is invoked at the end of the workflow, irrespective of the success, failure, or error of the primary io.argoproj.workflow.v1alpha1."]
    #[serde(rename = "onExit", default, skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<String>,
    #[doc = "Parallelism limits the max total parallel pods that can execute at the same time in a workflow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    #[doc = "PodDisruptionBudget holds the number of concurrent disruptions that you allow for Workflow's Pods. Controller will automatically add the selector with workflow name, if selector is empty. Optional: Defaults to empty."]
    #[serde(
        rename = "podDisruptionBudget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_disruption_budget: Option<IoK8sApiPolicyV1PodDisruptionBudgetSpec>,
    #[doc = "PodGC describes the strategy to use when deleting completed pods"]
    #[serde(rename = "podGC", default, skip_serializing_if = "Option::is_none")]
    pub pod_gc: Option<IoArgoprojWorkflowV1alpha1PodGc>,
    #[doc = "PodMetadata defines additional metadata that should be applied to workflow pods"]
    #[serde(
        rename = "podMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_metadata: Option<IoArgoprojWorkflowV1alpha1Metadata>,
    #[doc = "Priority to apply to workflow pods. DEPRECATED: Use PodPriorityClassName instead."]
    #[serde(
        rename = "podPriority",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_priority: Option<i64>,
    #[doc = "PriorityClassName to apply to workflow pods."]
    #[serde(
        rename = "podPriorityClassName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_priority_class_name: Option<String>,
    #[doc = "PodSpecPatch holds strategic merge patch to apply against the pod spec. Allows parameterization of container fields which are not strings (e.g. resource limits)."]
    #[serde(
        rename = "podSpecPatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_spec_patch: Option<String>,
    #[doc = "Priority is used if controller is configured to process limited number of workflows in parallel. Workflows with higher priority are processed first."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "RetryStrategy for all templates in the io.argoproj.workflow.v1alpha1."]
    #[serde(
        rename = "retryStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retry_strategy: Option<IoArgoprojWorkflowV1alpha1RetryStrategy>,
    #[doc = "Set scheduler name for all pods. Will be overridden if container/script template's scheduler name is set. Default scheduler will be used if neither specified."]
    #[serde(
        rename = "schedulerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduler_name: Option<String>,
    #[doc = "SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field."]
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1PodSecurityContext>,
    #[doc = "ServiceAccountName is the name of the ServiceAccount to run all pods of the workflow as."]
    #[serde(
        rename = "serviceAccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_name: Option<String>,
    #[doc = "Shutdown will shutdown the workflow according to its ShutdownStrategy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shutdown: Option<String>,
    #[doc = "Suspend will suspend the workflow and prevent execution of any future steps in the workflow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    #[doc = "Synchronization holds synchronization lock configuration for this Workflow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<IoArgoprojWorkflowV1alpha1Synchronization>,
    #[doc = "TemplateDefaults holds default template values that will apply to all templates in the Workflow, unless overridden on the template-level"]
    #[serde(
        rename = "templateDefaults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_defaults: Option<IoArgoprojWorkflowV1alpha1Template>,
    #[doc = "Templates is a list of workflow templates used in a workflow"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<IoArgoprojWorkflowV1alpha1Template>,
    #[doc = "Tolerations to apply to workflow pods."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tolerations: Vec<IoK8sApiCoreV1Toleration>,
    #[doc = "TTLStrategy limits the lifetime of a Workflow that has finished execution depending on if it Succeeded or Failed. If this struct is set, once the Workflow finishes, it will be deleted after the time to live expires. If this field is unset, the controller config map will hold the default values."]
    #[serde(
        rename = "ttlStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ttl_strategy: Option<IoArgoprojWorkflowV1alpha1TtlStrategy>,
    #[doc = "VolumeClaimGC describes the strategy to use when deleting volumes from completed workflows"]
    #[serde(
        rename = "volumeClaimGC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_claim_gc: Option<IoArgoprojWorkflowV1alpha1VolumeClaimGc>,
    #[doc = "VolumeClaimTemplates is a list of claims that containers are allowed to reference. The Workflow controller will create the claims at the beginning of the workflow and delete the claims upon completion of the workflow"]
    #[serde(
        rename = "volumeClaimTemplates",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_claim_templates: Vec<IoK8sApiCoreV1PersistentVolumeClaim>,
    #[doc = "Volumes is a list of volumes that can be mounted by containers in a io.argoproj.workflow.v1alpha1."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<IoK8sApiCoreV1Volume>,
    #[doc = "WorkflowMetadata contains some metadata of the workflow to refer to"]
    #[serde(
        rename = "workflowMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_metadata: Option<IoArgoprojWorkflowV1alpha1WorkflowMetadata>,
    #[doc = "WorkflowTemplateRef holds a reference to a WorkflowTemplate for execution"]
    #[serde(
        rename = "workflowTemplateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_template_ref: Option<IoArgoprojWorkflowV1alpha1WorkflowTemplateRef>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowSpec> for IoArgoprojWorkflowV1alpha1WorkflowSpec {
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowSpec) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowStatus contains overall status information about a workflow"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowStatus {
    #[doc = "ArtifactGCStatus maintains the status of Artifact Garbage Collection"]
    #[serde(
        rename = "artifactGCStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_gc_status: Option<IoArgoprojWorkflowV1alpha1ArtGcStatus>,
    #[doc = "ArtifactRepositoryRef is used to cache the repository to use so we do not need to determine it everytime we reconcile."]
    #[serde(
        rename = "artifactRepositoryRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_repository_ref: Option<IoArgoprojWorkflowV1alpha1ArtifactRepositoryRefStatus>,
    #[doc = "Compressed and base64 decoded Nodes map"]
    #[serde(
        rename = "compressedNodes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compressed_nodes: Option<String>,
    #[doc = "Conditions is a list of conditions the Workflow may have"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<IoArgoprojWorkflowV1alpha1Condition>,
    #[doc = "EstimatedDuration in seconds."]
    #[serde(
        rename = "estimatedDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_duration: Option<i64>,
    #[doc = "Time at which this workflow completed"]
    #[serde(
        rename = "finishedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub finished_at: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "A human readable message indicating details about why the workflow is in this condition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Nodes is a mapping between a node ID and the node's status."]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub nodes: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1NodeStatus>,
    #[doc = "Whether on not node status has been offloaded to a database. If exists, then Nodes and CompressedNodes will be empty. This will actually be populated with a hash of the offloaded data."]
    #[serde(
        rename = "offloadNodeStatusVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offload_node_status_version: Option<String>,
    #[doc = "Outputs captures output values and artifact locations produced by the workflow via global outputs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IoArgoprojWorkflowV1alpha1Outputs>,
    #[doc = "PersistentVolumeClaims tracks all PVCs that were created as part of the io.argoproj.workflow.v1alpha1. The contents of this list are drained at the end of the workflow."]
    #[serde(
        rename = "persistentVolumeClaims",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub persistent_volume_claims: Vec<IoK8sApiCoreV1Volume>,
    #[doc = "Phase a simple, high-level summary of where the workflow is in its lifecycle. Will be \"\" (Unknown), \"Pending\", or \"Running\" before the workflow is completed, and \"Succeeded\", \"Failed\" or \"Error\" once the workflow has completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[doc = "Progress to completion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[doc = "ResourcesDuration is the total for the workflow"]
    #[serde(
        rename = "resourcesDuration",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub resources_duration: std::collections::HashMap<String, i64>,
    #[doc = "Time at which this workflow started"]
    #[serde(rename = "startedAt", default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "StoredTemplates is a mapping between a template ref and the node's status."]
    #[serde(
        rename = "storedTemplates",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub stored_templates: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1Template>,
    #[doc = "StoredWorkflowSpec stores the WorkflowTemplate spec for future execution."]
    #[serde(
        rename = "storedWorkflowTemplateSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stored_workflow_template_spec: Option<IoArgoprojWorkflowV1alpha1WorkflowSpec>,
    #[doc = "Synchronization stores the status of synchronization locks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<IoArgoprojWorkflowV1alpha1SynchronizationStatus>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowStatus> for IoArgoprojWorkflowV1alpha1WorkflowStatus {
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowStatus) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowStep is a reference to a template to execute in a series of step"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowStep {
    #[doc = "Arguments hold arguments to the template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<IoArgoprojWorkflowV1alpha1Arguments>,
    #[doc = "ContinueOn makes argo to proceed with the following step even if this step fails. Errors and Failed states can be specified"]
    #[serde(
        rename = "continueOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub continue_on: Option<IoArgoprojWorkflowV1alpha1ContinueOn>,
    #[doc = "Hooks holds the lifecycle hook which is invoked at lifecycle of step, irrespective of the success, failure, or error status of the primary step"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub hooks: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1LifecycleHook>,
    #[doc = "Inline is the template. Template must be empty if this is declared (and vice-versa)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<IoArgoprojWorkflowV1alpha1Template>,
    #[doc = "Name of the step"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "OnExit is a template reference which is invoked at the end of the template, irrespective of the success, failure, or error of the primary template. DEPRECATED: Use Hooks[exit].Template instead."]
    #[serde(rename = "onExit", default, skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<String>,
    #[doc = "Template is the name of the template to execute as the step"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "TemplateRef is the reference to the template resource to execute as the step."]
    #[serde(
        rename = "templateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_ref: Option<IoArgoprojWorkflowV1alpha1TemplateRef>,
    #[doc = "When is an expression in which the step should conditionally execute"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    #[doc = "WithItems expands a step into multiple parallel steps from the items in the list"]
    #[serde(rename = "withItems", default, skip_serializing_if = "Vec::is_empty")]
    pub with_items: Vec<IoArgoprojWorkflowV1alpha1Item>,
    #[doc = "WithParam expands a step into multiple parallel steps from the value in the parameter, which is expected to be a JSON list."]
    #[serde(rename = "withParam", default, skip_serializing_if = "Option::is_none")]
    pub with_param: Option<String>,
    #[doc = "WithSequence expands a step into a numeric sequence"]
    #[serde(
        rename = "withSequence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub with_sequence: Option<IoArgoprojWorkflowV1alpha1Sequence>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowStep> for IoArgoprojWorkflowV1alpha1WorkflowStep {
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowStep) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowStopRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "nodeFieldSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_field_selector: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowStopRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowStopRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowStopRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowSubmitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "resourceKind",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_kind: Option<String>,
    #[serde(
        rename = "resourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_name: Option<String>,
    #[serde(
        rename = "submitOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub submit_options: Option<IoArgoprojWorkflowV1alpha1SubmitOpts>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowSubmitRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowSubmitRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowSubmitRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowSuspendRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowSuspendRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowSuspendRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowSuspendRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTaskSetSpec {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub tasks: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1Template>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTaskSetSpec>
    for IoArgoprojWorkflowV1alpha1WorkflowTaskSetSpec
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTaskSetSpec) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTaskSetStatus {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub nodes: std::collections::HashMap<String, IoArgoprojWorkflowV1alpha1NodeResult>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTaskSetStatus>
    for IoArgoprojWorkflowV1alpha1WorkflowTaskSetStatus
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTaskSetStatus) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowTemplate is the definition of a workflow template resource"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplate {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    pub spec: IoArgoprojWorkflowV1alpha1WorkflowSpec,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplate>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplate
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplate) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateCreateRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojWorkflowV1alpha1WorkflowTemplate>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplateCreateRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateCreateRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplateCreateRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse(
    pub serde_json::Map<String, serde_json::Value>,
);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse>
    for serde_json::Map<String, serde_json::Value>
{
    fn from(value: IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateDeleteResponse
{
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojWorkflowV1alpha1WorkflowTemplate>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplateLintRequest) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowTemplateList is list of WorkflowTemplate resources"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateList {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    pub items: Vec<IoArgoprojWorkflowV1alpha1WorkflowTemplate>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub metadata: IoK8sApimachineryPkgApisMetaV1ListMeta,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplateList>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateList
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplateList) -> Self {
        value.clone()
    }
}
#[doc = "WorkflowTemplateRef is a reference to a WorkflowTemplate resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateRef {
    #[doc = "ClusterScope indicates the referred template is cluster scoped (i.e. a ClusterWorkflowTemplate)."]
    #[serde(
        rename = "clusterScope",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_scope: Option<bool>,
    #[doc = "Name is the resource name of the workflow template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplateRef>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateRef
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplateRef) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTemplateUpdateRequest {
    #[doc = "DEPRECATED: This field is ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<IoArgoprojWorkflowV1alpha1WorkflowTemplate>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTemplateUpdateRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowTemplateUpdateRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTemplateUpdateRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowTerminateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowTerminateRequest>
    for IoArgoprojWorkflowV1alpha1WorkflowTerminateRequest
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowTerminateRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1WorkflowWatchEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<IoArgoprojWorkflowV1alpha1Workflow>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoArgoprojWorkflowV1alpha1WorkflowWatchEvent>
    for IoArgoprojWorkflowV1alpha1WorkflowWatchEvent
{
    fn from(value: &IoArgoprojWorkflowV1alpha1WorkflowWatchEvent) -> Self {
        value.clone()
    }
}
#[doc = "ZipStrategy will unzip zipped input artifacts"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoArgoprojWorkflowV1alpha1ZipStrategy(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IoArgoprojWorkflowV1alpha1ZipStrategy {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoArgoprojWorkflowV1alpha1ZipStrategy> for serde_json::Map<String, serde_json::Value> {
    fn from(value: IoArgoprojWorkflowV1alpha1ZipStrategy) -> Self {
        value.0
    }
}
impl From<&IoArgoprojWorkflowV1alpha1ZipStrategy> for IoArgoprojWorkflowV1alpha1ZipStrategy {
    fn from(value: &IoArgoprojWorkflowV1alpha1ZipStrategy) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for IoArgoprojWorkflowV1alpha1ZipStrategy {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Affinity is a group of affinity scheduling rules."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Affinity {
    #[doc = "Describes node affinity scheduling rules for the pod."]
    #[serde(
        rename = "nodeAffinity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_affinity: Option<IoK8sApiCoreV1NodeAffinity>,
    #[doc = "Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s))."]
    #[serde(
        rename = "podAffinity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_affinity: Option<IoK8sApiCoreV1PodAffinity>,
    #[doc = "Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s))."]
    #[serde(
        rename = "podAntiAffinity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_anti_affinity: Option<IoK8sApiCoreV1PodAntiAffinity>,
}
impl From<&IoK8sApiCoreV1Affinity> for IoK8sApiCoreV1Affinity {
    fn from(value: &IoK8sApiCoreV1Affinity) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Persistent Disk resource in AWS.\n\nAn AWS EBS disk must exist before mounting to a container. The disk must also be in the same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS EBS volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource {
    #[doc = "Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore"]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as \"1\". Similarly, the volume partition for /dev/sda is \"0\" (or you can leave the property empty)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i64>,
    #[doc = "Specify \"true\" to force and set the ReadOnly property in VolumeMounts to \"true\". If omitted, the default is \"false\". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore"]
    #[serde(rename = "volumeID")]
    pub volume_id: String,
}
impl From<&IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource>
    for IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource
{
    fn from(value: &IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1AzureDiskVolumeSource {
    #[doc = "Host Caching mode: None, Read Only, Read Write."]
    #[serde(
        rename = "cachingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caching_mode: Option<String>,
    #[doc = "The Name of the data disk in the blob storage"]
    #[serde(rename = "diskName")]
    pub disk_name: String,
    #[doc = "The URI the data disk in the blob storage"]
    #[serde(rename = "diskURI")]
    pub disk_uri: String,
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl From<&IoK8sApiCoreV1AzureDiskVolumeSource> for IoK8sApiCoreV1AzureDiskVolumeSource {
    fn from(value: &IoK8sApiCoreV1AzureDiskVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "AzureFile represents an Azure File Service mount on the host and bind mount to the pod."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1AzureFileVolumeSource {
    #[doc = "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "the name of secret that contains Azure Storage Account Name and Key"]
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[doc = "Share Name"]
    #[serde(rename = "shareName")]
    pub share_name: String,
}
impl From<&IoK8sApiCoreV1AzureFileVolumeSource> for IoK8sApiCoreV1AzureFileVolumeSource {
    fn from(value: &IoK8sApiCoreV1AzureFileVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Adds and removes POSIX capabilities from running containers."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Capabilities {
    #[doc = "Added capabilities"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<String>,
    #[doc = "Removed capabilities"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drop: Vec<String>,
}
impl From<&IoK8sApiCoreV1Capabilities> for IoK8sApiCoreV1Capabilities {
    fn from(value: &IoK8sApiCoreV1Capabilities) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1CephFsVolumeSource {
    #[doc = "Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it"]
    pub monitors: Vec<String>,
    #[doc = "Optional: Used as the mounted root, rather than the full Ceph tree, default is /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it"]
    #[serde(
        rename = "secretFile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_file: Option<String>,
    #[doc = "Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it"]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl From<&IoK8sApiCoreV1CephFsVolumeSource> for IoK8sApiCoreV1CephFsVolumeSource {
    fn from(value: &IoK8sApiCoreV1CephFsVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1CinderVolumeSource {
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md"]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Optional: points to a secret object containing parameters used to connect to OpenStack."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "volume id used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md"]
    #[serde(rename = "volumeID")]
    pub volume_id: String,
}
impl From<&IoK8sApiCoreV1CinderVolumeSource> for IoK8sApiCoreV1CinderVolumeSource {
    fn from(value: &IoK8sApiCoreV1CinderVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.\n\nThe contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ConfigMapEnvSource {
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the ConfigMap must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1ConfigMapEnvSource> for IoK8sApiCoreV1ConfigMapEnvSource {
    fn from(value: &IoK8sApiCoreV1ConfigMapEnvSource) -> Self {
        value.clone()
    }
}
#[doc = "Selects a key from a ConfigMap."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ConfigMapKeySelector {
    #[doc = "The key to select."]
    pub key: String,
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the ConfigMap or its key must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1ConfigMapKeySelector> for IoK8sApiCoreV1ConfigMapKeySelector {
    fn from(value: &IoK8sApiCoreV1ConfigMapKeySelector) -> Self {
        value.clone()
    }
}
#[doc = "Adapts a ConfigMap into a projected volume.\n\nThe contents of the target ConfigMap's Data field will be presented in a projected volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. Note that this is identical to a configmap volume source without the default mode."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ConfigMapProjection {
    #[doc = "If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoK8sApiCoreV1KeyToPath>,
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the ConfigMap or its keys must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1ConfigMapProjection> for IoK8sApiCoreV1ConfigMapProjection {
    fn from(value: &IoK8sApiCoreV1ConfigMapProjection) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1ConfigMapVolumeSourceBuilder as ConfigMapVolumeSourceBuilder;
pub use IoK8sApiCoreV1ConfigMapVolumeSourceBuilderError as ConfigMapVolumeSourceBuilderError;

#[doc = "Adapts a ConfigMap into a volume.\n\nThe contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. ConfigMap volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoK8sApiCoreV1ConfigMapVolumeSource {
    #[doc = "Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i64>,
    #[doc = "If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoK8sApiCoreV1KeyToPath>,
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the ConfigMap or its keys must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1ConfigMapVolumeSource> for IoK8sApiCoreV1ConfigMapVolumeSource {
    fn from(value: &IoK8sApiCoreV1ConfigMapVolumeSource) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1ContainerBuilder as ContainerBuilder;
pub use IoK8sApiCoreV1ContainerBuilderError as ContainerBuilderError;

#[doc = "A single application container that you want to run within a pod."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoK8sApiCoreV1Container {
    #[doc = "Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[doc = "Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "List of environment variables to set in the container. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<IoK8sApiCoreV1EnvVar>,
    #[doc = "List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated."]
    #[serde(rename = "envFrom", default, skip_serializing_if = "Vec::is_empty")]
    pub env_from: Vec<IoK8sApiCoreV1EnvFromSource>,
    #[doc = "Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets."]
    pub image: String,
    #[doc = "Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images\n\nPossible enum values:\n - `\"Always\"` means that kubelet always attempts to pull the latest image. Container will fail If the pull fails.\n - `\"IfNotPresent\"` means that kubelet pulls if the image isn't present on disk. Container will fail if the image isn't present and the pull fails.\n - `\"Never\"` means that kubelet never pulls an image, but only uses a local image. Container will fail if the image isn't present"]
    #[serde(
        rename = "imagePullPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_pull_policy: Option<IoK8sApiCoreV1ContainerImagePullPolicy>,
    #[doc = "Actions that the management system should take in response to container lifecycle events. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<IoK8sApiCoreV1Lifecycle>,
    #[doc = "Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "livenessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub liveness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<IoK8sApiCoreV1ContainerPort>,
    #[doc = "Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "readinessProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub readiness_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<IoK8sApiCoreV1ResourceRequirements>,
    #[doc = "SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/"]
    #[serde(
        rename = "securityContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_context: Option<IoK8sApiCoreV1SecurityContext>,
    #[doc = "StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "startupProbe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_probe: Option<IoK8sApiCoreV1Probe>,
    #[doc = "Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    #[doc = "Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false"]
    #[serde(rename = "stdinOnce", default, skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    #[doc = "Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated."]
    #[serde(
        rename = "terminationMessagePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_path: Option<String>,
    #[doc = "Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.\n\nPossible enum values:\n - `\"FallbackToLogsOnError\"` will read the most recent contents of the container logs for the container status message when the container exits with an error and the terminationMessagePath has no contents.\n - `\"File\"` is the default behavior and will set the container status message to the contents of the container's terminationMessagePath when the container exits."]
    #[serde(
        rename = "terminationMessagePolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_message_policy: Option<IoK8sApiCoreV1ContainerTerminationMessagePolicy>,
    #[doc = "Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[doc = "volumeDevices is the list of block devices to be used by the container."]
    #[serde(
        rename = "volumeDevices",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_devices: Vec<IoK8sApiCoreV1VolumeDevice>,
    #[doc = "Pod volumes to mount into the container's filesystem. Cannot be updated."]
    #[serde(
        rename = "volumeMounts",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_mounts: Vec<IoK8sApiCoreV1VolumeMount>,
    #[doc = "Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated."]
    #[serde(
        rename = "workingDir",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_dir: Option<String>,
}
impl From<&IoK8sApiCoreV1Container> for IoK8sApiCoreV1Container {
    fn from(value: &IoK8sApiCoreV1Container) -> Self {
        value.clone()
    }
}
#[doc = "Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images\n\nPossible enum values:\n - `\"Always\"` means that kubelet always attempts to pull the latest image. Container will fail If the pull fails.\n - `\"IfNotPresent\"` means that kubelet pulls if the image isn't present on disk. Container will fail if the image isn't present and the pull fails.\n - `\"Never\"` means that kubelet never pulls an image, but only uses a local image. Container will fail if the image isn't present"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1ContainerImagePullPolicy {
    Always,
    IfNotPresent,
    Never,
}
impl From<&IoK8sApiCoreV1ContainerImagePullPolicy> for IoK8sApiCoreV1ContainerImagePullPolicy {
    fn from(value: &IoK8sApiCoreV1ContainerImagePullPolicy) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1ContainerImagePullPolicy {
    fn to_string(&self) -> String {
        match *self {
            Self::Always => "Always".to_string(),
            Self::IfNotPresent => "IfNotPresent".to_string(),
            Self::Never => "Never".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1ContainerImagePullPolicy {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Always" => Ok(Self::Always),
            "IfNotPresent" => Ok(Self::IfNotPresent),
            "Never" => Ok(Self::Never),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1ContainerImagePullPolicy {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1ContainerImagePullPolicy {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1ContainerImagePullPolicy {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "ContainerPort represents a network port in a single container."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ContainerPort {
    #[doc = "Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536."]
    #[serde(rename = "containerPort")]
    pub container_port: i64,
    #[doc = "What host IP to bind the external port to."]
    #[serde(rename = "hostIP", default, skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    #[doc = "Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this."]
    #[serde(rename = "hostPort", default, skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    #[doc = "If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\".\n\nPossible enum values:\n - `\"SCTP\"` is the SCTP protocol.\n - `\"TCP\"` is the TCP protocol.\n - `\"UDP\"` is the UDP protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<IoK8sApiCoreV1ContainerPortProtocol>,
}
impl From<&IoK8sApiCoreV1ContainerPort> for IoK8sApiCoreV1ContainerPort {
    fn from(value: &IoK8sApiCoreV1ContainerPort) -> Self {
        value.clone()
    }
}
#[doc = "Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\".\n\nPossible enum values:\n - `\"SCTP\"` is the SCTP protocol.\n - `\"TCP\"` is the TCP protocol.\n - `\"UDP\"` is the UDP protocol."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1ContainerPortProtocol {
    #[serde(rename = "SCTP")]
    Sctp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}
impl From<&IoK8sApiCoreV1ContainerPortProtocol> for IoK8sApiCoreV1ContainerPortProtocol {
    fn from(value: &IoK8sApiCoreV1ContainerPortProtocol) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1ContainerPortProtocol {
    fn to_string(&self) -> String {
        match *self {
            Self::Sctp => "SCTP".to_string(),
            Self::Tcp => "TCP".to_string(),
            Self::Udp => "UDP".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1ContainerPortProtocol {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "SCTP" => Ok(Self::Sctp),
            "TCP" => Ok(Self::Tcp),
            "UDP" => Ok(Self::Udp),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1ContainerPortProtocol {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1ContainerPortProtocol {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1ContainerPortProtocol {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.\n\nPossible enum values:\n - `\"FallbackToLogsOnError\"` will read the most recent contents of the container logs for the container status message when the container exits with an error and the terminationMessagePath has no contents.\n - `\"File\"` is the default behavior and will set the container status message to the contents of the container's terminationMessagePath when the container exits."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1ContainerTerminationMessagePolicy {
    FallbackToLogsOnError,
    File,
}
impl From<&IoK8sApiCoreV1ContainerTerminationMessagePolicy>
    for IoK8sApiCoreV1ContainerTerminationMessagePolicy
{
    fn from(value: &IoK8sApiCoreV1ContainerTerminationMessagePolicy) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1ContainerTerminationMessagePolicy {
    fn to_string(&self) -> String {
        match *self {
            Self::FallbackToLogsOnError => "FallbackToLogsOnError".to_string(),
            Self::File => "File".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1ContainerTerminationMessagePolicy {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "FallbackToLogsOnError" => Ok(Self::FallbackToLogsOnError),
            "File" => Ok(Self::File),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1ContainerTerminationMessagePolicy {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1ContainerTerminationMessagePolicy {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1ContainerTerminationMessagePolicy {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Represents a source location of a volume to mount, managed by an external CSI driver"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1CsiVolumeSource {
    #[doc = "Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster."]
    pub driver: String,
    #[doc = "Filesystem type to mount. Ex. \"ext4\", \"xfs\", \"ntfs\". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed."]
    #[serde(
        rename = "nodePublishSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub node_publish_secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "Specifies a read-only configuration for the volume. Defaults to false (read/write)."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values."]
    #[serde(
        rename = "volumeAttributes",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub volume_attributes: std::collections::HashMap<String, String>,
}
impl From<&IoK8sApiCoreV1CsiVolumeSource> for IoK8sApiCoreV1CsiVolumeSource {
    fn from(value: &IoK8sApiCoreV1CsiVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1DownwardApiProjection {
    #[doc = "Items is a list of DownwardAPIVolume file"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoK8sApiCoreV1DownwardApiVolumeFile>,
}
impl From<&IoK8sApiCoreV1DownwardApiProjection> for IoK8sApiCoreV1DownwardApiProjection {
    fn from(value: &IoK8sApiCoreV1DownwardApiProjection) -> Self {
        value.clone()
    }
}
#[doc = "DownwardAPIVolumeFile represents information to create the file containing the pod field"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1DownwardApiVolumeFile {
    #[doc = "Required: Selects a field of the pod: only annotations, labels, name and namespace are supported."]
    #[serde(rename = "fieldRef", default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<IoK8sApiCoreV1ObjectFieldSelector>,
    #[doc = "Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[doc = "Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'"]
    pub path: String,
    #[doc = "Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported."]
    #[serde(
        rename = "resourceFieldRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_field_ref: Option<IoK8sApiCoreV1ResourceFieldSelector>,
}
impl From<&IoK8sApiCoreV1DownwardApiVolumeFile> for IoK8sApiCoreV1DownwardApiVolumeFile {
    fn from(value: &IoK8sApiCoreV1DownwardApiVolumeFile) -> Self {
        value.clone()
    }
}
#[doc = "DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1DownwardApiVolumeSource {
    #[doc = "Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i64>,
    #[doc = "Items is a list of downward API volume file"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoK8sApiCoreV1DownwardApiVolumeFile>,
}
impl From<&IoK8sApiCoreV1DownwardApiVolumeSource> for IoK8sApiCoreV1DownwardApiVolumeSource {
    fn from(value: &IoK8sApiCoreV1DownwardApiVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct IoK8sApiCoreV1EmptyDirVolumeSource {
    #[doc = "What type of storage medium should back this directory. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[doc = "Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir"]
    #[serde(rename = "sizeLimit", default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<IoK8sApimachineryPkgApiResourceQuantity>,
}
impl From<&IoK8sApiCoreV1EmptyDirVolumeSource> for IoK8sApiCoreV1EmptyDirVolumeSource {
    fn from(value: &IoK8sApiCoreV1EmptyDirVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "EnvFromSource represents the source of a set of ConfigMaps"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1EnvFromSource {
    #[doc = "The ConfigMap to select from"]
    #[serde(
        rename = "configMapRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_map_ref: Option<IoK8sApiCoreV1ConfigMapEnvSource>,
    #[doc = "An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "The Secret to select from"]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1SecretEnvSource>,
}
impl From<&IoK8sApiCoreV1EnvFromSource> for IoK8sApiCoreV1EnvFromSource {
    fn from(value: &IoK8sApiCoreV1EnvFromSource) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1EnvVarBuilder as EnvVarBuilder;
pub use IoK8sApiCoreV1EnvVarBuilderError as EnvVarBuilderError;

#[doc = "EnvVar represents an environment variable present in a Container."]
#[derive(Clone, Debug, Deserialize, Serialize, Builder, Default)]
pub struct IoK8sApiCoreV1EnvVar {
    #[doc = "Name of the environment variable. Must be a C_IDENTIFIER."]
    pub name: String,
    #[doc = "Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. \"$$(VAR_NAME)\" will produce the string literal \"$(VAR_NAME)\". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to \"\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Source for the environment variable's value. Cannot be used if value is not empty."]
    #[serde(rename = "valueFrom", default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<IoK8sApiCoreV1EnvVarSource>,
}
impl From<&IoK8sApiCoreV1EnvVar> for IoK8sApiCoreV1EnvVar {
    fn from(value: &IoK8sApiCoreV1EnvVar) -> Self {
        value.clone()
    }
}
pub use IoK8sApiCoreV1EnvVarSourceBuilder as EnvVarSourceBuilder;
pub use IoK8sApiCoreV1EnvVarSourceBuilderError as EnvVarSourceBuilderError;

#[doc = "EnvVarSource represents a source for the value of an EnvVar."]
#[derive(Clone, Debug, Deserialize, Serialize, Builder, Default)]
pub struct IoK8sApiCoreV1EnvVarSource {
    #[doc = "Selects a key of a ConfigMap."]
    #[serde(
        rename = "configMapKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_map_key_ref: Option<IoK8sApiCoreV1ConfigMapKeySelector>,
    #[doc = "Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs."]
    #[serde(rename = "fieldRef", default, skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<IoK8sApiCoreV1ObjectFieldSelector>,
    #[doc = "Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported."]
    #[serde(
        rename = "resourceFieldRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_field_ref: Option<IoK8sApiCoreV1ResourceFieldSelector>,
    #[doc = "Selects a key of a secret in the pod's namespace"]
    #[serde(
        rename = "secretKeyRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_ref: Option<IoK8sApiCoreV1SecretKeySelector>,
}
impl From<&IoK8sApiCoreV1EnvVarSource> for IoK8sApiCoreV1EnvVarSource {
    fn from(value: &IoK8sApiCoreV1EnvVarSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents an ephemeral volume that is handled by a normal storage driver."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1EphemeralVolumeSource {
    #[doc = "Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).\n\nAn existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.\n\nThis field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.\n\nRequired, must not be nil."]
    #[serde(
        rename = "volumeClaimTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_claim_template: Option<IoK8sApiCoreV1PersistentVolumeClaimTemplate>,
}
impl From<&IoK8sApiCoreV1EphemeralVolumeSource> for IoK8sApiCoreV1EphemeralVolumeSource {
    fn from(value: &IoK8sApiCoreV1EphemeralVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Event is a report of an event somewhere in the cluster.  Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Event {
    #[doc = "What action was taken/failed regarding to the Regarding object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "The number of times this event has occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Time when this Event was first observed."]
    #[serde(rename = "eventTime", default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<IoK8sApimachineryPkgApisMetaV1MicroTime>,
    #[doc = "The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)"]
    #[serde(
        rename = "firstTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub first_timestamp: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "The object that this event is about."]
    #[serde(rename = "involvedObject")]
    pub involved_object: IoK8sApiCoreV1ObjectReference,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The time at which the most recent occurrence of this event was recorded."]
    #[serde(
        rename = "lastTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_timestamp: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "A human-readable description of the status of this operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata"]
    pub metadata: IoK8sApimachineryPkgApisMetaV1ObjectMeta,
    #[doc = "This should be a short, machine understandable string that gives the reason for the transition into the object's current status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Optional secondary object for more complex actions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<IoK8sApiCoreV1ObjectReference>,
    #[doc = "Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`."]
    #[serde(
        rename = "reportingComponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reporting_component: Option<String>,
    #[doc = "ID of the controller instance, e.g. `kubelet-xyzf`."]
    #[serde(
        rename = "reportingInstance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reporting_instance: Option<String>,
    #[doc = "Data about the Event series this event represents or nil if it's a singleton Event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<IoK8sApiCoreV1EventSeries>,
    #[doc = "The component reporting this event. Should be a short machine understandable string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<IoK8sApiCoreV1EventSource>,
    #[doc = "Type of this event (Normal, Warning), new types could be added in the future"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoK8sApiCoreV1Event> for IoK8sApiCoreV1Event {
    fn from(value: &IoK8sApiCoreV1Event) -> Self {
        value.clone()
    }
}
#[doc = "EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1EventSeries {
    #[doc = "Number of occurrences in this series up to the last heartbeat time"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Time of the last occurrence observed"]
    #[serde(
        rename = "lastObservedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_observed_time: Option<IoK8sApimachineryPkgApisMetaV1MicroTime>,
}
impl From<&IoK8sApiCoreV1EventSeries> for IoK8sApiCoreV1EventSeries {
    fn from(value: &IoK8sApiCoreV1EventSeries) -> Self {
        value.clone()
    }
}
#[doc = "EventSource contains information for an event."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1EventSource {
    #[doc = "Component from which the event is generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[doc = "Node name on which the event is generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}
impl From<&IoK8sApiCoreV1EventSource> for IoK8sApiCoreV1EventSource {
    fn from(value: &IoK8sApiCoreV1EventSource) -> Self {
        value.clone()
    }
}
#[doc = "ExecAction describes a \"run in container\" action."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ExecAction {
    #[doc = "Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}
impl From<&IoK8sApiCoreV1ExecAction> for IoK8sApiCoreV1ExecAction {
    fn from(value: &IoK8sApiCoreV1ExecAction) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1FcVolumeSource {
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Optional: FC target lun number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i64>,
    #[doc = "Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Optional: FC target worldwide names (WWNs)"]
    #[serde(rename = "targetWWNs", default, skip_serializing_if = "Vec::is_empty")]
    pub target_ww_ns: Vec<String>,
    #[doc = "Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wwids: Vec<String>,
}
impl From<&IoK8sApiCoreV1FcVolumeSource> for IoK8sApiCoreV1FcVolumeSource {
    fn from(value: &IoK8sApiCoreV1FcVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1FlexVolumeSource {
    #[doc = "Driver is the name of the driver to use for this volume."]
    pub driver: String,
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default filesystem depends on FlexVolume script."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Optional: Extra command options if any."]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub options: std::collections::HashMap<String, String>,
    #[doc = "Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
}
impl From<&IoK8sApiCoreV1FlexVolumeSource> for IoK8sApiCoreV1FlexVolumeSource {
    fn from(value: &IoK8sApiCoreV1FlexVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1FlockerVolumeSource {
    #[doc = "Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated"]
    #[serde(
        rename = "datasetName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dataset_name: Option<String>,
    #[doc = "UUID of the dataset. This is unique identifier of a Flocker dataset"]
    #[serde(
        rename = "datasetUUID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dataset_uuid: Option<String>,
}
impl From<&IoK8sApiCoreV1FlockerVolumeSource> for IoK8sApiCoreV1FlockerVolumeSource {
    fn from(value: &IoK8sApiCoreV1FlockerVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Persistent Disk resource in Google Compute Engine.\n\nA GCE PD must exist before mounting to a container. The disk must also be in the same GCE project and zone as the kubelet. A GCE PD can only be mounted as read/write once or read-only many times. GCE PDs support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1GcePersistentDiskVolumeSource {
    #[doc = "Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as \"1\". Similarly, the volume partition for /dev/sda is \"0\" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i64>,
    #[doc = "Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"]
    #[serde(rename = "pdName")]
    pub pd_name: String,
    #[doc = "ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl From<&IoK8sApiCoreV1GcePersistentDiskVolumeSource>
    for IoK8sApiCoreV1GcePersistentDiskVolumeSource
{
    fn from(value: &IoK8sApiCoreV1GcePersistentDiskVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.\n\nDEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1GitRepoVolumeSource {
    #[doc = "Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[doc = "Repository URL"]
    pub repository: String,
    #[doc = "Commit hash for the specified revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
impl From<&IoK8sApiCoreV1GitRepoVolumeSource> for IoK8sApiCoreV1GitRepoVolumeSource {
    fn from(value: &IoK8sApiCoreV1GitRepoVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1GlusterfsVolumeSource {
    #[doc = "EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod"]
    pub endpoints: String,
    #[doc = "Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod"]
    pub path: String,
    #[doc = "ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl From<&IoK8sApiCoreV1GlusterfsVolumeSource> for IoK8sApiCoreV1GlusterfsVolumeSource {
    fn from(value: &IoK8sApiCoreV1GlusterfsVolumeSource) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1GrpcAction {
    #[doc = "Port number of the gRPC service. Number must be in the range 1 to 65535."]
    pub port: i64,
    #[doc = "Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\n\nIf this is not specified, the default behavior is defined by gRPC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}
impl From<&IoK8sApiCoreV1GrpcAction> for IoK8sApiCoreV1GrpcAction {
    fn from(value: &IoK8sApiCoreV1GrpcAction) -> Self {
        value.clone()
    }
}
#[doc = "HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1HostAlias {
    #[doc = "Hostnames for the above IP address."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hostnames: Vec<String>,
    #[doc = "IP address of the host file entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}
impl From<&IoK8sApiCoreV1HostAlias> for IoK8sApiCoreV1HostAlias {
    fn from(value: &IoK8sApiCoreV1HostAlias) -> Self {
        value.clone()
    }
}
#[doc = "Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1HostPathVolumeSource {
    #[doc = "Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath"]
    pub path: String,
    #[doc = "Type for HostPath Volume Defaults to \"\" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&IoK8sApiCoreV1HostPathVolumeSource> for IoK8sApiCoreV1HostPathVolumeSource {
    fn from(value: &IoK8sApiCoreV1HostPathVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "HTTPGetAction describes an action based on HTTP Get requests."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1HttpGetAction {
    #[doc = "Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "Custom headers to set in the request. HTTP allows repeated headers."]
    #[serde(rename = "httpHeaders", default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<IoK8sApiCoreV1HttpHeader>,
    #[doc = "Path to access on the HTTP server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME."]
    pub port: IoK8sApimachineryPkgUtilIntstrIntOrString,
    #[doc = "Scheme to use for connecting to the host. Defaults to HTTP.\n\nPossible enum values:\n - `\"HTTP\"` means that the scheme used will be http://\n - `\"HTTPS\"` means that the scheme used will be https://"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<IoK8sApiCoreV1HttpGetActionScheme>,
}
impl From<&IoK8sApiCoreV1HttpGetAction> for IoK8sApiCoreV1HttpGetAction {
    fn from(value: &IoK8sApiCoreV1HttpGetAction) -> Self {
        value.clone()
    }
}
#[doc = "Scheme to use for connecting to the host. Defaults to HTTP.\n\nPossible enum values:\n - `\"HTTP\"` means that the scheme used will be http://\n - `\"HTTPS\"` means that the scheme used will be https://"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1HttpGetActionScheme {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTPS")]
    Https,
}
impl From<&IoK8sApiCoreV1HttpGetActionScheme> for IoK8sApiCoreV1HttpGetActionScheme {
    fn from(value: &IoK8sApiCoreV1HttpGetActionScheme) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1HttpGetActionScheme {
    fn to_string(&self) -> String {
        match *self {
            Self::Http => "HTTP".to_string(),
            Self::Https => "HTTPS".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1HttpGetActionScheme {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "HTTP" => Ok(Self::Http),
            "HTTPS" => Ok(Self::Https),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1HttpGetActionScheme {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1HttpGetActionScheme {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1HttpGetActionScheme {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "HTTPHeader describes a custom header to be used in HTTP probes"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1HttpHeader {
    #[doc = "The header field name"]
    pub name: String,
    #[doc = "The header field value"]
    pub value: String,
}
impl From<&IoK8sApiCoreV1HttpHeader> for IoK8sApiCoreV1HttpHeader {
    fn from(value: &IoK8sApiCoreV1HttpHeader) -> Self {
        value.clone()
    }
}
#[doc = "Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1IscsiVolumeSource {
    #[doc = "whether support iSCSI Discovery CHAP authentication"]
    #[serde(
        rename = "chapAuthDiscovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chap_auth_discovery: Option<bool>,
    #[doc = "whether support iSCSI Session CHAP authentication"]
    #[serde(
        rename = "chapAuthSession",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chap_auth_session: Option<bool>,
    #[doc = "Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi"]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection."]
    #[serde(
        rename = "initiatorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator_name: Option<String>,
    #[doc = "Target iSCSI Qualified Name."]
    pub iqn: String,
    #[doc = "iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp)."]
    #[serde(
        rename = "iscsiInterface",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iscsi_interface: Option<String>,
    #[doc = "iSCSI Target Lun number."]
    pub lun: i64,
    #[doc = "iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub portals: Vec<String>,
    #[doc = "ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "CHAP Secret for iSCSI target and initiator authentication"]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260)."]
    #[serde(rename = "targetPortal")]
    pub target_portal: String,
}
impl From<&IoK8sApiCoreV1IscsiVolumeSource> for IoK8sApiCoreV1IscsiVolumeSource {
    fn from(value: &IoK8sApiCoreV1IscsiVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Maps a string key to a path within a volume."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1KeyToPath {
    #[doc = "The key to project."]
    pub key: String,
    #[doc = "Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[doc = "The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'."]
    pub path: String,
}
impl From<&IoK8sApiCoreV1KeyToPath> for IoK8sApiCoreV1KeyToPath {
    fn from(value: &IoK8sApiCoreV1KeyToPath) -> Self {
        value.clone()
    }
}
#[doc = "Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Lifecycle {
    #[doc = "PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks"]
    #[serde(rename = "postStart", default, skip_serializing_if = "Option::is_none")]
    pub post_start: Option<IoK8sApiCoreV1LifecycleHandler>,
    #[doc = "PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks"]
    #[serde(rename = "preStop", default, skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<IoK8sApiCoreV1LifecycleHandler>,
}
impl From<&IoK8sApiCoreV1Lifecycle> for IoK8sApiCoreV1Lifecycle {
    fn from(value: &IoK8sApiCoreV1Lifecycle) -> Self {
        value.clone()
    }
}
#[doc = "LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1LifecycleHandler {
    #[doc = "Exec specifies the action to take."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<IoK8sApiCoreV1ExecAction>,
    #[doc = "HTTPGet specifies the http request to perform."]
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<IoK8sApiCoreV1HttpGetAction>,
    #[doc = "Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified."]
    #[serde(rename = "tcpSocket", default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<IoK8sApiCoreV1TcpSocketAction>,
}
impl From<&IoK8sApiCoreV1LifecycleHandler> for IoK8sApiCoreV1LifecycleHandler {
    fn from(value: &IoK8sApiCoreV1LifecycleHandler) -> Self {
        value.clone()
    }
}
#[doc = "LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1LocalObjectReference {
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&IoK8sApiCoreV1LocalObjectReference> for IoK8sApiCoreV1LocalObjectReference {
    fn from(value: &IoK8sApiCoreV1LocalObjectReference) -> Self {
        value.clone()
    }
}
#[doc = "Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support ownership management or SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1NfsVolumeSource {
    #[doc = "Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs"]
    pub path: String,
    #[doc = "ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs"]
    pub server: String,
}
impl From<&IoK8sApiCoreV1NfsVolumeSource> for IoK8sApiCoreV1NfsVolumeSource {
    fn from(value: &IoK8sApiCoreV1NfsVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Node affinity is a group of node affinity scheduling rules."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1NodeAffinity {
    #[doc = "The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding \"weight\" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred."]
    #[serde(
        rename = "preferredDuringSchedulingIgnoredDuringExecution",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub preferred_during_scheduling_ignored_during_execution:
        Vec<IoK8sApiCoreV1PreferredSchedulingTerm>,
    #[doc = "If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node."]
    #[serde(
        rename = "requiredDuringSchedulingIgnoredDuringExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_during_scheduling_ignored_during_execution: Option<IoK8sApiCoreV1NodeSelector>,
}
impl From<&IoK8sApiCoreV1NodeAffinity> for IoK8sApiCoreV1NodeAffinity {
    fn from(value: &IoK8sApiCoreV1NodeAffinity) -> Self {
        value.clone()
    }
}
#[doc = "A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1NodeSelector {
    #[doc = "Required. A list of node selector terms. The terms are ORed."]
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<IoK8sApiCoreV1NodeSelectorTerm>,
}
impl From<&IoK8sApiCoreV1NodeSelector> for IoK8sApiCoreV1NodeSelector {
    fn from(value: &IoK8sApiCoreV1NodeSelector) -> Self {
        value.clone()
    }
}
#[doc = "A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1NodeSelectorRequirement {
    #[doc = "The label key that the selector applies to."]
    pub key: String,
    #[doc = "Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.\n\nPossible enum values:\n - `\"DoesNotExist\"`\n - `\"Exists\"`\n - `\"Gt\"`\n - `\"In\"`\n - `\"Lt\"`\n - `\"NotIn\"`"]
    pub operator: IoK8sApiCoreV1NodeSelectorRequirementOperator,
    #[doc = "An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
impl From<&IoK8sApiCoreV1NodeSelectorRequirement> for IoK8sApiCoreV1NodeSelectorRequirement {
    fn from(value: &IoK8sApiCoreV1NodeSelectorRequirement) -> Self {
        value.clone()
    }
}
#[doc = "Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.\n\nPossible enum values:\n - `\"DoesNotExist\"`\n - `\"Exists\"`\n - `\"Gt\"`\n - `\"In\"`\n - `\"Lt\"`\n - `\"NotIn\"`"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1NodeSelectorRequirementOperator {
    DoesNotExist,
    Exists,
    Gt,
    In,
    Lt,
    NotIn,
}
impl From<&IoK8sApiCoreV1NodeSelectorRequirementOperator>
    for IoK8sApiCoreV1NodeSelectorRequirementOperator
{
    fn from(value: &IoK8sApiCoreV1NodeSelectorRequirementOperator) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1NodeSelectorRequirementOperator {
    fn to_string(&self) -> String {
        match *self {
            Self::DoesNotExist => "DoesNotExist".to_string(),
            Self::Exists => "Exists".to_string(),
            Self::Gt => "Gt".to_string(),
            Self::In => "In".to_string(),
            Self::Lt => "Lt".to_string(),
            Self::NotIn => "NotIn".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1NodeSelectorRequirementOperator {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "DoesNotExist" => Ok(Self::DoesNotExist),
            "Exists" => Ok(Self::Exists),
            "Gt" => Ok(Self::Gt),
            "In" => Ok(Self::In),
            "Lt" => Ok(Self::Lt),
            "NotIn" => Ok(Self::NotIn),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1NodeSelectorRequirementOperator {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1NodeSelectorRequirementOperator {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1NodeSelectorRequirementOperator {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1NodeSelectorTerm {
    #[doc = "A list of node selector requirements by node's labels."]
    #[serde(
        rename = "matchExpressions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_expressions: Vec<IoK8sApiCoreV1NodeSelectorRequirement>,
    #[doc = "A list of node selector requirements by node's fields."]
    #[serde(rename = "matchFields", default, skip_serializing_if = "Vec::is_empty")]
    pub match_fields: Vec<IoK8sApiCoreV1NodeSelectorRequirement>,
}
impl From<&IoK8sApiCoreV1NodeSelectorTerm> for IoK8sApiCoreV1NodeSelectorTerm {
    fn from(value: &IoK8sApiCoreV1NodeSelectorTerm) -> Self {
        value.clone()
    }
}
#[doc = "ObjectFieldSelector selects an APIVersioned field of an object."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ObjectFieldSelector {
    #[doc = "Version of the schema the FieldPath is written in terms of, defaults to \"v1\"."]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Path of the field to select in the specified API version."]
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}
impl From<&IoK8sApiCoreV1ObjectFieldSelector> for IoK8sApiCoreV1ObjectFieldSelector {
    fn from(value: &IoK8sApiCoreV1ObjectFieldSelector) -> Self {
        value.clone()
    }
}
#[doc = "ObjectReference contains enough information to let you inspect or modify the referred object."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ObjectReference {
    #[doc = "API version of the referent."]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: \"spec.containers{name}\" (where \"name\" refers to the name of the container that triggered the event) or if no container name is specified \"spec.containers[2]\" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object."]
    #[serde(rename = "fieldPath", default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    #[doc = "Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency"]
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<String>,
    #[doc = "UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl From<&IoK8sApiCoreV1ObjectReference> for IoK8sApiCoreV1ObjectReference {
    fn from(value: &IoK8sApiCoreV1ObjectReference) -> Self {
        value.clone()
    }
}
#[doc = "PersistentVolumeClaim is a user's request for and claim to a persistent volume"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PersistentVolumeClaim {
    #[doc = "APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources"]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
    #[doc = "Spec defines the desired characteristics of a volume requested by a pod author. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IoK8sApiCoreV1PersistentVolumeClaimSpec>,
    #[doc = "Status represents the current information/status of a persistent volume claim. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IoK8sApiCoreV1PersistentVolumeClaimStatus>,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaim> for IoK8sApiCoreV1PersistentVolumeClaim {
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaim) -> Self {
        value.clone()
    }
}
#[doc = "PersistentVolumeClaimCondition contails details about state of pvc"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PersistentVolumeClaimCondition {
    #[doc = "Last time we probed the condition."]
    #[serde(
        rename = "lastProbeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_probe_time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "Last time the condition transitioned from one status to another."]
    #[serde(
        rename = "lastTransitionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_transition_time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[doc = "Human-readable message indicating details about last transition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"ResizeStarted\" that means the underlying persistent volume is being resized."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[doc = "\n\n\nPossible enum values:\n - `\"FileSystemResizePending\"` - controller resize is finished and a file system resize is pending on node\n - `\"Resizing\"` - a user trigger resize of pvc has been started"]
    #[serde(rename = "type")]
    pub type_: IoK8sApiCoreV1PersistentVolumeClaimConditionType,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimCondition>
    for IoK8sApiCoreV1PersistentVolumeClaimCondition
{
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimCondition) -> Self {
        value.clone()
    }
}
#[doc = "\n\n\nPossible enum values:\n - `\"FileSystemResizePending\"` - controller resize is finished and a file system resize is pending on node\n - `\"Resizing\"` - a user trigger resize of pvc has been started"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1PersistentVolumeClaimConditionType {
    FileSystemResizePending,
    Resizing,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimConditionType>
    for IoK8sApiCoreV1PersistentVolumeClaimConditionType
{
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimConditionType) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1PersistentVolumeClaimConditionType {
    fn to_string(&self) -> String {
        match *self {
            Self::FileSystemResizePending => "FileSystemResizePending".to_string(),
            Self::Resizing => "Resizing".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1PersistentVolumeClaimConditionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "FileSystemResizePending" => Ok(Self::FileSystemResizePending),
            "Resizing" => Ok(Self::Resizing),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1PersistentVolumeClaimConditionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1PersistentVolumeClaimConditionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1PersistentVolumeClaimConditionType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PersistentVolumeClaimSpec {
    #[doc = "AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1"]
    #[serde(rename = "accessModes", default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[doc = "This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field."]
    #[serde(
        rename = "dataSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source: Option<IoK8sApiCoreV1TypedLocalObjectReference>,
    #[doc = "Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef\n  allows any non-core object, as well as PersistentVolumeClaim objects.\n* While DataSource ignores disallowed values (dropping them), DataSourceRef\n  preserves all values, and generates an error if a disallowed value is\n  specified.\n(Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled."]
    #[serde(
        rename = "dataSourceRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_source_ref: Option<IoK8sApiCoreV1TypedLocalObjectReference>,
    #[doc = "Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<IoK8sApiCoreV1ResourceRequirements>,
    #[doc = "A label query over volumes to consider for binding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<IoK8sApimachineryPkgApisMetaV1LabelSelector>,
    #[doc = "Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1"]
    #[serde(
        rename = "storageClassName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_class_name: Option<String>,
    #[doc = "volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec."]
    #[serde(
        rename = "volumeMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_mode: Option<String>,
    #[doc = "VolumeName is the binding reference to the PersistentVolume backing this claim."]
    #[serde(
        rename = "volumeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_name: Option<String>,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimSpec> for IoK8sApiCoreV1PersistentVolumeClaimSpec {
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimSpec) -> Self {
        value.clone()
    }
}
#[doc = "PersistentVolumeClaimStatus is the current status of a persistent volume claim."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PersistentVolumeClaimStatus {
    #[doc = "AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1"]
    #[serde(rename = "accessModes", default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[doc = "The storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature."]
    #[serde(
        rename = "allocatedResources",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub allocated_resources:
        std::collections::HashMap<String, IoK8sApimachineryPkgApiResourceQuantity>,
    #[doc = "Represents the actual resources of the underlying volume."]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub capacity: std::collections::HashMap<String, IoK8sApimachineryPkgApiResourceQuantity>,
    #[doc = "Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<IoK8sApiCoreV1PersistentVolumeClaimCondition>,
    #[doc = "Phase represents the current phase of PersistentVolumeClaim.\n\nPossible enum values:\n - `\"Bound\"` used for PersistentVolumeClaims that are bound\n - `\"Lost\"` used for PersistentVolumeClaims that lost their underlying PersistentVolume. The claim was bound to a PersistentVolume and this volume does not exist any longer and all data on it was lost.\n - `\"Pending\"` used for PersistentVolumeClaims that are not yet bound"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<IoK8sApiCoreV1PersistentVolumeClaimStatusPhase>,
    #[doc = "ResizeStatus stores status of resize operation. ResizeStatus is not set by default but when expansion is complete resizeStatus is set to empty string by resize controller or kubelet. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature."]
    #[serde(
        rename = "resizeStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resize_status: Option<String>,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimStatus>
    for IoK8sApiCoreV1PersistentVolumeClaimStatus
{
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimStatus) -> Self {
        value.clone()
    }
}
#[doc = "Phase represents the current phase of PersistentVolumeClaim.\n\nPossible enum values:\n - `\"Bound\"` used for PersistentVolumeClaims that are bound\n - `\"Lost\"` used for PersistentVolumeClaims that lost their underlying PersistentVolume. The claim was bound to a PersistentVolume and this volume does not exist any longer and all data on it was lost.\n - `\"Pending\"` used for PersistentVolumeClaims that are not yet bound"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1PersistentVolumeClaimStatusPhase {
    Bound,
    Lost,
    Pending,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimStatusPhase>
    for IoK8sApiCoreV1PersistentVolumeClaimStatusPhase
{
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimStatusPhase) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1PersistentVolumeClaimStatusPhase {
    fn to_string(&self) -> String {
        match *self {
            Self::Bound => "Bound".to_string(),
            Self::Lost => "Lost".to_string(),
            Self::Pending => "Pending".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1PersistentVolumeClaimStatusPhase {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Bound" => Ok(Self::Bound),
            "Lost" => Ok(Self::Lost),
            "Pending" => Ok(Self::Pending),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1PersistentVolumeClaimStatusPhase {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1PersistentVolumeClaimStatusPhase {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1PersistentVolumeClaimStatusPhase {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "PersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects as part of an EphemeralVolumeSource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PersistentVolumeClaimTemplate {
    #[doc = "May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IoK8sApimachineryPkgApisMetaV1ObjectMeta>,
    #[doc = "The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here."]
    pub spec: IoK8sApiCoreV1PersistentVolumeClaimSpec,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimTemplate>
    for IoK8sApiCoreV1PersistentVolumeClaimTemplate
{
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimTemplate) -> Self {
        value.clone()
    }
}
#[doc = "PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system)."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PersistentVolumeClaimVolumeSource {
    #[doc = "ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims"]
    #[serde(rename = "claimName")]
    pub claim_name: String,
    #[doc = "Will force the ReadOnly setting in VolumeMounts. Default false."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl From<&IoK8sApiCoreV1PersistentVolumeClaimVolumeSource>
    for IoK8sApiCoreV1PersistentVolumeClaimVolumeSource
{
    fn from(value: &IoK8sApiCoreV1PersistentVolumeClaimVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Photon Controller persistent disk resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PhotonPersistentDiskVolumeSource {
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "ID that identifies Photon Controller persistent disk"]
    #[serde(rename = "pdID")]
    pub pd_id: String,
}
impl From<&IoK8sApiCoreV1PhotonPersistentDiskVolumeSource>
    for IoK8sApiCoreV1PhotonPersistentDiskVolumeSource
{
    fn from(value: &IoK8sApiCoreV1PhotonPersistentDiskVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Pod affinity is a group of inter pod affinity scheduling rules."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PodAffinity {
    #[doc = "The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding \"weight\" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred."]
    #[serde(
        rename = "preferredDuringSchedulingIgnoredDuringExecution",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub preferred_during_scheduling_ignored_during_execution:
        Vec<IoK8sApiCoreV1WeightedPodAffinityTerm>,
    #[doc = "If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied."]
    #[serde(
        rename = "requiredDuringSchedulingIgnoredDuringExecution",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_during_scheduling_ignored_during_execution: Vec<IoK8sApiCoreV1PodAffinityTerm>,
}
impl From<&IoK8sApiCoreV1PodAffinity> for IoK8sApiCoreV1PodAffinity {
    fn from(value: &IoK8sApiCoreV1PodAffinity) -> Self {
        value.clone()
    }
}
#[doc = "Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PodAffinityTerm {
    #[doc = "A label query over a set of resources, in this case pods."]
    #[serde(
        rename = "labelSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_selector: Option<IoK8sApimachineryPkgApisMetaV1LabelSelector>,
    #[doc = "A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means \"this pod's namespace\". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled."]
    #[serde(
        rename = "namespaceSelector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_selector: Option<IoK8sApimachineryPkgApisMetaV1LabelSelector>,
    #[doc = "namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means \"this pod's namespace\""]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    #[doc = "This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed."]
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}
impl From<&IoK8sApiCoreV1PodAffinityTerm> for IoK8sApiCoreV1PodAffinityTerm {
    fn from(value: &IoK8sApiCoreV1PodAffinityTerm) -> Self {
        value.clone()
    }
}
#[doc = "Pod anti affinity is a group of inter pod anti affinity scheduling rules."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PodAntiAffinity {
    #[doc = "The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding \"weight\" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred."]
    #[serde(
        rename = "preferredDuringSchedulingIgnoredDuringExecution",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub preferred_during_scheduling_ignored_during_execution:
        Vec<IoK8sApiCoreV1WeightedPodAffinityTerm>,
    #[doc = "If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied."]
    #[serde(
        rename = "requiredDuringSchedulingIgnoredDuringExecution",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_during_scheduling_ignored_during_execution: Vec<IoK8sApiCoreV1PodAffinityTerm>,
}
impl From<&IoK8sApiCoreV1PodAntiAffinity> for IoK8sApiCoreV1PodAntiAffinity {
    fn from(value: &IoK8sApiCoreV1PodAntiAffinity) -> Self {
        value.clone()
    }
}
#[doc = "PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PodDnsConfig {
    #[doc = "A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nameservers: Vec<String>,
    #[doc = "A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<IoK8sApiCoreV1PodDnsConfigOption>,
    #[doc = "A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<String>,
}
impl From<&IoK8sApiCoreV1PodDnsConfig> for IoK8sApiCoreV1PodDnsConfig {
    fn from(value: &IoK8sApiCoreV1PodDnsConfig) -> Self {
        value.clone()
    }
}
#[doc = "PodDNSConfigOption defines DNS resolver options of a pod."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PodDnsConfigOption {
    #[doc = "Required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoK8sApiCoreV1PodDnsConfigOption> for IoK8sApiCoreV1PodDnsConfigOption {
    fn from(value: &IoK8sApiCoreV1PodDnsConfigOption) -> Self {
        value.clone()
    }
}
#[doc = "PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PodSecurityContext {
    #[doc = "A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:\n\n1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----\n\nIf unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(rename = "fsGroup", default, skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,
    #[doc = "fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are \"OnRootMismatch\" and \"Always\". If not specified, \"Always\" is used. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "fsGroupChangePolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fs_group_change_policy: Option<String>,
    #[doc = "The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "runAsGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_as_group: Option<i64>,
    #[doc = "Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence."]
    #[serde(
        rename = "runAsNonRoot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_as_non_root: Option<bool>,
    #[doc = "The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(rename = "runAsUser", default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[doc = "The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "seLinuxOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub se_linux_options: Option<IoK8sApiCoreV1SeLinuxOptions>,
    #[doc = "The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "seccompProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seccomp_profile: Option<IoK8sApiCoreV1SeccompProfile>,
    #[doc = "A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "supplementalGroups",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supplemental_groups: Vec<i64>,
    #[doc = "Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sysctls: Vec<IoK8sApiCoreV1Sysctl>,
    #[doc = "The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux."]
    #[serde(
        rename = "windowsOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub windows_options: Option<IoK8sApiCoreV1WindowsSecurityContextOptions>,
}
impl From<&IoK8sApiCoreV1PodSecurityContext> for IoK8sApiCoreV1PodSecurityContext {
    fn from(value: &IoK8sApiCoreV1PodSecurityContext) -> Self {
        value.clone()
    }
}
#[doc = "PortworxVolumeSource represents a Portworx volume resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PortworxVolumeSource {
    #[doc = "FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\". Implicitly inferred to be \"ext4\" if unspecified."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "VolumeID uniquely identifies a Portworx volume"]
    #[serde(rename = "volumeID")]
    pub volume_id: String,
}
impl From<&IoK8sApiCoreV1PortworxVolumeSource> for IoK8sApiCoreV1PortworxVolumeSource {
    fn from(value: &IoK8sApiCoreV1PortworxVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op)."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1PreferredSchedulingTerm {
    #[doc = "A node selector term, associated with the corresponding weight."]
    pub preference: IoK8sApiCoreV1NodeSelectorTerm,
    #[doc = "Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100."]
    pub weight: i64,
}
impl From<&IoK8sApiCoreV1PreferredSchedulingTerm> for IoK8sApiCoreV1PreferredSchedulingTerm {
    fn from(value: &IoK8sApiCoreV1PreferredSchedulingTerm) -> Self {
        value.clone()
    }
}
#[doc = "Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Probe {
    #[doc = "Exec specifies the action to take."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<IoK8sApiCoreV1ExecAction>,
    #[doc = "Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1."]
    #[serde(
        rename = "failureThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_threshold: Option<i64>,
    #[doc = "GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<IoK8sApiCoreV1GrpcAction>,
    #[doc = "HTTPGet specifies the http request to perform."]
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<IoK8sApiCoreV1HttpGetAction>,
    #[doc = "Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "initialDelaySeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_delay_seconds: Option<i64>,
    #[doc = "How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1."]
    #[serde(
        rename = "periodSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub period_seconds: Option<i64>,
    #[doc = "Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1."]
    #[serde(
        rename = "successThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub success_threshold: Option<i64>,
    #[doc = "TCPSocket specifies an action involving a TCP port."]
    #[serde(rename = "tcpSocket", default, skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<IoK8sApiCoreV1TcpSocketAction>,
    #[doc = "Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset."]
    #[serde(
        rename = "terminationGracePeriodSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_grace_period_seconds: Option<i64>,
    #[doc = "Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes"]
    #[serde(
        rename = "timeoutSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_seconds: Option<i64>,
}
impl From<&IoK8sApiCoreV1Probe> for IoK8sApiCoreV1Probe {
    fn from(value: &IoK8sApiCoreV1Probe) -> Self {
        value.clone()
    }
}
#[doc = "Represents a projected volume source"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ProjectedVolumeSource {
    #[doc = "Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i64>,
    #[doc = "list of volume projections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<IoK8sApiCoreV1VolumeProjection>,
}
impl From<&IoK8sApiCoreV1ProjectedVolumeSource> for IoK8sApiCoreV1ProjectedVolumeSource {
    fn from(value: &IoK8sApiCoreV1ProjectedVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1QuobyteVolumeSource {
    #[doc = "Group to map volume access to Default is no group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = "ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes"]
    pub registry: String,
    #[doc = "Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[doc = "User to map volume access to Defaults to serivceaccount user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "Volume is a string that references an already created Quobyte volume by name."]
    pub volume: String,
}
impl From<&IoK8sApiCoreV1QuobyteVolumeSource> for IoK8sApiCoreV1QuobyteVolumeSource {
    fn from(value: &IoK8sApiCoreV1QuobyteVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1RbdVolumeSource {
    #[doc = "Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd"]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "The rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    pub image: String,
    #[doc = "Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyring: Option<String>,
    #[doc = "A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    pub monitors: Vec<String>,
    #[doc = "The rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[doc = "ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "The rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl From<&IoK8sApiCoreV1RbdVolumeSource> for IoK8sApiCoreV1RbdVolumeSource {
    fn from(value: &IoK8sApiCoreV1RbdVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "ResourceFieldSelector represents container resources (cpu, memory) and their output format"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ResourceFieldSelector {
    #[doc = "Container name: required for volumes, optional for env vars"]
    #[serde(
        rename = "containerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name: Option<String>,
    #[doc = "Specifies the output format of the exposed resources, defaults to \"1\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IoK8sApimachineryPkgApiResourceQuantity>,
    #[doc = "Required: resource to select"]
    pub resource: String,
}
impl From<&IoK8sApiCoreV1ResourceFieldSelector> for IoK8sApiCoreV1ResourceFieldSelector {
    fn from(value: &IoK8sApiCoreV1ResourceFieldSelector) -> Self {
        value.clone()
    }
}
#[doc = "ResourceRequirements describes the compute resource requirements."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ResourceRequirements {
    #[doc = "Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub limits: std::collections::HashMap<String, IoK8sApimachineryPkgApiResourceQuantity>,
    #[doc = "Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub requests: std::collections::HashMap<String, IoK8sApimachineryPkgApiResourceQuantity>,
}
impl From<&IoK8sApiCoreV1ResourceRequirements> for IoK8sApiCoreV1ResourceRequirements {
    fn from(value: &IoK8sApiCoreV1ResourceRequirements) -> Self {
        value.clone()
    }
}
#[doc = "ScaleIOVolumeSource represents a persistent ScaleIO volume"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ScaleIoVolumeSource {
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Default is \"xfs\"."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "The host address of the ScaleIO API Gateway."]
    pub gateway: String,
    #[doc = "The name of the ScaleIO Protection Domain for the configured storage."]
    #[serde(
        rename = "protectionDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protection_domain: Option<String>,
    #[doc = "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail."]
    #[serde(rename = "secretRef")]
    pub secret_ref: IoK8sApiCoreV1LocalObjectReference,
    #[doc = "Flag to enable/disable SSL communication with Gateway, default false"]
    #[serde(
        rename = "sslEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssl_enabled: Option<bool>,
    #[doc = "Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned."]
    #[serde(
        rename = "storageMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_mode: Option<String>,
    #[doc = "The ScaleIO Storage Pool associated with the protection domain."]
    #[serde(
        rename = "storagePool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_pool: Option<String>,
    #[doc = "The name of the storage system as configured in ScaleIO."]
    pub system: String,
    #[doc = "The name of a volume already created in the ScaleIO system that is associated with this volume source."]
    #[serde(
        rename = "volumeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_name: Option<String>,
}
impl From<&IoK8sApiCoreV1ScaleIoVolumeSource> for IoK8sApiCoreV1ScaleIoVolumeSource {
    fn from(value: &IoK8sApiCoreV1ScaleIoVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "SELinuxOptions are the labels to be applied to the container"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1SeLinuxOptions {
    #[doc = "Level is SELinux level label that applies to the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "Role is a SELinux role label that applies to the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "Type is a SELinux type label that applies to the container."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "User is a SELinux user label that applies to the container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl From<&IoK8sApiCoreV1SeLinuxOptions> for IoK8sApiCoreV1SeLinuxOptions {
    fn from(value: &IoK8sApiCoreV1SeLinuxOptions) -> Self {
        value.clone()
    }
}
#[doc = "SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1SeccompProfile {
    #[doc = "localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is \"Localhost\"."]
    #[serde(
        rename = "localhostProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub localhost_profile: Option<String>,
    #[doc = "type indicates which kind of seccomp profile will be applied. Valid options are:\n\nLocalhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.\n\nPossible enum values:\n - `\"Localhost\"` indicates a profile defined in a file on the node should be used. The file's location relative to <kubelet-root-dir>/seccomp.\n - `\"RuntimeDefault\"` represents the default container runtime seccomp profile.\n - `\"Unconfined\"` indicates no seccomp profile is applied (A.K.A. unconfined)."]
    #[serde(rename = "type")]
    pub type_: IoK8sApiCoreV1SeccompProfileType,
}
impl From<&IoK8sApiCoreV1SeccompProfile> for IoK8sApiCoreV1SeccompProfile {
    fn from(value: &IoK8sApiCoreV1SeccompProfile) -> Self {
        value.clone()
    }
}
#[doc = "type indicates which kind of seccomp profile will be applied. Valid options are:\n\nLocalhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.\n\nPossible enum values:\n - `\"Localhost\"` indicates a profile defined in a file on the node should be used. The file's location relative to <kubelet-root-dir>/seccomp.\n - `\"RuntimeDefault\"` represents the default container runtime seccomp profile.\n - `\"Unconfined\"` indicates no seccomp profile is applied (A.K.A. unconfined)."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1SeccompProfileType {
    Localhost,
    RuntimeDefault,
    Unconfined,
}
impl From<&IoK8sApiCoreV1SeccompProfileType> for IoK8sApiCoreV1SeccompProfileType {
    fn from(value: &IoK8sApiCoreV1SeccompProfileType) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1SeccompProfileType {
    fn to_string(&self) -> String {
        match *self {
            Self::Localhost => "Localhost".to_string(),
            Self::RuntimeDefault => "RuntimeDefault".to_string(),
            Self::Unconfined => "Unconfined".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1SeccompProfileType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Localhost" => Ok(Self::Localhost),
            "RuntimeDefault" => Ok(Self::RuntimeDefault),
            "Unconfined" => Ok(Self::Unconfined),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1SeccompProfileType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1SeccompProfileType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1SeccompProfileType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "SecretEnvSource selects a Secret to populate the environment variables with.\n\nThe contents of the target Secret's Data field will represent the key-value pairs as environment variables."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1SecretEnvSource {
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the Secret must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1SecretEnvSource> for IoK8sApiCoreV1SecretEnvSource {
    fn from(value: &IoK8sApiCoreV1SecretEnvSource) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1SecretKeySelectorBuilder as SecretKeySelectorBuilder;
pub use IoK8sApiCoreV1SecretKeySelectorBuilderError as SecretKeySelectorBuilderError;

#[doc = "SecretKeySelector selects a key of a Secret."]
#[derive(Clone, Debug, Deserialize, Serialize, Builder, Default)]
pub struct IoK8sApiCoreV1SecretKeySelector {
    #[doc = "The key of the secret to select from.  Must be a valid secret key."]
    pub key: String,
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the Secret or its key must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1SecretKeySelector> for IoK8sApiCoreV1SecretKeySelector {
    fn from(value: &IoK8sApiCoreV1SecretKeySelector) -> Self {
        value.clone()
    }
}
#[doc = "Adapts a secret into a projected volume.\n\nThe contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1SecretProjection {
    #[doc = "If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoK8sApiCoreV1KeyToPath>,
    #[doc = "Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specify whether the Secret or its key must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
impl From<&IoK8sApiCoreV1SecretProjection> for IoK8sApiCoreV1SecretProjection {
    fn from(value: &IoK8sApiCoreV1SecretProjection) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1SecretVolumeSourceBuilder as SecretVolumeSourceBuilder;
pub use IoK8sApiCoreV1SecretVolumeSourceBuilderError as SecretVolumeSourceBuilderError;

#[doc = "Adapts a Secret into a volume.\n\nThe contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoK8sApiCoreV1SecretVolumeSource {
    #[doc = "Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set."]
    #[serde(
        rename = "defaultMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_mode: Option<i64>,
    #[doc = "If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IoK8sApiCoreV1KeyToPath>,
    #[doc = "Specify whether the Secret or its keys must be defined"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[doc = "Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret"]
    #[serde(
        rename = "secretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
}
impl From<&IoK8sApiCoreV1SecretVolumeSource> for IoK8sApiCoreV1SecretVolumeSource {
    fn from(value: &IoK8sApiCoreV1SecretVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1SecurityContext {
    #[doc = "AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "allowPrivilegeEscalation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_privilege_escalation: Option<bool>,
    #[doc = "The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<IoK8sApiCoreV1Capabilities>,
    #[doc = "Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[doc = "procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(rename = "procMount", default, skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<String>,
    #[doc = "Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "readOnlyRootFilesystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub read_only_root_filesystem: Option<bool>,
    #[doc = "The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "runAsGroup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_as_group: Option<i64>,
    #[doc = "Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence."]
    #[serde(
        rename = "runAsNonRoot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_as_non_root: Option<bool>,
    #[doc = "The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(rename = "runAsUser", default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[doc = "The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "seLinuxOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub se_linux_options: Option<IoK8sApiCoreV1SeLinuxOptions>,
    #[doc = "The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows."]
    #[serde(
        rename = "seccompProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seccomp_profile: Option<IoK8sApiCoreV1SeccompProfile>,
    #[doc = "The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux."]
    #[serde(
        rename = "windowsOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub windows_options: Option<IoK8sApiCoreV1WindowsSecurityContextOptions>,
}
impl From<&IoK8sApiCoreV1SecurityContext> for IoK8sApiCoreV1SecurityContext {
    fn from(value: &IoK8sApiCoreV1SecurityContext) -> Self {
        value.clone()
    }
}
#[doc = "ServiceAccountTokenProjection represents a projected service account token volume. This projection can be used to insert a service account token into the pods runtime filesystem for use against APIs (Kubernetes API Server or otherwise)."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ServiceAccountTokenProjection {
    #[doc = "Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes."]
    #[serde(
        rename = "expirationSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_seconds: Option<i64>,
    #[doc = "Path is the path relative to the mount point of the file to project the token into."]
    pub path: String,
}
impl From<&IoK8sApiCoreV1ServiceAccountTokenProjection>
    for IoK8sApiCoreV1ServiceAccountTokenProjection
{
    fn from(value: &IoK8sApiCoreV1ServiceAccountTokenProjection) -> Self {
        value.clone()
    }
}
#[doc = "ServicePort contains information on service's port."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1ServicePort {
    #[doc = "The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol."]
    #[serde(
        rename = "appProtocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub app_protocol: Option<String>,
    #[doc = "The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport"]
    #[serde(rename = "nodePort", default, skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i64>,
    #[doc = "The port that will be exposed by this service."]
    pub port: i64,
    #[doc = "The IP protocol for this port. Supports \"TCP\", \"UDP\", and \"SCTP\". Default is TCP.\n\nPossible enum values:\n - `\"SCTP\"` is the SCTP protocol.\n - `\"TCP\"` is the TCP protocol.\n - `\"UDP\"` is the UDP protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<IoK8sApiCoreV1ServicePortProtocol>,
    #[doc = "Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service"]
    #[serde(
        rename = "targetPort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_port: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
}
impl From<&IoK8sApiCoreV1ServicePort> for IoK8sApiCoreV1ServicePort {
    fn from(value: &IoK8sApiCoreV1ServicePort) -> Self {
        value.clone()
    }
}
#[doc = "The IP protocol for this port. Supports \"TCP\", \"UDP\", and \"SCTP\". Default is TCP.\n\nPossible enum values:\n - `\"SCTP\"` is the SCTP protocol.\n - `\"TCP\"` is the TCP protocol.\n - `\"UDP\"` is the UDP protocol."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1ServicePortProtocol {
    #[serde(rename = "SCTP")]
    Sctp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}
impl From<&IoK8sApiCoreV1ServicePortProtocol> for IoK8sApiCoreV1ServicePortProtocol {
    fn from(value: &IoK8sApiCoreV1ServicePortProtocol) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1ServicePortProtocol {
    fn to_string(&self) -> String {
        match *self {
            Self::Sctp => "SCTP".to_string(),
            Self::Tcp => "TCP".to_string(),
            Self::Udp => "UDP".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1ServicePortProtocol {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "SCTP" => Ok(Self::Sctp),
            "TCP" => Ok(Self::Tcp),
            "UDP" => Ok(Self::Udp),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1ServicePortProtocol {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1ServicePortProtocol {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1ServicePortProtocol {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Represents a StorageOS persistent volume resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1StorageOsVolumeSource {
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted."]
    #[serde(rename = "secretRef", default, skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<IoK8sApiCoreV1LocalObjectReference>,
    #[doc = "VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace."]
    #[serde(
        rename = "volumeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_name: Option<String>,
    #[doc = "VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to \"default\" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created."]
    #[serde(
        rename = "volumeNamespace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_namespace: Option<String>,
}
impl From<&IoK8sApiCoreV1StorageOsVolumeSource> for IoK8sApiCoreV1StorageOsVolumeSource {
    fn from(value: &IoK8sApiCoreV1StorageOsVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "Sysctl defines a kernel parameter to be set"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Sysctl {
    #[doc = "Name of a property to set"]
    pub name: String,
    #[doc = "Value of a property to set"]
    pub value: String,
}
impl From<&IoK8sApiCoreV1Sysctl> for IoK8sApiCoreV1Sysctl {
    fn from(value: &IoK8sApiCoreV1Sysctl) -> Self {
        value.clone()
    }
}
#[doc = "TCPSocketAction describes an action based on opening a socket"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1TcpSocketAction {
    #[doc = "Optional: Host name to connect to, defaults to the pod IP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME."]
    pub port: IoK8sApimachineryPkgUtilIntstrIntOrString,
}
impl From<&IoK8sApiCoreV1TcpSocketAction> for IoK8sApiCoreV1TcpSocketAction {
    fn from(value: &IoK8sApiCoreV1TcpSocketAction) -> Self {
        value.clone()
    }
}
#[doc = "The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1Toleration {
    #[doc = "Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.\n\nPossible enum values:\n - `\"NoExecute\"` Evict any already-running pods that do not tolerate the taint. Currently enforced by NodeController.\n - `\"NoSchedule\"` Do not allow new pods to schedule onto the node unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running. Enforced by the scheduler.\n - `\"PreferNoSchedule\"` Like TaintEffectNoSchedule, but the scheduler tries not to schedule new pods onto the node, rather than prohibiting new pods from scheduling onto the node entirely. Enforced by the scheduler."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<IoK8sApiCoreV1TolerationEffect>,
    #[doc = "Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.\n\nPossible enum values:\n - `\"Equal\"`\n - `\"Exists\"`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<IoK8sApiCoreV1TolerationOperator>,
    #[doc = "TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system."]
    #[serde(
        rename = "tolerationSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub toleration_seconds: Option<i64>,
    #[doc = "Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&IoK8sApiCoreV1Toleration> for IoK8sApiCoreV1Toleration {
    fn from(value: &IoK8sApiCoreV1Toleration) -> Self {
        value.clone()
    }
}
#[doc = "Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.\n\nPossible enum values:\n - `\"NoExecute\"` Evict any already-running pods that do not tolerate the taint. Currently enforced by NodeController.\n - `\"NoSchedule\"` Do not allow new pods to schedule onto the node unless they tolerate the taint, but allow all pods submitted to Kubelet without going through the scheduler to start, and allow all already-running pods to continue running. Enforced by the scheduler.\n - `\"PreferNoSchedule\"` Like TaintEffectNoSchedule, but the scheduler tries not to schedule new pods onto the node, rather than prohibiting new pods from scheduling onto the node entirely. Enforced by the scheduler."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1TolerationEffect {
    NoExecute,
    NoSchedule,
    PreferNoSchedule,
}
impl From<&IoK8sApiCoreV1TolerationEffect> for IoK8sApiCoreV1TolerationEffect {
    fn from(value: &IoK8sApiCoreV1TolerationEffect) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1TolerationEffect {
    fn to_string(&self) -> String {
        match *self {
            Self::NoExecute => "NoExecute".to_string(),
            Self::NoSchedule => "NoSchedule".to_string(),
            Self::PreferNoSchedule => "PreferNoSchedule".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1TolerationEffect {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "NoExecute" => Ok(Self::NoExecute),
            "NoSchedule" => Ok(Self::NoSchedule),
            "PreferNoSchedule" => Ok(Self::PreferNoSchedule),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1TolerationEffect {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1TolerationEffect {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1TolerationEffect {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.\n\nPossible enum values:\n - `\"Equal\"`\n - `\"Exists\"`"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IoK8sApiCoreV1TolerationOperator {
    Equal,
    Exists,
}
impl From<&IoK8sApiCoreV1TolerationOperator> for IoK8sApiCoreV1TolerationOperator {
    fn from(value: &IoK8sApiCoreV1TolerationOperator) -> Self {
        value.clone()
    }
}
impl ToString for IoK8sApiCoreV1TolerationOperator {
    fn to_string(&self) -> String {
        match *self {
            Self::Equal => "Equal".to_string(),
            Self::Exists => "Exists".to_string(),
        }
    }
}
impl std::str::FromStr for IoK8sApiCoreV1TolerationOperator {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Equal" => Ok(Self::Equal),
            "Exists" => Ok(Self::Exists),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IoK8sApiCoreV1TolerationOperator {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApiCoreV1TolerationOperator {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApiCoreV1TolerationOperator {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1TypedLocalObjectReference {
    #[doc = "APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required."]
    #[serde(rename = "apiGroup", default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    #[doc = "Kind is the type of resource being referenced"]
    pub kind: String,
    #[doc = "Name is the name of resource being referenced"]
    pub name: String,
}
impl From<&IoK8sApiCoreV1TypedLocalObjectReference> for IoK8sApiCoreV1TypedLocalObjectReference {
    fn from(value: &IoK8sApiCoreV1TypedLocalObjectReference) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1VolumeBuilder as VolumeBuilder;
pub use IoK8sApiCoreV1VolumeBuilderError as VolumeBuilderError;

#[doc = "Volume represents a named volume in a pod that may be accessed by any container in the pod."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoK8sApiCoreV1Volume {
    #[doc = "AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore"]
    #[serde(
        rename = "awsElasticBlockStore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_elastic_block_store: Option<IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource>,
    #[doc = "AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod."]
    #[serde(rename = "azureDisk", default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<IoK8sApiCoreV1AzureDiskVolumeSource>,
    #[doc = "AzureFile represents an Azure File Service mount on the host and bind mount to the pod."]
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<IoK8sApiCoreV1AzureFileVolumeSource>,
    #[doc = "CephFS represents a Ceph FS mount on the host that shares a pod's lifetime"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<IoK8sApiCoreV1CephFsVolumeSource>,
    #[doc = "Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<IoK8sApiCoreV1CinderVolumeSource>,
    #[doc = "ConfigMap represents a configMap that should populate this volume"]
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<IoK8sApiCoreV1ConfigMapVolumeSource>,
    #[doc = "CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<IoK8sApiCoreV1CsiVolumeSource>,
    #[doc = "DownwardAPI represents downward API about the pod that should populate this volume"]
    #[serde(
        rename = "downwardAPI",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub downward_api: Option<IoK8sApiCoreV1DownwardApiVolumeSource>,
    #[doc = "EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir"]
    #[serde(rename = "emptyDir", default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<IoK8sApiCoreV1EmptyDirVolumeSource>,
    #[doc = "Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.\n\nUse this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity\n   tracking are needed,\nc) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through\n   a PersistentVolumeClaim (see EphemeralVolumeSource for more\n   information on the connection between this volume type\n   and PersistentVolumeClaim).\n\nUse PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.\n\nUse CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.\n\nA pod can use both types of ephemeral volumes and persistent volumes at the same time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<IoK8sApiCoreV1EphemeralVolumeSource>,
    #[doc = "FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<IoK8sApiCoreV1FcVolumeSource>,
    #[doc = "FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin."]
    #[serde(
        rename = "flexVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flex_volume: Option<IoK8sApiCoreV1FlexVolumeSource>,
    #[doc = "Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<IoK8sApiCoreV1FlockerVolumeSource>,
    #[doc = "GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk"]
    #[serde(
        rename = "gcePersistentDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gce_persistent_disk: Option<IoK8sApiCoreV1GcePersistentDiskVolumeSource>,
    #[doc = "GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container."]
    #[serde(rename = "gitRepo", default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<IoK8sApiCoreV1GitRepoVolumeSource>,
    #[doc = "Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<IoK8sApiCoreV1GlusterfsVolumeSource>,
    #[doc = "HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath"]
    #[serde(rename = "hostPath", default, skip_serializing_if = "Option::is_none")]
    pub host_path: Option<IoK8sApiCoreV1HostPathVolumeSource>,
    #[doc = "ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<IoK8sApiCoreV1IscsiVolumeSource>,
    #[doc = "Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    pub name: String,
    #[doc = "NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<IoK8sApiCoreV1NfsVolumeSource>,
    #[doc = "PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims"]
    #[serde(
        rename = "persistentVolumeClaim",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_volume_claim: Option<IoK8sApiCoreV1PersistentVolumeClaimVolumeSource>,
    #[doc = "PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine"]
    #[serde(
        rename = "photonPersistentDisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub photon_persistent_disk: Option<IoK8sApiCoreV1PhotonPersistentDiskVolumeSource>,
    #[doc = "PortworxVolume represents a portworx volume attached and mounted on kubelets host machine"]
    #[serde(
        rename = "portworxVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub portworx_volume: Option<IoK8sApiCoreV1PortworxVolumeSource>,
    #[doc = "Items for all in one resources secrets, configmaps, and downward API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<IoK8sApiCoreV1ProjectedVolumeSource>,
    #[doc = "Quobyte represents a Quobyte mount on the host that shares a pod's lifetime"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<IoK8sApiCoreV1QuobyteVolumeSource>,
    #[doc = "RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<IoK8sApiCoreV1RbdVolumeSource>,
    #[doc = "ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes."]
    #[serde(rename = "scaleIO", default, skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<IoK8sApiCoreV1ScaleIoVolumeSource>,
    #[doc = "Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<IoK8sApiCoreV1SecretVolumeSource>,
    #[doc = "StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storageos: Option<IoK8sApiCoreV1StorageOsVolumeSource>,
    #[doc = "VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine"]
    #[serde(
        rename = "vsphereVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vsphere_volume: Option<IoK8sApiCoreV1VsphereVirtualDiskVolumeSource>,
}
impl From<&IoK8sApiCoreV1Volume> for IoK8sApiCoreV1Volume {
    fn from(value: &IoK8sApiCoreV1Volume) -> Self {
        value.clone()
    }
}
#[doc = "volumeDevice describes a mapping of a raw block device within a container."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1VolumeDevice {
    #[doc = "devicePath is the path inside of the container that the device will be mapped to."]
    #[serde(rename = "devicePath")]
    pub device_path: String,
    #[doc = "name must match the name of a persistentVolumeClaim in the pod"]
    pub name: String,
}
impl From<&IoK8sApiCoreV1VolumeDevice> for IoK8sApiCoreV1VolumeDevice {
    fn from(value: &IoK8sApiCoreV1VolumeDevice) -> Self {
        value.clone()
    }
}

pub use IoK8sApiCoreV1VolumeMountBuilder as VolumeMountBuilder;
pub use IoK8sApiCoreV1VolumeMountBuilderError as VolumeMountBuilderError;

#[doc = "VolumeMount describes a mounting of a Volume within a container."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
#[builder(default)]
pub struct IoK8sApiCoreV1VolumeMount {
    #[doc = "Path within the container at which the volume should be mounted.  Must not contain ':'."]
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[doc = "mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10."]
    #[serde(
        rename = "mountPropagation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_propagation: Option<String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: String,
    #[doc = "Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root)."]
    #[serde(rename = "subPath", default, skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    #[doc = "Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to \"\" (volume's root). SubPathExpr and SubPath are mutually exclusive."]
    #[serde(
        rename = "subPathExpr",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_path_expr: Option<String>,
}
impl From<&IoK8sApiCoreV1VolumeMount> for IoK8sApiCoreV1VolumeMount {
    fn from(value: &IoK8sApiCoreV1VolumeMount) -> Self {
        value.clone()
    }
}
#[doc = "Projection that may be projected along with other supported volume types"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1VolumeProjection {
    #[doc = "information about the configMap data to project"]
    #[serde(rename = "configMap", default, skip_serializing_if = "Option::is_none")]
    pub config_map: Option<IoK8sApiCoreV1ConfigMapProjection>,
    #[doc = "information about the downwardAPI data to project"]
    #[serde(
        rename = "downwardAPI",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub downward_api: Option<IoK8sApiCoreV1DownwardApiProjection>,
    #[doc = "information about the secret data to project"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<IoK8sApiCoreV1SecretProjection>,
    #[doc = "information about the serviceAccountToken data to project"]
    #[serde(
        rename = "serviceAccountToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_token: Option<IoK8sApiCoreV1ServiceAccountTokenProjection>,
}
impl From<&IoK8sApiCoreV1VolumeProjection> for IoK8sApiCoreV1VolumeProjection {
    fn from(value: &IoK8sApiCoreV1VolumeProjection) -> Self {
        value.clone()
    }
}
#[doc = "Represents a vSphere volume resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1VsphereVirtualDiskVolumeSource {
    #[doc = "Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified."]
    #[serde(rename = "fsType", default, skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[doc = "Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName."]
    #[serde(
        rename = "storagePolicyID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_policy_id: Option<String>,
    #[doc = "Storage Policy Based Management (SPBM) profile name."]
    #[serde(
        rename = "storagePolicyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_policy_name: Option<String>,
    #[doc = "Path that identifies vSphere volume vmdk"]
    #[serde(rename = "volumePath")]
    pub volume_path: String,
}
impl From<&IoK8sApiCoreV1VsphereVirtualDiskVolumeSource>
    for IoK8sApiCoreV1VsphereVirtualDiskVolumeSource
{
    fn from(value: &IoK8sApiCoreV1VsphereVirtualDiskVolumeSource) -> Self {
        value.clone()
    }
}
#[doc = "The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1WeightedPodAffinityTerm {
    #[doc = "Required. A pod affinity term, associated with the corresponding weight."]
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: IoK8sApiCoreV1PodAffinityTerm,
    #[doc = "weight associated with matching the corresponding podAffinityTerm, in the range 1-100."]
    pub weight: i64,
}
impl From<&IoK8sApiCoreV1WeightedPodAffinityTerm> for IoK8sApiCoreV1WeightedPodAffinityTerm {
    fn from(value: &IoK8sApiCoreV1WeightedPodAffinityTerm) -> Self {
        value.clone()
    }
}
#[doc = "WindowsSecurityContextOptions contain Windows-specific options and credentials."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiCoreV1WindowsSecurityContextOptions {
    #[doc = "GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field."]
    #[serde(
        rename = "gmsaCredentialSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gmsa_credential_spec: Option<String>,
    #[doc = "GMSACredentialSpecName is the name of the GMSA credential spec to use."]
    #[serde(
        rename = "gmsaCredentialSpecName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gmsa_credential_spec_name: Option<String>,
    #[doc = "HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true."]
    #[serde(
        rename = "hostProcess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub host_process: Option<bool>,
    #[doc = "The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence."]
    #[serde(
        rename = "runAsUserName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_as_user_name: Option<String>,
}
impl From<&IoK8sApiCoreV1WindowsSecurityContextOptions>
    for IoK8sApiCoreV1WindowsSecurityContextOptions
{
    fn from(value: &IoK8sApiCoreV1WindowsSecurityContextOptions) -> Self {
        value.clone()
    }
}
#[doc = "PodDisruptionBudgetSpec is a description of a PodDisruptionBudget."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApiPolicyV1PodDisruptionBudgetSpec {
    #[doc = "An eviction is allowed if at most \"maxUnavailable\" pods selected by \"selector\" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with \"minAvailable\"."]
    #[serde(
        rename = "maxUnavailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_unavailable: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "An eviction is allowed if at least \"minAvailable\" pods selected by \"selector\" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying \"100%\"."]
    #[serde(
        rename = "minAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_available: Option<IoK8sApimachineryPkgUtilIntstrIntOrString>,
    #[doc = "Label query over pods whose evictions are managed by the disruption budget. A null selector will match no pods, while an empty ({}) selector will select all pods within the namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<IoK8sApimachineryPkgApisMetaV1LabelSelector>,
}
impl From<&IoK8sApiPolicyV1PodDisruptionBudgetSpec> for IoK8sApiPolicyV1PodDisruptionBudgetSpec {
    fn from(value: &IoK8sApiPolicyV1PodDisruptionBudgetSpec) -> Self {
        value.clone()
    }
}
#[doc = "Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.\n\nThe serialization format is:\n\n<quantity>        ::= <signedNumber><suffix>\n  (Note that <suffix> may be empty, from the \"\" case in <decimalSI>.)\n<digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= \"+\" | \"-\" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei\n  (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)\n<decimalSI>       ::= m | \"\" | k | M | G | T | P | E\n  (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)\n<decimalExponent> ::= \"e\" <signedNumber> | \"E\" <signedNumber>\n\nNo matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.\n\nWhen a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.\n\nBefore serializing, Quantity will be put in \"canonical form\". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:\n  a. No precision is lost\n  b. No fractional digits will be emitted\n  c. The exponent (or suffix) is as large as possible.\nThe sign will be omitted unless the number is negative.\n\nExamples:\n  1.5 will be serialized as \"1500m\"\n  1.5Gi will be serialized as \"1536Mi\"\n\nNote that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.\n\nNon-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)\n\nThis format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IoK8sApimachineryPkgApiResourceQuantity(pub String);
impl std::ops::Deref for IoK8sApimachineryPkgApiResourceQuantity {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IoK8sApimachineryPkgApiResourceQuantity> for String {
    fn from(value: IoK8sApimachineryPkgApiResourceQuantity) -> Self {
        value.0
    }
}
impl From<&IoK8sApimachineryPkgApiResourceQuantity> for IoK8sApimachineryPkgApiResourceQuantity {
    fn from(value: &IoK8sApimachineryPkgApiResourceQuantity) -> Self {
        value.clone()
    }
}
impl From<String> for IoK8sApimachineryPkgApiResourceQuantity {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IoK8sApimachineryPkgApiResourceQuantity {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for IoK8sApimachineryPkgApiResourceQuantity {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "CreateOptions may be provided when creating an API object."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1CreateOptions {
    #[serde(rename = "dryRun", default, skip_serializing_if = "Vec::is_empty")]
    pub dry_run: Vec<String>,
    #[serde(
        rename = "fieldManager",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_manager: Option<String>,
    #[serde(
        rename = "fieldValidation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_validation: Option<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1CreateOptions>
    for IoK8sApimachineryPkgApisMetaV1CreateOptions
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1CreateOptions) -> Self {
        value.clone()
    }
}
#[doc = "Duration is a wrapper around time.Duration which supports correct\nmarshaling to YAML and JSON. In particular, it marshals into strings, which\ncan be used as map keys in json."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1Duration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1Duration> for IoK8sApimachineryPkgApisMetaV1Duration {
    fn from(value: &IoK8sApimachineryPkgApisMetaV1Duration) -> Self {
        value.clone()
    }
}
#[doc = "FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.\n\nEach key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:<name>', where <name> is the name of a field in a struct, or key in a map 'v:<value>', where <value> is the exact json formatted value of a list item 'i:<index>', where <index> is position of a item in a list 'k:<keys>', where <keys> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.\n\nThe exact format is defined in sigs.k8s.io/structured-merge-diff"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1FieldsV1(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for IoK8sApimachineryPkgApisMetaV1FieldsV1 {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<IoK8sApimachineryPkgApisMetaV1FieldsV1> for serde_json::Map<String, serde_json::Value> {
    fn from(value: IoK8sApimachineryPkgApisMetaV1FieldsV1) -> Self {
        value.0
    }
}
impl From<&IoK8sApimachineryPkgApisMetaV1FieldsV1> for IoK8sApimachineryPkgApisMetaV1FieldsV1 {
    fn from(value: &IoK8sApimachineryPkgApisMetaV1FieldsV1) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for IoK8sApimachineryPkgApisMetaV1FieldsV1 {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "+protobuf.options.(gogoproto.goproto_stringer)=false"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1GroupVersionResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1GroupVersionResource>
    for IoK8sApimachineryPkgApisMetaV1GroupVersionResource
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1GroupVersionResource) -> Self {
        value.clone()
    }
}
#[doc = "A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1LabelSelector {
    #[doc = "matchExpressions is a list of label selector requirements. The requirements are ANDed."]
    #[serde(
        rename = "matchExpressions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub match_expressions: Vec<IoK8sApimachineryPkgApisMetaV1LabelSelectorRequirement>,
    #[doc = "matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is \"key\", the operator is \"In\", and the values array contains only \"value\". The requirements are ANDed."]
    #[serde(
        rename = "matchLabels",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub match_labels: std::collections::HashMap<String, String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1LabelSelector>
    for IoK8sApimachineryPkgApisMetaV1LabelSelector
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1LabelSelector) -> Self {
        value.clone()
    }
}
#[doc = "A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1LabelSelectorRequirement {
    #[doc = "key is the label key that the selector applies to."]
    pub key: String,
    #[doc = "operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist."]
    pub operator: String,
    #[doc = "values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1LabelSelectorRequirement>
    for IoK8sApimachineryPkgApisMetaV1LabelSelectorRequirement
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1LabelSelectorRequirement) -> Self {
        value.clone()
    }
}
#[doc = "ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1ListMeta {
    #[doc = "continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a consistent list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response, unless you have received this token from an error message."]
    #[serde(rename = "continue", default, skip_serializing_if = "Option::is_none")]
    pub continue_: Option<String>,
    #[doc = "remainingItemCount is the number of subsequent items in the list which are not included in this list response. If the list request contained label or field selectors, then the number of remaining items is unknown and the field will be left unset and omitted during serialization. If the list is complete (either because it is not chunking or because this is the last chunk), then there are no more remaining items and this field will be left unset and omitted during serialization. Servers older than v1.15 do not set this field. The intended use of the remainingItemCount is *estimating* the size of a collection. Clients should not rely on the remainingItemCount to be set or to be exact."]
    #[serde(
        rename = "remainingItemCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_item_count: Option<i64>,
    #[doc = "String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency"]
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<String>,
    #[doc = "selfLink is a URL representing this object. Populated by the system. Read-only.\n\nDEPRECATED Kubernetes will stop propagating this field in 1.20 release and the field is planned to be removed in 1.21 release."]
    #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1ListMeta> for IoK8sApimachineryPkgApisMetaV1ListMeta {
    fn from(value: &IoK8sApimachineryPkgApisMetaV1ListMeta) -> Self {
        value.clone()
    }
}
#[doc = "ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1ManagedFieldsEntry {
    #[doc = "APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted."]
    #[serde(
        rename = "apiVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_version: Option<String>,
    #[doc = "FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\""]
    #[serde(
        rename = "fieldsType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fields_type: Option<String>,
    #[doc = "FieldsV1 holds the first JSON version format as described in the \"FieldsV1\" type."]
    #[serde(rename = "fieldsV1", default, skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<IoK8sApimachineryPkgApisMetaV1FieldsV1>,
    #[doc = "Manager is an identifier of the workflow managing these fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[doc = "Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    #[doc = "Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1ManagedFieldsEntry>
    for IoK8sApimachineryPkgApisMetaV1ManagedFieldsEntry
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1ManagedFieldsEntry) -> Self {
        value.clone()
    }
}
#[doc = "MicroTime is version of Time with microsecond level precision."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1MicroTime(pub chrono::DateTime<chrono::offset::Utc>);
impl std::ops::Deref for IoK8sApimachineryPkgApisMetaV1MicroTime {
    type Target = chrono::DateTime<chrono::offset::Utc>;
    fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
        &self.0
    }
}
impl From<IoK8sApimachineryPkgApisMetaV1MicroTime> for chrono::DateTime<chrono::offset::Utc> {
    fn from(value: IoK8sApimachineryPkgApisMetaV1MicroTime) -> Self {
        value.0
    }
}
impl From<&IoK8sApimachineryPkgApisMetaV1MicroTime> for IoK8sApimachineryPkgApisMetaV1MicroTime {
    fn from(value: &IoK8sApimachineryPkgApisMetaV1MicroTime) -> Self {
        value.clone()
    }
}
impl From<chrono::DateTime<chrono::offset::Utc>> for IoK8sApimachineryPkgApisMetaV1MicroTime {
    fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IoK8sApimachineryPkgApisMetaV1MicroTime {
    type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for IoK8sApimachineryPkgApisMetaV1MicroTime {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApimachineryPkgApisMetaV1MicroTime {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApimachineryPkgApisMetaV1MicroTime {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for IoK8sApimachineryPkgApisMetaV1MicroTime {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub use IoK8sApimachineryPkgApisMetaV1ObjectMetaBuilder as ObjectMetaBuilder;
pub use IoK8sApimachineryPkgApisMetaV1ObjectMetaBuilderError as ObjectMetaBuilderError;
#[doc = "ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create."]
#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
pub struct IoK8sApimachineryPkgApisMetaV1ObjectMeta {
    #[builder(default)]
    #[doc = "Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub annotations: std::collections::HashMap<String, String>,
    #[builder(default)]
    #[doc = "The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request."]
    #[serde(
        rename = "clusterName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_name: Option<String>,
    #[builder(default)]
    #[doc = "CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.\n\nPopulated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata"]
    #[serde(
        rename = "creationTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_timestamp: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[builder(default)]
    #[doc = "Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only."]
    #[serde(
        rename = "deletionGracePeriodSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_grace_period_seconds: Option<i64>,
    #[builder(default)]
    #[doc = "DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.\n\nPopulated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata"]
    #[serde(
        rename = "deletionTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_timestamp: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[builder(default)]
    #[doc = "Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,
    #[builder(default)]
    #[doc = "GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.\n\nIf this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).\n\nApplied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency"]
    #[serde(
        rename = "generateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_name: Option<String>,
    #[builder(default)]
    #[doc = "A sequence number representing a specific generation of the desired state. Populated by the system. Read-only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    #[builder(default)]
    #[doc = "Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,
    #[builder(default)]
    #[doc = "ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like \"ci-cd\". The set of fields is always in the version that the workflow used when modifying the object."]
    #[serde(
        rename = "managedFields",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub managed_fields: Vec<IoK8sApimachineryPkgApisMetaV1ManagedFieldsEntry>,
    #[doc = "Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(default)]
    #[doc = "Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the \"default\" namespace, but \"default\" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.\n\nMust be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[builder(default)]
    #[doc = "List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller."]
    #[serde(
        rename = "ownerReferences",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub owner_references: Vec<IoK8sApimachineryPkgApisMetaV1OwnerReference>,
    #[builder(default)]
    #[doc = "An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.\n\nPopulated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency"]
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<String>,
    #[builder(default)]
    #[doc = "SelfLink is a URL representing this object. Populated by the system. Read-only.\n\nDEPRECATED Kubernetes will stop propagating this field in 1.20 release and the field is planned to be removed in 1.21 release."]
    #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[builder(default)]
    #[doc = "UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.\n\nPopulated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1ObjectMeta> for IoK8sApimachineryPkgApisMetaV1ObjectMeta {
    fn from(value: &IoK8sApimachineryPkgApisMetaV1ObjectMeta) -> Self {
        value.clone()
    }
}
#[doc = "OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1OwnerReference {
    #[doc = "API version of the referent."]
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[doc = "If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned."]
    #[serde(
        rename = "blockOwnerDeletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_owner_deletion: Option<bool>,
    #[doc = "If true, this reference points to the managing controller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    #[doc = "Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"]
    pub kind: String,
    #[doc = "Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names"]
    pub name: String,
    #[doc = "UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids"]
    pub uid: String,
}
impl From<&IoK8sApimachineryPkgApisMetaV1OwnerReference>
    for IoK8sApimachineryPkgApisMetaV1OwnerReference
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1OwnerReference) -> Self {
        value.clone()
    }
}
#[doc = "StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1StatusCause {
    #[doc = "The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.\n\nExamples:\n  \"name\" - the field \"name\" on the current resource\n  \"items[0].name\" - the field \"name\" on the first array entry in \"items\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[doc = "A human-readable description of the cause of the error.  This field may be presented as-is to a reader."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "A machine-readable description of the cause of the error. If this value is empty there is no information available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl From<&IoK8sApimachineryPkgApisMetaV1StatusCause>
    for IoK8sApimachineryPkgApisMetaV1StatusCause
{
    fn from(value: &IoK8sApimachineryPkgApisMetaV1StatusCause) -> Self {
        value.clone()
    }
}
#[doc = "Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IoK8sApimachineryPkgApisMetaV1Time(pub chrono::DateTime<chrono::offset::Utc>);
impl std::ops::Deref for IoK8sApimachineryPkgApisMetaV1Time {
    type Target = chrono::DateTime<chrono::offset::Utc>;
    fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
        &self.0
    }
}
impl From<IoK8sApimachineryPkgApisMetaV1Time> for chrono::DateTime<chrono::offset::Utc> {
    fn from(value: IoK8sApimachineryPkgApisMetaV1Time) -> Self {
        value.0
    }
}
impl From<&IoK8sApimachineryPkgApisMetaV1Time> for IoK8sApimachineryPkgApisMetaV1Time {
    fn from(value: &IoK8sApimachineryPkgApisMetaV1Time) -> Self {
        value.clone()
    }
}
impl From<chrono::DateTime<chrono::offset::Utc>> for IoK8sApimachineryPkgApisMetaV1Time {
    fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IoK8sApimachineryPkgApisMetaV1Time {
    type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for IoK8sApimachineryPkgApisMetaV1Time {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IoK8sApimachineryPkgApisMetaV1Time {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IoK8sApimachineryPkgApisMetaV1Time {
    type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for IoK8sApimachineryPkgApisMetaV1Time {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IoK8sApimachineryPkgUtilIntstrIntOrString(pub String);
impl std::ops::Deref for IoK8sApimachineryPkgUtilIntstrIntOrString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IoK8sApimachineryPkgUtilIntstrIntOrString> for String {
    fn from(value: IoK8sApimachineryPkgUtilIntstrIntOrString) -> Self {
        value.0
    }
}
impl From<&IoK8sApimachineryPkgUtilIntstrIntOrString>
    for IoK8sApimachineryPkgUtilIntstrIntOrString
{
    fn from(value: &IoK8sApimachineryPkgUtilIntstrIntOrString) -> Self {
        value.clone()
    }
}
impl From<String> for IoK8sApimachineryPkgUtilIntstrIntOrString {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IoK8sApimachineryPkgUtilIntstrIntOrString {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for IoK8sApimachineryPkgUtilIntstrIntOrString {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorCreateSensorRequest {
    #[serde(
        rename = "createOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_options: Option<IoK8sApimachineryPkgApisMetaV1CreateOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor: Option<IoArgoprojEventsV1alpha1Sensor>,
}
impl From<&SensorCreateSensorRequest> for SensorCreateSensorRequest {
    fn from(value: &SensorCreateSensorRequest) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorDeleteSensorResponse(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for SensorDeleteSensorResponse {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<SensorDeleteSensorResponse> for serde_json::Map<String, serde_json::Value> {
    fn from(value: SensorDeleteSensorResponse) -> Self {
        value.0
    }
}
impl From<&SensorDeleteSensorResponse> for SensorDeleteSensorResponse {
    fn from(value: &SensorDeleteSensorResponse) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for SensorDeleteSensorResponse {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorLogEntry {
    #[serde(
        rename = "dependencyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dependency_name: Option<String>,
    #[serde(
        rename = "eventContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(
        rename = "sensorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<IoK8sApimachineryPkgApisMetaV1Time>,
    #[serde(
        rename = "triggerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_name: Option<String>,
}
impl From<&SensorLogEntry> for SensorLogEntry {
    fn from(value: &SensorLogEntry) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorSensorWatchEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<IoArgoprojEventsV1alpha1Sensor>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&SensorSensorWatchEvent> for SensorSensorWatchEvent {
    fn from(value: &SensorSensorWatchEvent) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorUpdateSensorRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensor: Option<IoArgoprojEventsV1alpha1Sensor>,
}
impl From<&SensorUpdateSensorRequest> for SensorUpdateSensorRequest {
    fn from(value: &SensorUpdateSensorRequest) -> Self {
        value.clone()
    }
}
