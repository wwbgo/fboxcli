use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::models::control::WriteControlValueRequest;
use crate::output::{OutputFormat, print_list, print_single, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum ControlCmd {
    /// List all control groups
    List,
    /// Get a control group by ID
    Get {
        /// Group ID
        group_id: i64,
    },
    /// Add a control group from JSON
    Add {
        /// JSON string with group definition
        json_data: String,
    },
    /// Delete control groups
    Delete {
        /// Comma-separated group IDs
        #[arg(long)]
        ids: String,
    },
    /// Write value to a control group
    Write {
        /// Group UID
        #[arg(long)]
        uid: Option<i64>,
        /// Group name (alternative to uid)
        #[arg(long)]
        name: Option<String>,
        /// Value to write
        #[arg(long)]
        value: String,
    },
}

pub async fn handle(cmd: ControlCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        ControlCmd::List => {
            let groups = crate::api::control::list_groups(client).await?;
            print_list(&groups, format)?;
        }
        ControlCmd::Get { group_id } => {
            let group = crate::api::control::get_group(client, group_id).await?;
            print_single(&group, format)?;
        }
        ControlCmd::Add { json_data } => {
            let data: serde_json::Value = serde_json::from_str(&json_data)?;
            let result = crate::api::control::add_group(client, &data).await?;
            print_single(&result, format)?;
        }
        ControlCmd::Delete { ids } => {
            let id_list: Vec<i64> = ids.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            crate::api::control::delete_groups(client, &id_list).await?;
            print_success(&t!("Control groups deleted.", "控制组已删除。"), format)?;
        }
        ControlCmd::Write { uid, name, value } => {
            let val: serde_json::Value = serde_json::from_str(&value)
                .unwrap_or(serde_json::Value::String(value));
            let req = WriteControlValueRequest { uid, name, value: val };
            crate::api::control::write_value(client, &req).await?;
            print_success(&t!("Value written.", "值已写入。"), format)?;
        }
    }
    Ok(())
}
