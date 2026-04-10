use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::alarm::*;
use crate::t;

pub async fn list_items(client: &mut FBoxClient, box_id: i64) -> Result<Vec<AlarmItem>> {
    let body = serde_json::json!({
        "skipCount": 0,
        "maxResultCount": 100
    });
    let resp = client.post_json(&format!("/api/v4/box/{}/alarm", box_id), &body).await
        .context(t!("Failed to get alarm list(BoxID: {})", "获取告警列表(BoxID: {}) 失败", box_id))?;
    let result: AlarmListResponse = resp.json().await
        .context(t!("Failed to parse alarm list response", "解析告警列表响应失败"))?;
    Ok(result.items)
}

pub async fn get_history(client: &mut FBoxClient, box_id: i64, begin: i64, end: i64, limit: Option<i32>) -> Result<Vec<AlarmHistory>> {
    let mut url = format!("/api/v2/box/{}/alarm/data?beginTime={}&endTime={}", box_id, begin, end);
    if let Some(l) = limit {
        url.push_str(&format!("&limit={}", l));
    }
    let resp = client.get(&url).await
        .context(t!("Failed to get alarm history(BoxID: {})", "获取告警历史(BoxID: {}) 失败", box_id))?;
    let history: Vec<AlarmHistory> = resp.json().await
        .context(t!("Failed to parse alarm history response", "解析告警历史响应失败"))?;
    Ok(history)
}

pub async fn confirm(client: &mut FBoxClient, uid: i64) -> Result<()> {
    client.post_json(&format!("/api/alarm/{}/confirm", uid), &serde_json::json!({})).await
        .context(t!("Failed to confirm alarm(UID: {})", "确认告警(UID: {}) 失败", uid))?;
    Ok(())
}

pub async fn get_groups(client: &mut FBoxClient, box_id: i64) -> Result<Vec<AlarmGroupInfo>> {
    let body = serde_json::json!({ "boxId": box_id });
    let resp = client.post_json("/api/v2/alarm/group/get", &body).await
        .context(t!("Failed to get alarm groups(BoxID: {})", "获取告警分组(BoxID: {}) 失败", box_id))?;
    let groups: Vec<AlarmGroupInfo> = resp.json().await
        .context(t!("Failed to parse alarm groups response", "解析告警分组响应失败"))?;
    Ok(groups)
}

pub async fn add_group(client: &mut FBoxClient, req: &AddAlarmGroupRequest) -> Result<serde_json::Value> {
    let resp = client.put_json("/api/v2/alarm/group", req).await
        .context(t!("Failed to add alarm group", "添加告警分组失败"))?;
    let val: serde_json::Value = resp.json().await
        .context(t!("Failed to parse add alarm group response", "解析添加告警分组响应失败"))?;
    Ok(val)
}

pub async fn delete_group(client: &mut FBoxClient, box_id: i64, uid: i64) -> Result<()> {
    let body = serde_json::json!({ "uid": uid, "boxId": box_id });
    client.post_json("/api/v2/alarm/group/del", &body).await
        .context(t!("Failed to delete alarm group(UID: {})", "删除告警分组(UID: {}) 失败", uid))?;
    Ok(())
}

pub async fn delete_items(client: &mut FBoxClient, box_id: i64, ids: &[i64]) -> Result<()> {
    client.post_json(&format!("/api/v2/box/{}/alarm/del", box_id), &ids).await
        .context(t!("Failed to delete alarm items(BoxID: {})", "删除告警项(BoxID: {}) 失败", box_id))?;
    Ok(())
}
