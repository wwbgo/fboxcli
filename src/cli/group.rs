use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::output::{OutputFormat, print_list, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum GroupCmd {
    /// List all groups
    List,
    /// Add a new FBox group
    Add {
        /// Group name
        name: String,
        /// Parent group ID
        #[arg(long)]
        parent: Option<i64>,
    },
    /// Rename an FBox group
    Rename {
        /// Group ID
        group_id: i64,
        /// New name
        name: String,
    },
    /// Delete an FBox group
    Delete {
        /// Group ID
        group_id: i64,
    },
}

pub async fn handle(cmd: GroupCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        GroupCmd::List => {
            let groups = crate::api::group::list_groups(client).await?;
            print_list(&groups, format)?;
        }
        GroupCmd::Add { name, parent } => {
            let result = crate::api::group::add_group(client, &name, parent).await?;
            crate::output::print_single(&result, format)?;
        }
        GroupCmd::Rename { group_id, name } => {
            crate::api::group::rename_group(client, group_id, &name).await?;
            print_success(&t!("Group renamed.", "分组已重命名。"), format)?;
        }
        GroupCmd::Delete { group_id } => {
            crate::api::group::delete_group(client, group_id).await?;
            print_success(&t!("Group deleted.", "分组已删除。"), format)?;
        }
    }
    Ok(())
}
