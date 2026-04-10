use anyhow::{Context, Result, bail};
use crate::api::FBoxClient;
use crate::models::dmon::*;
use crate::t;

pub async fn list_grouped(client: &mut FBoxClient, box_id: i64) -> Result<Vec<DmonGroupList>> {
    let resp = client.get(&format!("/api/v2/box/{}/dmon/grouped", box_id)).await
        .context(t!("Failed to get dmon list(BoxID: {})", "获取监控点列表(BoxID: {}) 失败", box_id))?;
    let groups: Vec<DmonGroupList> = resp.json().await
        .context(t!("Failed to parse dmon list response", "解析监控点列表响应失败"))?;
    Ok(groups)
}

/// Get dmon values by box_id or box_no
/// URL1: /api/v2/box/{boxId}/dmon/value/get (by ID)
/// URL2: /api/v2/dmon/value/get?boxNo={boxNo} (by serial number)
pub async fn get_values_by(
    client: &mut FBoxClient,
    box_id: Option<i64>,
    box_no: Option<&str>,
    req: &GetDmonValueRequest,
) -> Result<Vec<DmonValue>> {
    let path = match (box_id, box_no) {
        (Some(id), _) => format!("/api/v2/box/{}/dmon/value/get", id),
        (None, Some(no)) => format!("/api/v2/dmon/value/get?boxNo={}", no),
        _ => bail!("{}", t!("Either --box-id or --box-no is required", "必须指定 --box-id 或 --box-no")),
    };
    let resp = client.post_json(&path, req).await
        .context(t!("Failed to get dmon values", "获取监控点值失败"))?;
    let values: Vec<DmonValue> = resp.json().await
        .context(t!("Failed to parse dmon values response", "解析监控点值响应失败"))?;
    Ok(values)
}

/// Write dmon value by box_id or box_no
/// URL1: /api/v2/box/{boxId}/dmon/value (by ID)
/// URL2: /api/v2/dmon/value?boxNo={boxNo} (by serial number)
pub async fn set_value_by(
    client: &mut FBoxClient,
    box_id: Option<i64>,
    box_no: Option<&str>,
    req: &SetDmonValueRequest,
) -> Result<()> {
    let path = match (box_id, box_no) {
        (Some(id), _) => format!("/api/v2/box/{}/dmon/value", id),
        (None, Some(no)) => format!("/api/v2/dmon/value?boxNo={}", no),
        _ => bail!("{}", t!("Either --box-id or --box-no is required", "必须指定 --box-id 或 --box-no")),
    };
    client.post_json(&path, req).await
        .context(t!("Failed to write dmon value", "写入监控点值失败"))?;
    Ok(())
}

pub async fn list_groups(client: &mut FBoxClient, box_id: i64) -> Result<Vec<DmonGroup>> {
    let resp = client.get(&format!("/api/v2/box/{}/dmon/groups", box_id)).await
        .context(t!("Failed to get dmon groups(BoxID: {})", "获取监控点分组(BoxID: {}) 失败", box_id))?;
    let groups: Vec<DmonGroup> = resp.json().await
        .context(t!("Failed to parse dmon groups response", "解析监控点分组响应失败"))?;
    Ok(groups)
}

pub async fn delete_dmons(client: &mut FBoxClient, box_id: i64, ids: &[i64]) -> Result<()> {
    client.post_json(&format!("/api/v2/box/{}/dmon/del", box_id), &ids).await
        .context(t!("Failed to delete dmons(BoxID: {})", "删除监控点(BoxID: {}) 失败", box_id))?;
    Ok(())
}
