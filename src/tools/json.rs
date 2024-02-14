use std::time::Duration;

use serde::{Deserialize, Deserializer};
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};

pub fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let duration_str: String = Deserialize::deserialize(deserializer)?;
    humantime::parse_duration(&duration_str).map_err(serde::de::Error::custom)
}

// Custom deserializer for SQLite timestamp
pub fn deserialize_sqlite_timestamp<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let raw_date = PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)?;

    Ok(raw_date.assume_utc())
}
