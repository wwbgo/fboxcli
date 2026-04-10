use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::box_model::*;
use crate::t;

pub async fn list_grouped(client: &mut FBoxClient) -> Result<Vec<BoxGroupList>> {
    let resp = client.get("/api/client/v2/box/grouped?lite=true").await
        .context(t!("Failed to get FBox group list", "获取 FBox 分组列表失败"))?;
    let groups: Vec<BoxGroupList> = resp.json().await
        .context(t!("Failed to parse FBox group list response", "解析 FBox 分组列表响应失败"))?;
    Ok(groups)
}

pub async fn get_by_box_no(client: &mut FBoxClient, box_no: &str) -> Result<BoxReg> {
    let resp = client.get(&format!("/api/client/box/reg/boxno/{}", box_no)).await
        .context(t!("Failed to get FBox(BoxNo: {})", "获取 FBox(序列号: {}) 失败", box_no))?;
    let reg: BoxReg = resp.json().await
        .context(t!("Failed to parse FBox info response", "解析 FBox 信息响应失败"))?;
    Ok(reg)
}

pub async fn add_box(client: &mut FBoxClient, req: &AddBoxRequest) -> Result<serde_json::Value> {
    let resp = client.post_json("/api/client/box/reg", req).await
        .context(t!("Failed to add FBox", "添加 FBox 失败"))?;
    let val: serde_json::Value = resp.json().await
        .context(t!("Failed to parse add FBox response", "解析添加 FBox 响应失败"))?;
    Ok(val)
}

pub async fn rename_box(client: &mut FBoxClient, box_id: i64, alias: &str) -> Result<()> {
    let body = serde_json::json!({ "alias": alias });
    client.post_json(&format!("/api/client/v2/box/{}/alias", box_id), &body).await
        .context(t!("Failed to rename FBox(ID: {})", "重命名 FBox(ID: {}) 失败", box_id))?;
    Ok(())
}

pub async fn delete_box(client: &mut FBoxClient, box_id: i64) -> Result<()> {
    client.delete(&format!("/api/client/v2/box/{}", box_id)).await
        .context(t!("Failed to delete FBox(ID: {})", "删除 FBox(ID: {}) 失败", box_id))?;
    Ok(())
}

pub async fn get_info(client: &mut FBoxClient, box_id: i64) -> Result<BoxInfo> {
    let resp = client.get(&format!("/api/v2/box/{}/info", box_id)).await
        .context(t!("Failed to get FBox info(ID: {})", "获取 FBox(ID: {}) 详情失败", box_id))?;
    let info: BoxInfo = resp.json().await
        .context(t!("Failed to parse FBox info response", "解析 FBox 详情响应失败"))?;
    Ok(info)
}

pub async fn set_memo(client: &mut FBoxClient, box_id: i64, content: &str) -> Result<()> {
    let body = serde_json::json!({ "content": content });
    client.post_json(&format!("/api/client/box/{}/memo", box_id), &body).await
        .context(t!("Failed to set FBox memo(ID: {})", "设置 FBox(ID: {}) 备注失败", box_id))?;
    Ok(())
}
