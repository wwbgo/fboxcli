use anyhow::{Context, Result};
use crate::api::FBoxClient;
use crate::models::serde_helper::option_string_i64;
use crate::t;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct BoxLocation {
    #[tabled(rename = "BoxID", display = "display_option")]
    #[serde(rename = "boxId", default, with = "option_string_i64")]
    pub box_id: Option<i64>,
    #[tabled(rename = "BoxNo", display = "display_option")]
    #[serde(rename = "boxNo", default)]
    pub box_no: Option<String>,
    #[tabled(skip)]
    #[serde(default)]
    pub longitude: Option<f64>,
    #[tabled(skip)]
    #[serde(default)]
    pub latitude: Option<f64>,
    #[tabled(skip)]
    #[serde(default)]
    pub radius: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "locationFetchType", default)]
    pub location_fetch_type: Option<i32>,
    #[tabled(skip)]
    #[serde(rename = "useLongitude", default)]
    pub use_longitude: Option<f64>,
    #[tabled(skip)]
    #[serde(rename = "useLatitude", default)]
    pub use_latitude: Option<f64>,
    #[tabled(skip)]
    #[serde(rename = "useAddress", default)]
    pub use_address: Option<String>,
    /// Resolved longitude based on locationFetchType
    #[tabled(rename = "Longitude", display = "display_option")]
    #[serde(skip)]
    pub display_longitude: Option<f64>,
    /// Resolved latitude based on locationFetchType
    #[tabled(rename = "Latitude", display = "display_option")]
    #[serde(skip)]
    pub display_latitude: Option<f64>,
    /// Resolved address
    #[tabled(rename = "Address", display = "display_option")]
    #[serde(skip)]
    pub display_address: Option<String>,
}

impl BoxLocation {
    /// Resolve display fields based on locationFetchType
    /// 0 = auto positioning (use longitude/latitude)
    /// 1 = manual positioning (use useLongitude/useLatitude/useAddress)
    fn resolve(&mut self) {
        match self.location_fetch_type.unwrap_or(0) {
            1 => {
                self.display_longitude = self.use_longitude;
                self.display_latitude = self.use_latitude;
                self.display_address = self.use_address.clone();
            }
            _ => {
                self.display_longitude = self.longitude;
                self.display_latitude = self.latitude;
                self.display_address = None;
            }
        }
    }
}

pub async fn get_locations(client: &mut FBoxClient, ids: &[i64]) -> Result<Vec<BoxLocation>> {
    let body = serde_json::json!({ "ids": ids });
    let resp = client.post_json("/api/client/v2/box/location", &body).await
        .context(t!("Failed to get FBox locations", "获取 FBox 位置信息失败"))?;
    let mut locations: Vec<BoxLocation> = resp.json().await
        .context(t!("Failed to parse location response", "解析位置信息响应失败"))?;
    for loc in &mut locations {
        loc.resolve();
    }
    Ok(locations)
}
