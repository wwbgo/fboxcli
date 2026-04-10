use clap::{Parser, Subcommand};

pub mod auth;
pub mod config_cmd;
pub mod box_cmd;
pub mod group;
pub mod dmon;
pub mod history;
pub mod alarm;
pub mod contact;
pub mod device;
pub mod control;
pub mod location;

/// FBox IoT Platform CLI - Agent-native command line interface
#[derive(Parser)]
#[command(name = "fboxcli", version, about, long_about = None)]
pub struct Cli {
    /// Output in JSON format (for AI agent consumption)
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        cmd: config_cmd::ConfigCmd,
    },
    /// Authentication and token management
    Auth {
        #[command(subcommand)]
        cmd: auth::AuthCmd,
    },
    /// FBox device management
    Box {
        #[command(subcommand)]
        cmd: box_cmd::BoxCmd,
    },
    /// FBox group management
    Group {
        #[command(subcommand)]
        cmd: group::GroupCmd,
    },
    /// Data monitoring point management
    Dmon {
        #[command(subcommand)]
        cmd: dmon::DmonCmd,
    },
    /// Historical data management
    History {
        #[command(subcommand)]
        cmd: history::HistoryCmd,
    },
    /// Alarm management
    Alarm {
        #[command(subcommand)]
        cmd: alarm::AlarmCmd,
    },
    /// Alarm contact management
    Contact {
        #[command(subcommand)]
        cmd: contact::ContactCmd,
    },
    /// Device and driver management
    Device {
        #[command(subcommand)]
        cmd: device::DeviceCmd,
    },
    /// Unified write group (control) management
    Control {
        #[command(subcommand)]
        cmd: control::ControlCmd,
    },
    /// Get FBox geographic locations
    Location {
        /// Comma-separated FBox IDs
        ids: String,
    },
}
