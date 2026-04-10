use anyhow::{Context, Result};
use reqwest::Client;
use crate::config::AppConfig;
use crate::models::auth::TokenResponse;
use crate::t;

/// Built-in client credentials for password grant mode.
/// Injected at compile time via environment variables:
///   FBOX_BUILTIN_CLIENT_ID / FBOX_BUILTIN_CLIENT_SECRET
/// Falls back to "builtin" if not set.
const BUILTIN_CLIENT_ID: &str = match option_env!("FBOX_BUILTIN_CLIENT_ID") {
    Some(v) => v,
    None => "builtin",
};
const BUILTIN_CLIENT_SECRET: &str = match option_env!("FBOX_BUILTIN_CLIENT_SECRET") {
    Some(v) => v,
    None => "builtin",
};

/// Returns (client_id, client_secret) based on grant type.
/// - password grant: use built-in static credentials
/// - client_credentials: use user-configured credentials
fn resolve_client_credentials(config: &AppConfig) -> (&str, &str) {
    if config.grant_type == "password" {
        (BUILTIN_CLIENT_ID, BUILTIN_CLIENT_SECRET)
    } else {
        (config.client_id.as_str(), config.client_secret.as_str())
    }
}

pub async fn login(http: &Client, config: &AppConfig) -> Result<TokenResponse> {
    let url = format!("{}/idserver/core/connect/token", config.server);
    let (cid, csecret) = resolve_client_credentials(config);
    let mut params = vec![
        ("client_id", cid),
        ("client_secret", csecret),
        ("grant_type", config.grant_type.as_str()),
    ];
    if config.grant_type == "password" {
        params.push(("username", config.username.as_deref().unwrap_or("")));
        params.push(("password", config.password.as_deref().unwrap_or("")));
        params.push(("scope", "openid offline_access fbox email profile"));
    } else {
        params.push(("scope", "fbox"));
    }
    let resp = http.post(&url).form(&params).send().await
        .context(t!("Failed to send login request", "登录请求发送失败"))?;
    let token: TokenResponse = resp.json().await
        .context(t!("Failed to parse login response, please check server URL", "登录响应解析失败，请检查服务器地址是否正确"))?;
    Ok(token)
}

pub async fn refresh(http: &Client, config: &AppConfig, refresh_token: &str) -> Result<TokenResponse> {
    let url = format!("{}/idserver/core/connect/token", config.server);
    let (cid, csecret) = resolve_client_credentials(config);
    let params = vec![
        ("client_id", cid),
        ("client_secret", csecret),
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("scope", "openid offline_access fbox email profile"),
    ];
    let resp = http.post(&url).form(&params).send().await
        .context(t!("Failed to send refresh token request", "刷新令牌请求发送失败"))?;
    let token: TokenResponse = resp.json().await
        .context(t!("Failed to parse refresh token response", "刷新令牌响应解析失败"))?;
    Ok(token)
}
