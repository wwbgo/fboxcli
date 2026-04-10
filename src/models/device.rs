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
pub struct DeviceItem {
    #[tabled(rename = "ID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub id: Option<i64>,
    #[tabled(rename = "Alias", display = "display_option")]
    #[serde(rename = "alias", default)]
    pub alias: Option<String>,
    #[tabled(rename = "Driver", display = "display_option")]
    #[serde(rename = "driverName", default)]
    pub driver_name: Option<String>,
    #[tabled(rename = "Station", display = "display_option")]
    #[serde(default)]
    pub station: Option<i32>,
    #[tabled(skip)]
    #[serde(default)]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct DriverSpec {
    #[tabled(rename = "ID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub id: Option<i64>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Description", display = "display_option")]
    #[serde(rename = "desc", default)]
    pub desc: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub regs: Option<Vec<serde_json::Value>>,
}
