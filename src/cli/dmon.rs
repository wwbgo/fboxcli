use clap::Subcommand;
use anyhow::{Result, bail};
use crate::api::FBoxClient;
use crate::models::dmon::*;
use crate::output::{OutputFormat, print_list, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum DmonCmd {
    /// List monitoring points for a box
    List {
        /// FBox ID
        box_id: i64,
    },
    /// Get monitoring point values
    GetValue {
        /// FBox ID (use -- before negative IDs)
        #[arg(long, allow_hyphen_values = true)]
        box_id: Option<i64>,
        /// FBox serial number (alternative to box_id)
        #[arg(long)]
        box_no: Option<String>,
        /// Comma-separated monitoring point IDs
        #[arg(long)]
        ids: Option<String>,
        /// Comma-separated monitoring point names
        #[arg(long)]
        names: Option<String>,
        /// Comma-separated group names (used with --names for disambiguation)
        #[arg(long)]
        groupnames: Option<String>,
        /// Timeout in milliseconds
        #[arg(long, default_value = "6000")]
        timeout: i32,
    },
    /// Write a value to a monitoring point
    SetValue {
        /// FBox ID
        #[arg(long, allow_hyphen_values = true)]
        box_id: Option<i64>,
        /// FBox serial number (alternative to box_id)
        #[arg(long)]
        box_no: Option<String>,
        /// Monitoring point ID (alternative to --name)
        #[arg(long)]
        id: Option<String>,
        /// Monitoring point name (alternative to --id)
        #[arg(long)]
        name: Option<String>,
        /// Group name (used with --name for disambiguation)
        #[arg(long)]
        groupname: Option<String>,
        /// Value to write
        #[arg(long)]
        value: String,
    },
    /// List monitoring point groups
    Groups {
        /// FBox ID
        box_id: i64,
    },
    /// Delete monitoring points
    Delete {
        /// FBox ID
        box_id: i64,
        /// Comma-separated monitoring point IDs
        #[arg(long)]
        ids: String,
    },
}

pub async fn handle(cmd: DmonCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        DmonCmd::List { box_id } => {
            let groups = crate::api::dmon::list_grouped(client, box_id).await?;
            match format {
                OutputFormat::Json => crate::output::print_json(&groups)?,
                OutputFormat::Table => {
                    let items: Vec<DmonItem> = groups.into_iter().flat_map(|g| g.items).collect();
                    print_list(&items, format)?;
                }
            }
        }
        DmonCmd::GetValue { box_id, box_no, ids, names, groupnames, timeout } => {
            if ids.is_none() && names.is_none() {
                bail!("{}", t!("Either --ids or --names is required", "必须指定 --ids 或 --names"));
            }
            let req = GetDmonValueRequest {
                ids: ids.map(|s| s.split(',').filter_map(|v| v.trim().parse().ok()).collect()),
                names: names.map(|s| s.split(',').map(|v| v.trim().to_string()).collect()),
                groupnames: groupnames.map(|s| s.split(',').map(|v| v.trim().to_string()).collect()),
                timeout: Some(timeout),
            };
            let values = crate::api::dmon::get_values_by(client, box_id, box_no.as_deref(), &req).await?;
            print_list(&values, format)?;
        }
        DmonCmd::SetValue { box_id, box_no, id, name, groupname, value } => {
            if id.is_none() && name.is_none() {
                bail!("{}", t!("Either --id or --name is required", "必须指定 --id 或 --name"));
            }
            let req = SetDmonValueRequest {
                id,
                name,
                groupname,
                value_type: Some(0),
                value,
                flag: Some(true),
            };
            crate::api::dmon::set_value_by(client, box_id, box_no.as_deref(), &req).await?;
            print_success(&t!("Value written.", "值已写入。"), format)?;
        }
        DmonCmd::Groups { box_id } => {
            let groups = crate::api::dmon::list_groups(client, box_id).await?;
            print_list(&groups, format)?;
        }
        DmonCmd::Delete { box_id, ids } => {
            let id_list: Vec<i64> = ids.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            crate::api::dmon::delete_dmons(client, box_id, &id_list).await?;
            print_success(&t!("Monitoring points deleted.", "监控点已删除。"), format)?;
        }
    }
    Ok(())
}
