use serde::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::enums::{BoxConnectionState, BoxType, NetworkType};
use crate::models::serde_helper::{string_i64, option_string_i64};

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

fn display_bool(v: &bool) -> String {
    if *v { "Y" } else { "N" }.to_string()
}

/// Box group info (API: GET /api/client/v2/box/grouped/without/box)
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct BoxGroup {
    #[tabled(rename = "ID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub id: Option<i64>,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Node", display = "display_option")]
    #[serde(default)]
    pub node: Option<String>,
    #[tabled(rename = "ParentID", display = "display_option")]
    #[serde(default, with = "option_string_i64")]
    pub pid: Option<i64>,
}

/// Box group with nested box list (API: GET /api/client/v2/box/grouped)
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxGroupList {
    #[serde(default, with = "option_string_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub node: Option<String>,
    #[serde(default, rename = "boxRegs")]
    pub box_regs: Vec<BoxReg>,
}

/// Box registration info (from grouped list)
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct BoxReg {
    #[tabled(rename = "ID")]
    #[serde(with = "string_i64")]
    pub id: i64,
    #[tabled(rename = "Alias", display = "display_option")]
    #[serde(default)]
    pub alias: Option<String>,
    #[tabled(rename = "BoxNo", display = "display_option")]
    #[serde(rename = "boxNo", default)]
    pub box_no: Option<String>,
    #[tabled(rename = "Group", display = "display_option")]
    #[serde(skip)]
    pub group_name: Option<String>,
    #[tabled(rename = "Type", display = "display_option")]
    #[serde(rename = "boxType", default)]
    pub box_type: Option<BoxType>,
    #[tabled(rename = "State", display = "display_option")]
    #[serde(rename = "connState", default)]
    pub conn_state: Option<BoxConnectionState>,
    #[tabled(rename = "Net", display = "display_option")]
    #[serde(default)]
    pub net: Option<NetworkType>,
    #[tabled(skip)]
    #[serde(default)]
    pub carrier: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub boxnet: Option<i32>,
    #[tabled(rename = "Alarms")]
    #[serde(rename = "alarmCount", default)]
    pub alarm_count: i32,
    #[tabled(skip)]
    #[serde(default)]
    pub disabled: bool,
    #[tabled(skip)]
    #[serde(default)]
    pub mode: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "powerLost", default)]
    pub power_lost: bool,
    #[tabled(skip)]
    #[serde(default)]
    pub favorite: bool,
    #[tabled(rename = "Shared", display = "display_bool")]
    #[serde(default)]
    pub shared: bool,
    #[tabled(skip)]
    #[serde(default)]
    pub given: bool,
    #[tabled(skip)]
    #[serde(default)]
    pub owned: bool,
    #[tabled(skip)]
    #[serde(rename = "isCellular", default)]
    pub is_cellular: bool,
    #[tabled(skip)]
    #[serde(rename = "has_expired", default)]
    pub has_expired: bool,
}

/// Box detail info (API: GET /api/v2/box/{boxId}/info)
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxInfo {
    #[serde(rename = "boxId", default, with = "option_string_i64")]
    pub box_id: Option<i64>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub dns: Option<String>,
    #[serde(rename = "refreshTime", default)]
    pub refresh_time: Option<i32>,
    #[serde(rename = "firmwareVersion", default)]
    pub firmware_version: Option<String>,
    #[serde(rename = "subnetMask", default)]
    pub subnet_mask: Option<String>,
    #[serde(default)]
    pub gateway: Option<String>,
    #[serde(rename = "primaryDns", default)]
    pub primary_dns: Option<String>,
    #[serde(rename = "secondaryDns", default)]
    pub secondary_dns: Option<String>,
}

/// Add box request (API: POST /api/client/box/reg)
#[derive(Debug, Serialize)]
pub struct AddBoxRequest {
    pub alias: String,
    #[serde(rename = "boxNo")]
    pub box_no: String,
    #[serde(rename = "boxPassword")]
    pub box_password: String,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}
