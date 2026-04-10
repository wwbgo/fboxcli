use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::config;
use crate::output::{OutputFormat, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum AuthCmd {
    /// Login and obtain access token
    Login {
        /// FlexManager username (for password grant)
        #[arg(short, long)]
        username: Option<String>,
        /// FlexManager password (for password grant)
        #[arg(short, long)]
        password: Option<String>,
        /// Server URL
        #[arg(long)]
        server: Option<String>,
        /// Developer client ID (for developer mode)
        #[arg(long)]
        client_id: Option<String>,
        /// Developer client secret (for developer mode)
        #[arg(long)]
        client_secret: Option<String>,
        /// Login mode: "developer" (client_credentials) or "user" (password)
        #[arg(long, value_name = "MODE")]
        login_mode: Option<String>,
    },
    /// Show current cached token info
    Token,
    /// Clear cached token
    Logout,
}

pub async fn handle(cmd: AuthCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        AuthCmd::Login { username, password, server, client_id, client_secret, login_mode } => {
            if let Some(s) = server {
                client.config.server = s;
            }
            if let Some(id) = client_id {
                client.config.client_id = id;
            }
            if let Some(secret) = client_secret {
                client.config.client_secret = secret;
            }
            if let Some(mode) = login_mode {
                let gt = config::resolve_grant_type(&mode)?;
                client.config.grant_type = gt.to_string();
            }
            if let (Some(u), Some(p)) = (username, password) {
                client.config.grant_type = "password".to_string();
                client.config.username = Some(u);
                client.config.password = Some(p);
            }
            crate::config::save_config(&client.config)?;
            let token = client.ensure_token().await?;
            match format {
                OutputFormat::Json => {
                    println!(r#"{{"status":"ok","access_token":"{}"}}"#, token);
                }
                OutputFormat::Table => {
                    println!("{}", t!("Login successful. Token cached.", "登录成功，令牌已缓存。"));
                }
            }
        }
        AuthCmd::Token => {
            match crate::config::load_token()? {
                Some(t) => {
                    crate::output::print_single(&t, format)?;
                }
                None => {
                    print_success(&t!("No cached token. Run `fboxcli auth login` first.", "无缓存令牌，请先运行 `fboxcli auth login`。"), format)?;
                }
            }
        }
        AuthCmd::Logout => {
            crate::config::clear_token()?;
            print_success(&t!("Token cleared.", "令牌已清除。"), format)?;
        }
    }
    Ok(())
}
