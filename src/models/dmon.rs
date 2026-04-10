use crate::models::enums::{DMonStatus, DataType, EncodeType, PrivilegeType};
use crate::models::serde_helper::{option_string_i64, string_i64};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

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

/// Dmon group with items (API: GET /api/v2/box/{boxId}/dmon/grouped)
#[derive(Debug, Serialize, Deserialize)]
pub struct DmonGroupList {
    #[serde(default, with = "option_string_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub items: Vec<DmonItem>,
}

/// Dmon item definition
/// Ref: FBoxClientDriver.Contract/Entity/V2/DmonItemDtoV2.cs + DMonDtoV2.cs
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct DmonItem {
    #[tabled(rename = "ID")]
    #[serde(with = "string_i64")]
    pub id: i64,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(skip)]
    #[serde(rename = "grpId", default, with = "option_string_i64")]
    pub grp_id: Option<i64>,
    #[tabled(rename = "Group", display = "display_option")]
    #[serde(rename = "grpName", default)]
    pub grp_name: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "devAlias", default)]
    pub dev_alias: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub station: Option<i32>,
    #[tabled(rename = "DataType", display = "display_option")]
    #[serde(rename = "dataType", default)]
    pub data_type: Option<DataType>,
    #[tabled(rename = "Privilege", display = "display_option")]
    #[serde(default)]
    pub privilege: Option<PrivilegeType>,
    #[tabled(rename = "Unit", display = "display_option")]
    #[serde(default)]
    pub unit: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub memo: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "intDigits", default)]
    pub int_digits: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "fracDigits", default)]
    pub frac_digits: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "trafficSaving", default)]
    pub traffic_saving: Option<bool>,
    #[tabled(skip)]
    #[serde(rename = "deadValue", default)]
    pub dead_value: Option<f32>,
    #[tabled(skip)]
    #[serde(default)]
    pub encoding: Option<EncodeType>,
    #[tabled(skip)]
    #[serde(rename = "charCount", default)]
    pub char_count: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "regId", default)]
    pub reg_id: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "regName", default)]
    pub reg_name: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "addr", default)]
    pub addr: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "subAddr", default)]
    pub sub_addr: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "tagName", default)]
    pub tag_name: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub sort: Option<i32>,
}

/// Get dmon value request (API: POST /api/v2/box/{boxId}/dmon/value/get)
/// Ref: FBoxClientDriver.Contract/BoxArgs/GetDMonValueArgs.cs
#[derive(Debug, Serialize)]
pub struct GetDmonValueRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupnames: Option<Vec<String>>,
    #[serde(rename = "timeOut", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

/// Dmon realtime value
/// Ref: FBoxClientDriver.Contract/Entity/DMonEntry.cs
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct DmonValue {
    #[tabled(rename = "ID")]
    #[serde(with = "string_i64")]
    pub id: i64,
    #[tabled(rename = "Name", display = "display_option")]
    #[serde(default)]
    pub name: Option<String>,
    #[tabled(rename = "Value", display = "display_json_option")]
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[tabled(rename = "Status", display = "display_option")]
    #[serde(default)]
    pub status: Option<DMonStatus>,
    #[tabled(skip)]
    #[serde(default)]
    pub timestamp: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "dataType", default)]
    pub data_type: Option<DataType>,
    #[tabled(skip)]
    #[serde(rename = "boxNo", default)]
    pub box_no: Option<String>,
    #[tabled(skip)]
    #[serde(rename = "connState", default)]
    pub conn_state: Option<i32>,
}

/// Set dmon value request (API: POST /api/v2/box{boxId}/dmon/value)
/// Ref: FBoxClientDriver.Contract/BoxArgs/DataMonitorWriteValueArgs.cs
#[derive(Debug, Serialize)]
pub struct SetDmonValueRequest {
    /// Dmon UID (as string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Dmon name (alternative to id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Group name (used with name for disambiguation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    /// Write type: 0=auto, 1=int/decimal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<i32>,
    /// Value to write
    pub value: String,
    /// Flag for write confirmation
    #[serde(default)]
    pub flag: Option<bool>,
}

/// Dmon group info
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct DmonGroup {
    #[tabled(rename = "ID")]
    #[serde(with = "string_i64")]
    pub id: i64,
    #[tabled(rename = "Name")]
    pub name: String,
}
