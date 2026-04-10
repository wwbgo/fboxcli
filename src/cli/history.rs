use crate::api::FBoxClient;
use crate::models::history::HistoryQueryRequest;
use crate::output::{OutputFormat, print_list, print_success};
use crate::t;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum HistoryCmd {
    /// Query historical data
    Query {
        /// Comma-separated channel IDs
        #[arg(long)]
        ids: String,
        /// Begin time (millisecond timestamp)
        #[arg(long)]
        begin: i64,
        /// End time (millisecond timestamp)
        #[arg(long)]
        end: i64,
        /// Granularity: 0=raw, 1=minute, 2=hour, 3=day
        #[arg(long, default_value = "0")]
        granularity: i32,
        /// Max records (negative = latest N records)
        #[arg(long, default_value = "-500")]
        limit: i32,
        /// Time range type: 0=open-open, 1=open-close, 2=close-open, 3=close-close
        #[arg(long, default_value = "3")]
        tr: i32,
        /// Timezone string
        #[arg(long, default_value = "Asia/Shanghai")]
        tz: String,
        /// Fill mode: 0=none
        #[arg(long, default_value = "0")]
        fill: i32,
    },
    /// List history record items for a box
    List {
        /// FBox ID
        box_id: i64,
    },
    /// Delete history record items
    Delete {
        /// FBox ID
        box_id: i64,
        /// Comma-separated item IDs
        #[arg(long)]
        ids: String,
    },
}

pub async fn handle(cmd: HistoryCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        HistoryCmd::Query {
            ids,
            begin,
            end,
            granularity,
            limit,
            tr,
            tz,
            fill,
        } => {
            let id_list: Vec<i64> = ids
                .split(',')
                .filter_map(|v| v.trim().parse().ok())
                .collect();
            let req = HistoryQueryRequest {
                query_type: 0,
                ids: id_list,
                g: granularity,
                begin,
                end,
                tr: Some(tr),
                limit: Some(limit),
                tz: Some(tz),
                fill: Some(fill),
            };
            let result = crate::api::history::query(client, &req).await?;
            match format {
                OutputFormat::Json => crate::output::print_json(&result.rows)?,
                OutputFormat::Table => print_list(&result.rows, format)?,
            }
        }
        HistoryCmd::List { box_id } => {
            let items = crate::api::history::list_items(client, box_id).await?;
            match format {
                OutputFormat::Json => crate::output::print_json(&items)?,
                OutputFormat::Table => {
                    for item in &items {
                        println!(
                            "[{}] {} (period={}s, channels={})",
                            item.uid.map(|v| v.to_string()).unwrap_or("-".into()),
                            item.name.as_deref().unwrap_or("-"),
                            item.period.map(|v| v.to_string()).unwrap_or("-".into()),
                            item.channels.len(),
                        );
                        if !item.channels.is_empty() {
                            print_list(&item.channels, format)?;
                        }
                        println!();
                    }
                    if items.is_empty() {
                        println!("(empty)");
                    }
                }
            }
        }
        HistoryCmd::Delete { box_id, ids } => {
            let id_list: Vec<i64> = ids
                .split(',')
                .filter_map(|v| v.trim().parse().ok())
                .collect();
            crate::api::history::delete_items(client, box_id, &id_list).await?;
            print_success(&t!("History items deleted.", "历史记录项已删除。"), format)?;
        }
    }
    Ok(())
}
