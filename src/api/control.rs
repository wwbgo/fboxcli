use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::control::*;
use crate::t;

pub async fn list_groups(client: &mut FBoxClient) -> Result<Vec<ControlGroup>> {
    let resp = client.get("/api/v2/control/groups").await
        .context(t!("Failed to get control group list", "获取控制组列表失败"))?;
    let groups: Vec<ControlGroup> = resp.json().await
        .context(t!("Failed to parse control group list response", "解析控制组列表响应失败"))?;
    Ok(groups)
}

pub async fn get_group(client: &mut FBoxClient, group_id: i64) -> Result<ControlGroup> {
    let resp = client.get(&format!("/api/v2/control/group/{}", group_id)).await
        .context(t!("Failed to get control group(ID: {})", "获取控制组(ID: {}) 失败", group_id))?;
    let group: ControlGroup = resp.json().await
        .context(t!("Failed to parse control group response", "解析控制组信息响应失败"))?;
    Ok(group)
}

pub async fn add_group(client: &mut FBoxClient, data: &serde_json::Value) -> Result<serde_json::Value> {
    let resp = client.put_json("/api/v2/control/group", data).await
        .context(t!("Failed to add control group", "添加控制组失败"))?;
    let val: serde_json::Value = resp.json().await
        .context(t!("Failed to parse add control group response", "解析添加控制组响应失败"))?;
    Ok(val)
}

pub async fn delete_groups(client: &mut FBoxClient, uids: &[i64]) -> Result<()> {
    let body = serde_json::json!({ "uids": uids });
    client.post_json("/api/v2/control/group/del", &body).await
        .context(t!("Failed to delete control groups", "删除控制组失败"))?;
    Ok(())
}

pub async fn write_value(client: &mut FBoxClient, req: &WriteControlValueRequest) -> Result<()> {
    client.post_json("/api/v2/control/group/write/value", req).await
        .context(t!("Failed to write control group value", "写入控制组值失败"))?;
    Ok(())
}
