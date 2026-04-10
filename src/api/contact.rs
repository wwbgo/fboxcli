use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::contact::*;
use crate::t;

pub async fn list_contacts(client: &mut FBoxClient) -> Result<Vec<Contact>> {
    let body = serde_json::json!({
        "enabled": null,
        "skipCount": 0,
        "maxResultCount": 100,
        "excludeAlarmIds": [],
        "phoneFormat": "raw"
    });
    let resp = client.post_json("/api/v2/contacts", &body).await
        .context(t!("Failed to get contact list", "获取联系人列表失败"))?;
    let result: ContactListResponse = resp.json().await
        .context(t!("Failed to parse contact list response", "解析联系人列表响应失败"))?;
    Ok(result.items)
}

pub async fn get_contact(client: &mut FBoxClient, uid: i64) -> Result<Contact> {
    let resp = client.get(&format!("/api/v2/contact/{}", uid)).await
        .context(t!("Failed to get contact(UID: {})", "获取联系人(UID: {}) 失败", uid))?;
    let contact: Contact = resp.json().await
        .context(t!("Failed to parse contact response", "解析联系人信息响应失败"))?;
    Ok(contact)
}

pub async fn add_contact(client: &mut FBoxClient, req: &AddContactRequest) -> Result<serde_json::Value> {
    let resp = client.put_json("/api/v2/contact", req).await
        .context(t!("Failed to add contact", "添加联系人失败"))?;
    let val: serde_json::Value = resp.json().await
        .context(t!("Failed to parse add contact response", "解析添加联系人响应失败"))?;
    Ok(val)
}

pub async fn update_contact(client: &mut FBoxClient, data: &serde_json::Value) -> Result<()> {
    client.post_json("/api/v2/contact", data).await
        .context(t!("Failed to update contact", "更新联系人失败"))?;
    Ok(())
}

pub async fn delete_contact(client: &mut FBoxClient, uid: i64) -> Result<()> {
    client.delete(&format!("/api/v2/contact/{}", uid)).await
        .context(t!("Failed to delete contact(UID: {})", "删除联系人(UID: {}) 失败", uid))?;
    Ok(())
}
