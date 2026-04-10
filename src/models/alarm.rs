use serde::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::serde_helper::option_string_i64;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

fn display_json_option(o: &Option<serde_json::Value>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

fn display_alarm_group(o: &Option<AlarmGroupRef>) -> String {
    match o {
        Some(g) => g.name.clone().unwrap_or_else(|| "-".to_string()),
        None => "-".to_string(),
    }
}

fn display_bool(v: &bool) -> String {
    if *v { "Y" } else { "N" }.to_string()
}

/// Alarm group reference (nested in alarm item)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmGroupRef {
    #[serde(default, with = "option_string_i64")]
    pub uid: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}

/// Alarm group with contacts
#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmGroupInfo {
    #[serde(default, with = "option_string_i64")]
    pub uid: Option<i64>,
    pub name: Option<String>,
    pub memo: Option<String>,
    #[serde(default)]
    pub contacts: Vec<serde_json::Value>,
}

/// Alarm list response (paginated)
#[derive(Debug, Deserialize)]
pub struct AlarmListResponse {
    #[serde(rename = "totalCount", default)]
    pub total_count: Option<String>,
    #[serde(default)]
    pub items: Vec<AlarmItem>,
}

/// Alarm item definition (API: POST /api/v4/box/{boxId}/alarm)
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct AlarmItem {
    #[tabled(rename = "ID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub id: Option<i64>,
    #[tabled(rename = "Code", display = "display_option")]
    #[serde(default)]
    pub code: Option<i32>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Message", display = "display_option")]
    #[serde(rename = "alarmMsg", default)]
    pub alarm_msg: Option<String>,
    #[tabled(rename = "Group", display = "display_alarm_group")]
    #[serde(default)]
    pub group: Option<AlarmGroupRef>,
    #[tabled(rename = "Disabled", display = "display_bool")]
    #[serde(default)]
    pub disabled: bool,
    #[tabled(rename = "Delay", display = "display_option")]
    #[serde(rename = "delayTime", default)]
    pub delay_time: Option<i32>,
    #[tabled(rename = "Addr", display = "display_option")]
    #[serde(rename = "addrDesc", default)]
    pub addr_desc: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub condition1: Option<i32>,
    #[tabled(skip)]
    #[serde(default)]
    pub operand1: Option<f64>,
    #[tabled(skip)]
    #[serde(default)]
    pub condition2: Option<i32>,
    #[tabled(skip)]
    #[serde(default)]
    pub operand2: Option<f64>,
    #[tabled(skip)]
    #[serde(rename = "condMethod", default)]
    pub cond_method: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "executeOnEdge", default)]
    pub execute_on_edge: bool,
    #[tabled(skip)]
    #[serde(rename = "floatPrecision", default)]
    pub float_precision: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "deadValue", default)]
    pub dead_value: Option<f64>,
    #[tabled(skip)]
    #[serde(rename = "devAlias", default)]
    pub dev_alias: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "regName", default)]
    pub reg_name: Option<String>,
}

/// Alarm history event
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct AlarmHistory {
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(rename = "rn", default)]
    pub name: Option<String>,
    #[tabled(rename = "Code", display = "display_option")]
    #[serde(rename = "n", default)]
    pub code: Option<String>,
    #[tabled(rename = "Message", display = "display_option")]
    #[serde(rename = "m", default)]
    pub msg: Option<String>,
    #[tabled(rename = "Value", display = "display_json_option")]
    #[serde(rename = "v", default)]
    pub value: Option<serde_json::Value>,
    #[tabled(rename = "Action", display = "display_option")]
    #[serde(rename = "a", default)]
    pub action: Option<i32>,
    #[tabled(rename = "Time", display = "display_option")]
    #[serde(rename = "t", default)]
    pub t: Option<i64>,
    #[tabled(skip)]
    #[serde(rename = "i", default, with = "option_string_i64")]
    pub alarm_id: Option<i64>,
    #[tabled(rename = "Confirmed", display = "display_option")]
    #[serde(rename = "confirmState", default)]
    pub confirm_state: Option<i32>,
}

/// Add alarm group request
#[derive(Debug, Serialize)]
pub struct AddAlarmGroupRequest {
    #[serde(rename = "boxId")]
    pub box_id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cuids: Option<Vec<i64>>,
}
