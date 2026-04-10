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
pub struct ControlGroup {
    #[tabled(rename = "UID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub uid: Option<i64>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Type", display = "display_option")]
    #[serde(rename = "type", default)]
    pub data_type: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "controlOptions", default)]
    pub control_options: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize)]
pub struct WriteControlValueRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub value: serde_json::Value,
}
