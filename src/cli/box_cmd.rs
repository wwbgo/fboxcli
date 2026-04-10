use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::models::box_model::*;
use crate::models::enums::BoxConnectionState;
use crate::output::{OutputFormat, print_list, print_single, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum BoxCmd {
    /// List FBox devices
    List {
        /// Show all devices (default: online only)
        #[arg(short, long)]
        all: bool,
    },
    /// Get FBox by serial number
    Get {
        /// FBox serial number
        box_no: String,
    },
    /// Add a new FBox
    Add {
        /// FBox serial number
        box_no: String,
        /// FBox password
        password: String,
        /// Alias name
        #[arg(long)]
        alias: Option<String>,
        /// Group name
        #[arg(long)]
        group: Option<String>,
    },
    /// Rename an FBox
    Rename {
        /// FBox ID
        box_id: i64,
        /// New alias
        alias: String,
    },
    /// Delete an FBox
    Delete {
        /// FBox ID
        box_id: i64,
    },
    /// Get FBox detailed info
    Info {
        /// FBox ID
        box_id: i64,
    },
    /// Set FBox memo/note
    Memo {
        /// FBox ID
        box_id: i64,
        /// Memo content
        content: String,
    },
}

pub async fn handle(cmd: BoxCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        BoxCmd::List { all } => {
            let groups = crate::api::box_api::list_grouped(client).await?;
            match format {
                OutputFormat::Json => {
                    if all {
                        crate::output::print_json(&groups)?;
                    } else {
                        let filtered: Vec<&BoxReg> = groups.iter()
                            .flat_map(|g| g.box_regs.iter())
                            .filter(|b| b.conn_state == Some(BoxConnectionState::Connected))
                            .collect();
                        crate::output::print_json(&filtered)?;
                    }
                }
                OutputFormat::Table => {
                    let mut all_boxes: Vec<BoxReg> = Vec::new();
                    for g in &groups {
                        let gname = g.name.as_deref();
                        for b in &g.box_regs {
                            if !all && b.conn_state != Some(BoxConnectionState::Connected) {
                                continue;
                            }
                            let mut b = b.clone();
                            b.group_name = gname.map(|s| s.to_string());
                            all_boxes.push(b);
                        }
                    }
                    print_list(&all_boxes, format)?;
                }
            }
        }
        BoxCmd::Get { box_no } => {
            let reg = crate::api::box_api::get_by_box_no(client, &box_no).await?;
            print_single(&reg, format)?;
        }
        BoxCmd::Add { box_no, password, alias, group } => {
            let req = AddBoxRequest {
                alias: alias.unwrap_or_else(|| box_no.clone()),
                box_no,
                box_password: password,
                group_id: None,
                group_name: group,
            };
            let result = crate::api::box_api::add_box(client, &req).await?;
            print_single(&result, format)?;
        }
        BoxCmd::Rename { box_id, alias } => {
            crate::api::box_api::rename_box(client, box_id, &alias).await?;
            print_success(&t!("Box renamed.", "FBox 已重命名。"), format)?;
        }
        BoxCmd::Delete { box_id } => {
            crate::api::box_api::delete_box(client, box_id).await?;
            print_success(&t!("Box deleted.", "FBox 已删除。"), format)?;
        }
        BoxCmd::Info { box_id } => {
            let info = crate::api::box_api::get_info(client, box_id).await?;
            print_single(&info, format)?;
        }
        BoxCmd::Memo { box_id, content } => {
            crate::api::box_api::set_memo(client, box_id, &content).await?;
            print_success(&t!("Memo updated.", "备注已更新。"), format)?;
        }
    }
    Ok(())
}
