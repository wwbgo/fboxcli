use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::box_model::BoxGroup;
use crate::t;

pub async fn list_groups(client: &mut FBoxClient) -> Result<Vec<BoxGroup>> {
    let resp = client.get("/api/client/v2/box/grouped/without/box").await
        .context(t!("Failed to get group list", "获取分组列表失败"))?;
    let groups: Vec<BoxGroup> = resp.json().await
        .context(t!("Failed to parse group list response", "解析分组列表响应失败"))?;
    Ok(groups)
}

pub async fn add_group(client: &mut FBoxClient, name: &str, prev_id: Option<i64>) -> Result<serde_json::Value> {
    let mut body = serde_json::json!({ "name": name });
    if let Some(pid) = prev_id {
        body["prevId"] = serde_json::json!(pid);
    }
    let resp = client.put_json("/api/client/group", &body).await
        .context(t!("Failed to add group '{}'", "添加分组 '{}' 失败", name))?;
    let val: serde_json::Value = resp.json().await
        .context(t!("Failed to parse add group response", "解析添加分组响应失败"))?;
    Ok(val)
}

pub async fn rename_group(client: &mut FBoxClient, group_id: i64, name: &str) -> Result<()> {
    let body = serde_json::json!({ "id": group_id, "name": name });
    client.post_json("/api/client/v2/box/group/mgt", &body).await
        .context(t!("Failed to rename group(ID: {})", "重命名分组(ID: {}) 失败", group_id))?;
    Ok(())
}

pub async fn delete_group(client: &mut FBoxClient, group_id: i64) -> Result<()> {
    client.delete(&format!("/api/client/group/{}", group_id)).await
        .context(t!("Failed to delete group(ID: {})", "删除分组(ID: {}) 失败", group_id))?;
    Ok(())
}
