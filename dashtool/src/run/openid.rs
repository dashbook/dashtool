use std::fs;

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, DeviceAuthorizationResponse, DeviceAuthorizationUrl,
    EmptyExtraDeviceAuthorizationFields, IssuerUrl, OAuth2TokenResponse, RefreshToken,
    TokenResponse,
};

use crate::error::Error;

pub async fn authentication(issuer_url: &str, client_id: &str) -> Result<String, Error> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(issuer_url.to_string())?,
        async_http_client,
    )
    .await?;
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id.to_string()),
        None,
    )
    .enable_openid_scope()
    .set_device_authorization_uri(DeviceAuthorizationUrl::new(
        issuer_url.to_string() + "/protocol/openid-connect/auth/device",
    )?);
    let details: DeviceAuthorizationResponse<EmptyExtraDeviceAuthorizationFields> = client
        .exchange_device_code()?
        .request_async(async_http_client)
        .await?;

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri().to_string(),
        details.user_code().secret().to_string()
    );
    let tokens = client
        .exchange_device_access_token(&details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await?;

    let refresh_token = tokens
        .refresh_token()
        .ok_or(Error::NoToken("device".to_string()))?;

    fs::write(".dashtool/refresh.jwt", refresh_token.secret())?;
    Ok(refresh_token.secret().to_string())
}

pub async fn authorization(issuer_url: &str, client_id: &str) -> Result<(String, String), Error> {
    let refresh_token = if fs::metadata(".dashtool/refresh.jwt").is_ok() {
        fs::read_to_string(".dashtool/refresh.jwt").map_err(|err| Error::from(err))
    } else {
        authentication(issuer_url, client_id).await
    }?;

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(issuer_url.to_string())?,
        async_http_client,
    )
    .await?;
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id.to_string()),
        None,
    )
    .enable_openid_scope();

    let response = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await?;

    let access_token = response.access_token().secret().to_string();
    let id_token = response
        .id_token()
        .ok_or(Error::NoToken("id".to_string()))?;
    Ok((access_token, id_token.to_string()))
}
