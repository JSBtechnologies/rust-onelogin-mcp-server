// Allow dead code - serde helpers available for future use
#![allow(dead_code)]

use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

/// Deserialize an Option<String> that flexibly handles the OneLogin API returning
/// integers, booleans, or other non-string types where a string is expected.
/// For example, some fields return `0` instead of `null` or a string value.
pub fn flexible_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<Value>::deserialize(deserializer)?;
    Ok(v.and_then(|v| match v {
        Value::Null => None,
        Value::String(s) if s.is_empty() => None,
        Value::String(s) => Some(s),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        other => Some(other.to_string()),
    }))
}
