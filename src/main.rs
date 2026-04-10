mod api;
mod cli;
mod config;
mod crypto;
mod i18n;
mod models;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use output::OutputFormat;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let format = OutputFormat::from_json_flag(args.json);

    if let Err(err) = run(args.command, format).await {
        output::print_error(&err, format);
        std::process::exit(1);
    }
}

async fn run(command: Commands, format: OutputFormat) -> Result<()> {
    match command {
        Commands::Config { cmd } => {
            cli::config_cmd::handle(cmd, format)?;
        }
        Commands::Auth { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::auth::handle(cmd, &mut client, format).await?;
        }
        Commands::Box { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::box_cmd::handle(cmd, &mut client, format).await?;
        }
        Commands::Group { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::group::handle(cmd, &mut client, format).await?;
        }
        Commands::Dmon { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::dmon::handle(cmd, &mut client, format).await?;
        }
        Commands::History { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::history::handle(cmd, &mut client, format).await?;
        }
        Commands::Alarm { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::alarm::handle(cmd, &mut client, format).await?;
        }
        Commands::Contact { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::contact::handle(cmd, &mut client, format).await?;
        }
        Commands::Device { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::device::handle(cmd, &mut client, format).await?;
        }
        Commands::Control { cmd } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::control::handle(cmd, &mut client, format).await?;
        }
        Commands::Location { ids } => {
            let cfg = config::load_config()?;
            let mut client = api::FBoxClient::new(cfg)?;
            cli::location::handle(&ids, &mut client, format).await?;
        }
    }

    Ok(())
}
