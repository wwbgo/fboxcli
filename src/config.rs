use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::crypto;
use crate::t;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: String,
    pub client_id: String,
    pub client_secret: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default = "default_grant_type")]
    pub grant_type: String,
    #[serde(default)]
    pub client_uid: Option<String>,
}

fn default_grant_type() -> String {
    "user".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: "https://openapi.fbox360.com".to_string(),
            client_id: String::new(),
            client_secret: String::new(),
            username: None,
            password: None,
            grant_type: default_grant_type(),
            client_uid: None,
        }
    }
}

/// Map user-friendly login mode to OAuth2 grant_type.
///   "developer" -> "client_credentials"
///   "user"      -> "password"
/// Also accepts raw grant_type values for backward compatibility.
pub fn resolve_grant_type(mode: &str) -> Result<&'static str> {
    match mode {
        "developer" | "client_credentials" => Ok("client_credentials"),
        "user" | "password" => Ok("password"),
        _ => bail!("{}", t!("Unknown login mode '{}'. Use 'developer' or 'user'.", "未知登录模式 '{}'，请使用 'developer' 或 'user'。", mode)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64,
    pub grant_type: String,
}

pub fn config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context(t!("Cannot find home directory", "无法找到主目录"))?;
    let dir = home.join(".fboxcli");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn token_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("token.json"))
}

pub fn load_config() -> Result<AppConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(&path)?;
    let mut config: AppConfig = toml::from_str(&content)?;
    // Decrypt sensitive fields (backward compatible with plaintext)
    config.client_secret = crypto::decrypt(&config.client_secret)?;
    if let Some(ref pw) = config.password {
        config.password = Some(crypto::decrypt(pw)?);
    }
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path()?;
    // Encrypt sensitive fields before saving
    let mut to_save = config.clone();
    to_save.client_secret = crypto::encrypt(&to_save.client_secret)?;
    if let Some(ref pw) = to_save.password {
        to_save.password = Some(crypto::encrypt(pw)?);
    }
    let content = toml::to_string_pretty(&to_save)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn load_token() -> Result<Option<CachedToken>> {
    let path = token_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)?;
    let token: CachedToken = serde_json::from_str(&content)?;
    Ok(Some(token))
}

pub fn save_token(token: &CachedToken) -> Result<()> {
    let path = token_path()?;
    let content = serde_json::to_string_pretty(token)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn clear_token() -> Result<()> {
    let path = token_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn ensure_client_uid(config: &mut AppConfig) -> Result<String> {
    if let Some(ref uid) = config.client_uid {
        return Ok(uid.clone());
    }
    let uid = format!("cli_{}", uuid::Uuid::new_v4());
    config.client_uid = Some(uid.clone());
    save_config(config)?;
    Ok(uid)
}
