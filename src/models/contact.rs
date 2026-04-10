use serde::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::serde_helper::option_string_i64;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Contact {
    #[tabled(rename = "UID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub uid: Option<i64>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Email", display = "display_option")]
    #[serde(default)]
    pub email: Option<String>,
    #[tabled(rename = "Phone", display = "display_option")]
    #[serde(default)]
    pub cellphone: Option<String>,
    #[tabled(rename = "Enabled", display = "display_option")]
    #[serde(default)]
    pub enabled: Option<bool>,
    #[tabled(rename = "NoticeType", display = "display_option")]
    #[serde(rename = "noticeType", default)]
    pub notice_type: Option<i32>,
    #[tabled(skip)]
    #[serde(default)]
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactListResponse {
    #[serde(rename = "totalCount", default)]
    pub total_count: Option<String>,
    #[serde(default)]
    pub items: Vec<Contact>,
}

#[derive(Debug, Serialize)]
pub struct AddContactRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellphone: Option<String>,
    pub enabled: bool,
    #[serde(rename = "noticeType", default)]
    pub notice_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}
