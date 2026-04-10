use anyhow::{bail, Result};
use reqwest::{Client, Method, RequestBuilder, Response};
use crate::config::{self, AppConfig, CachedToken};
use crate::models::auth::TokenResponse;
use crate::t;

pub mod auth;
pub mod box_api;
pub mod dmon;
pub mod group;
pub mod alarm;
pub mod history;
pub mod device;
pub mod contact;
pub mod control;
pub mod location;

pub struct FBoxClient {
    pub http: Client,
    pub config: AppConfig,
    token: Option<CachedToken>,
}

impl FBoxClient {
    pub fn new(mut config: AppConfig) -> Result<Self> {
        // Resolve friendly login mode to OAuth2 grant_type
        let gt = config::resolve_grant_type(&config.grant_type)?;
        config.grant_type = gt.to_string();
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        let token = config::load_token()?;
        Ok(Self { http, config, token })
    }

    pub async fn ensure_token(&mut self) -> Result<String> {
        if let Some(ref token) = self.token {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as i64;
            if now < token.expires_at - 60 {
                return Ok(token.access_token.clone());
            }
            // Try refresh - clone the refresh token to avoid borrow conflict
            if let Some(rt) = token.refresh_token.clone() {
                if let Ok(new_token) = self.refresh_token(&rt).await {
                    return Ok(new_token);
                }
            }
        }
        // Login fresh
        let token = self.login().await?;
        Ok(token)
    }

    async fn login(&mut self) -> Result<String> {
        let resp = auth::login(&self.http, &self.config).await?;
        let cached = self.cache_token_response(&resp)?;
        Ok(cached.access_token.clone())
    }

    async fn refresh_token(&mut self, refresh_token: &str) -> Result<String> {
        let resp = auth::refresh(&self.http, &self.config, refresh_token).await?;
        let cached = self.cache_token_response(&resp)?;
        Ok(cached.access_token.clone())
    }

    fn cache_token_response(&mut self, resp: &TokenResponse) -> Result<&CachedToken> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let cached = CachedToken {
            access_token: resp.access_token.clone(),
            refresh_token: resp.refresh_token.clone(),
            expires_at: now + resp.expires_in,
            grant_type: self.config.grant_type.clone(),
        };
        config::save_token(&cached)?;
        self.token = Some(cached);
        Ok(self.token.as_ref().unwrap())
    }

    pub async fn request(&mut self, method: Method, path: &str) -> Result<RequestBuilder> {
        let token = self.ensure_token().await?;
        let url = format!("{}{}", self.config.server, path);
        Ok(self.http.request(method, &url).bearer_auth(token))
    }

    pub async fn get(&mut self, path: &str) -> Result<Response> {
        let req = self.request(Method::GET, path).await?;
        let resp = req.send().await?;
        check_response(resp).await
    }

    pub async fn post_json<T: serde::Serialize>(&mut self, path: &str, body: &T) -> Result<Response> {
        let req = self.request(Method::POST, path).await?;
        let resp = req.json(body).send().await?;
        check_response(resp).await
    }

    pub async fn put_json<T: serde::Serialize>(&mut self, path: &str, body: &T) -> Result<Response> {
        let req = self.request(Method::PUT, path).await?;
        let resp = req.json(body).send().await?;
        check_response(resp).await
    }

    pub async fn delete(&mut self, path: &str) -> Result<Response> {
        let req = self.request(Method::DELETE, path).await?;
        let resp = req.send().await?;
        check_response(resp).await
    }
}

pub(crate) async fn check_response(resp: Response) -> Result<Response> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let fbox_code = resp
        .headers()
        .get("X-FBox-Code")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let body = resp.text().await.unwrap_or_default();
    match status.as_u16() {
        429 => bail!("{}", t!("Rate limited (80/min, 3000/hour, 30000/day)", "请求频率超限 (限制: 80次/分, 3000次/时, 30000次/天)")),
        401 => bail!("{}", t!("Auth failed, please re-login (fboxcli auth login)", "认证失败，请重新登录 (fboxcli auth login)")),
        403 => bail!("{}", t!("Permission denied", "权限不足，无法访问该资源")),
        404 => bail!("{}: {}", t!("Resource not found", "资源不存在"), body),
        _ => {
            if let Some(code) = fbox_code {
                bail!("{} (code: {}): {}", t!("API error", "API 错误"), code, body);
            }
            bail!("HTTP {} : {}", status, body);
        }
    }
}
