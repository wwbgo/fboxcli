use anyhow::Result;
use crate::api::FBoxClient;
use crate::output::OutputFormat;

pub async fn handle(ids: &str, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    let id_list: Vec<i64> = ids.split(',').filter_map(|v| v.trim().parse().ok()).collect();
    let locations = crate::api::location::get_locations(client, &id_list).await?;
    crate::output::print_list(&locations, format)?;
    Ok(())
}
