//! Serde helpers for FBox API compatibility.
//! FBox API returns long IDs as JSON strings to avoid JS precision loss.

use serde::{self, Deserialize, Deserializer, Serializer};

/// Deserialize a value that may be either a JSON number or a JSON string into i64.
/// Serialize as string to match API convention.
pub mod string_i64 {
    use super::*;

    pub fn serialize<S: Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(n) => n
                .as_i64()
                .ok_or_else(|| serde::de::Error::custom("invalid number")),
            serde_json::Value::String(s) => s
                .parse::<i64>()
                .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom("expected number or string")),
        }
    }
}

/// Same as string_i64 but for Option<i64>.
pub mod option_string_i64 {
    use super::*;

    pub fn serialize<S: Serializer>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<i64>, D::Error> {
        let value = Option::<serde_json::Value>::deserialize(deserializer)?;
        match value {
            None | Some(serde_json::Value::Null) => Ok(None),
            Some(serde_json::Value::Number(n)) => Ok(n.as_i64()),
            Some(serde_json::Value::String(s)) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    s.parse::<i64>().map(Some).map_err(serde::de::Error::custom)
                }
            }
            _ => Err(serde::de::Error::custom("expected number, string, or null")),
        }
    }
}
