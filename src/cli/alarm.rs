use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::models::alarm::AddAlarmGroupRequest;
use crate::output::{OutputFormat, print_list, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum AlarmCmd {
    /// List alarm items for a box
    List {
        /// FBox ID
        box_id: i64,
    },
    /// Get alarm history
    History {
        /// FBox ID
        box_id: i64,
        /// Begin time (millisecond timestamp)
        #[arg(long)]
        begin: i64,
        /// End time (millisecond timestamp)
        #[arg(long)]
        end: i64,
        /// Max number of records
        #[arg(long)]
        limit: Option<i32>,
    },
    /// Confirm an alarm
    Confirm {
        /// Alarm UID
        uid: i64,
    },
    /// List alarm groups
    Groups {
        /// FBox ID
        box_id: i64,
    },
    /// Add an alarm group
    AddGroup {
        /// FBox ID
        box_id: i64,
        /// Group name
        name: String,
        /// Comma-separated contact UIDs
        #[arg(long)]
        contacts: Option<String>,
    },
    /// Delete an alarm group
    DeleteGroup {
        /// FBox ID
        box_id: i64,
        /// Group UID
        uid: i64,
    },
    /// Delete alarm items
    Delete {
        /// FBox ID
        box_id: i64,
        /// Comma-separated alarm item IDs
        #[arg(long)]
        ids: String,
    },
}

pub async fn handle(cmd: AlarmCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        AlarmCmd::List { box_id } => {
            let items = crate::api::alarm::list_items(client, box_id).await?;
            print_list(&items, format)?;
        }
        AlarmCmd::History { box_id, begin, end, limit } => {
            let history = crate::api::alarm::get_history(client, box_id, begin, end, limit).await?;
            print_list(&history, format)?;
        }
        AlarmCmd::Confirm { uid } => {
            crate::api::alarm::confirm(client, uid).await?;
            print_success(&t!("Alarm confirmed.", "告警已确认。"), format)?;
        }
        AlarmCmd::Groups { box_id } => {
            let groups = crate::api::alarm::get_groups(client, box_id).await?;
            crate::output::print_json(&groups)?;
        }
        AlarmCmd::AddGroup { box_id, name, contacts } => {
            let cuids = contacts.map(|s| s.split(',').filter_map(|v| v.trim().parse().ok()).collect());
            let req = AddAlarmGroupRequest { box_id, name, memo: None, cuids };
            let result = crate::api::alarm::add_group(client, &req).await?;
            crate::output::print_single(&result, format)?;
        }
        AlarmCmd::DeleteGroup { box_id, uid } => {
            crate::api::alarm::delete_group(client, box_id, uid).await?;
            print_success(&t!("Alarm group deleted.", "告警分组已删除。"), format)?;
        }
        AlarmCmd::Delete { box_id, ids } => {
            let id_list: Vec<i64> = ids.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            crate::api::alarm::delete_items(client, box_id, &id_list).await?;
            print_success(&t!("Alarm items deleted.", "告警项已删除。"), format)?;
        }
    }
    Ok(())
}
