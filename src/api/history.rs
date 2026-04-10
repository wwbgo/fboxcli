use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::history::*;
use crate::t;

pub async fn query(client: &mut FBoxClient, req: &HistoryQueryRequest) -> Result<HistoryQueryResponse> {
    let resp = client.post_json("/hs/v2/hdata/get", req).await
        .context(t!("Failed to query history data", "查询历史数据失败"))?;
    let result: HistoryQueryResponse = resp.json().await
        .context(t!("Failed to parse history data response", "解析历史数据响应失败"))?;
    Ok(result)
}

pub async fn list_items(client: &mut FBoxClient, box_id: i64) -> Result<Vec<HistoryItem>> {
    let resp = client.get(&format!("/api/v2/box/{}/hdataitems", box_id)).await
        .context(t!("Failed to get history items(BoxID: {})", "获取历史记录项(BoxID: {}) 失败", box_id))?;
    let items: Vec<HistoryItem> = resp.json().await
        .context(t!("Failed to parse history items response", "解析历史记录项响应失败"))?;
    Ok(items)
}

pub async fn delete_items(client: &mut FBoxClient, box_id: i64, ids: &[i64]) -> Result<()> {
    let body = serde_json::json!({ "ids": ids });
    client.post_json(&format!("/api/v2/box/{}/hdataitems/del", box_id), &body).await
        .context(t!("Failed to delete history items(BoxID: {})", "删除历史记录项(BoxID: {}) 失败", box_id))?;
    Ok(())
}
