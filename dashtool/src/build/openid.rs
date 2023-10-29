use std::{fs, str::FromStr};

use anyhow::anyhow;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, DeviceAuthorizationResponse, DeviceAuthorizationUrl,
    EmptyExtraDeviceAuthorizationFields, IssuerUrl, OAuth2TokenResponse, RefreshToken,
    TokenResponse,
};

use crate::{config::Config, error::Error};

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
    Ok(refresh_token.secret().to_string())
}

pub async fn authorization(
    issuer_url: &str,
    client_id: &str,
    refresh_token: &str,
) -> Result<(String, String), Error> {
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
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_owned()))
        .request_async(async_http_client)
        .await?;

    let access_token = response.access_token().secret().to_string();
    let id_token = response
        .id_token()
        .ok_or(Error::NoToken("id".to_string()))?;
    Ok((access_token, id_token.to_string()))
}

pub(crate) async fn get_refresh_token(config: &Config) -> Result<String, Error> {
    #[cfg(not(target_arch = "wasm32"))]
    let refresh_token = match fs::read_to_string(
        dirs::config_local_dir()
            .and_then(|x| Some(String::from_str(x.to_str()?).ok()?))
            .ok_or(Error::Anyhow(anyhow!("Failed to get config directory.")))?
            + "/dashtool/refresh.jwt",
    ) {
        Ok(token) => token,
        Err(_) => {
            let refresh_token = fetch_refresh_token(&config).await?;
            refresh_token
        }
    };
    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    let refresh_token = std::env::var("REFRESH_TOKEN")?;
    Ok(refresh_token)
}

pub(crate) async fn fetch_refresh_token(config: &Config) -> Result<String, Error> {
    let refresh_token = authentication(&config.issuer, &config.client_id).await?;
    fs::write(
        dirs::config_local_dir()
            .and_then(|x| Some(String::from_str(x.to_str()?).ok()?))
            .ok_or(Error::Anyhow(anyhow!("Failed to get config directory.")))?
            + "/dashtool/refresh.jwt",
        &refresh_token,
    )?;
    Ok(refresh_token)
}
