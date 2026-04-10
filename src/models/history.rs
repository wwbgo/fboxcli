use serde::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::serde_helper::option_string_i64;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

fn display_channels(channels: &Vec<HistoryChannel>) -> String {
    channels.len().to_string()
}

fn display_i64(v: &i64) -> String {
    v.to_string()
}

fn display_values(v: &Vec<serde_json::Value>) -> String {
    v.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
}

/// History data query request (API: POST /hs/v2/hdata/get)
#[derive(Debug, Serialize)]
pub struct HistoryQueryRequest {
    /// 0 = row format
    #[serde(rename = "type")]
    pub query_type: i32,
    /// Channel IDs
    pub ids: Vec<i64>,
    /// Granularity: 0=raw, 1=minute, 2=hour, 3=day
    pub g: i32,
    /// Begin time (ms timestamp)
    pub begin: i64,
    /// End time (ms timestamp)
    pub end: i64,
    /// Time range type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<i32>,
    /// Max records (max 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Timezone string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
    /// Fill mode: 0=none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<i32>,
}

/// History data row (value array with timestamp)
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct HistoryRow {
    #[tabled(rename = "Time", display = "display_i64")]
    #[serde(rename = "t")]
    pub timestamp: i64,
    #[tabled(rename = "Values", display = "display_values")]
    #[serde(rename = "c")]
    pub values: Vec<serde_json::Value>,
}

/// History query response
#[derive(Debug, Deserialize)]
pub struct HistoryQueryResponse {
    #[serde(default)]
    pub rows: Vec<HistoryRow>,
}

/// History channel definition
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct HistoryChannel {
    #[tabled(rename = "UID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub uid: Option<i64>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Addr", display = "display_option")]
    #[serde(rename = "addrDesc", default)]
    pub addr_desc: Option<String>,
    #[tabled(rename = "Device", display = "display_option")]
    #[serde(rename = "devAlias", default)]
    pub dev_alias: Option<String>,
    #[tabled(rename = "DataType", display = "display_option")]
    #[serde(rename = "dataType", default)]
    pub data_type: Option<i32>,
    #[tabled(rename = "RegName", display = "display_option")]
    #[serde(rename = "regName", default)]
    pub reg_name: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub sort: Option<i32>,
}

/// History record item (API: GET /api/v2/box/{boxId}/hdataitems)
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct HistoryItem {
    #[tabled(rename = "UID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub uid: Option<i64>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Period(s)", display = "display_option")]
    #[serde(default)]
    pub period: Option<i32>,
    #[tabled(rename = "SampleType", display = "display_option")]
    #[serde(rename = "sampleType", default)]
    pub sample_type: Option<i32>,
    #[tabled(rename = "Channels", display = "display_channels")]
    #[serde(default)]
    pub channels: Vec<HistoryChannel>,
    #[tabled(skip)]
    #[serde(rename = "boxId", default, with = "option_string_i64")]
    pub box_id: Option<i64>,
    #[tabled(skip)]
    #[serde(rename = "hasCtrl", default)]
    pub has_ctrl: bool,
    #[tabled(skip)]
    #[serde(rename = "hasError", default)]
    pub has_error: bool,
    #[tabled(skip)]
    #[serde(default)]
    pub tstate: Option<i32>,
    #[tabled(skip)]
    #[serde(default)]
    pub sort: Option<i32>,
}
