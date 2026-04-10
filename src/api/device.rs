use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::device::*;
use crate::t;

pub async fn list_devices(client: &mut FBoxClient, box_id: i64) -> Result<Vec<DeviceItem>> {
    let resp = client.get(&format!("/api/v2/box/{}/device", box_id)).await
        .context(t!("Failed to get device list(BoxID: {})", "获取设备列表(BoxID: {}) 失败", box_id))?;
    let items: Vec<DeviceItem> = resp.json().await
        .context(t!("Failed to parse device list response", "解析设备列表响应失败"))?;
    Ok(items)
}

pub async fn get_drivers(client: &mut FBoxClient, box_type: i32) -> Result<Vec<DriverSpec>> {
    let resp = client.get(&format!("/api/device/spec/{}", box_type)).await
        .context(t!("Failed to get driver list(BoxType: {})", "获取驱动列表(BoxType: {}) 失败", box_type))?;
    let specs: Vec<DriverSpec> = resp.json().await
        .context(t!("Failed to parse driver list response", "解析驱动列表响应失败"))?;
    Ok(specs)
}

pub async fn get_registers(client: &mut FBoxClient, device_id: i64) -> Result<serde_json::Value> {
    let resp = client.get(&format!("/api/device/{}/spec", device_id)).await
        .context(t!("Failed to get registers(DeviceID: {})", "获取寄存器信息(DeviceID: {}) 失败", device_id))?;
    let val: serde_json::Value = resp.json().await
        .context(t!("Failed to parse registers response", "解析寄存器信息响应失败"))?;
    Ok(val)
}
