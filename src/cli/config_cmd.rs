use clap::Subcommand;
use anyhow::Result;
use crate::config::{self, AppConfig};
use crate::output::{OutputFormat, print_single, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum ConfigCmd {
    /// Set configuration values
    Set {
        /// Server URL
        #[arg(long)]
        server: Option<String>,
        /// Login mode: "developer" (client_credentials) or "user" (password)
        #[arg(long, value_name = "MODE")]
        login_mode: Option<String>,
    },
    /// Show current configuration
    Show,
    /// Reset configuration to defaults
    Reset,
}

pub fn handle(cmd: ConfigCmd, format: OutputFormat) -> Result<()> {
    match cmd {
        ConfigCmd::Set { server, login_mode } => {
            let mut cfg = config::load_config()?;
            if let Some(s) = server { cfg.server = s; }
            if let Some(mode) = login_mode {
                let gt = config::resolve_grant_type(&mode)?;
                cfg.grant_type = gt.to_string();
            }
            config::save_config(&cfg)?;
            print_success(&t!("Configuration saved.", "配置已保存。"), format)?;
        }
        ConfigCmd::Show => {
            let cfg = config::load_config()?;
            print_single(&cfg, format)?;
        }
        ConfigCmd::Reset => {
            config::save_config(&AppConfig::default())?;
            config::clear_token()?;
            print_success(&t!("Configuration reset to defaults.", "配置已重置为默认值。"), format)?;
        }
    }
    Ok(())
}
