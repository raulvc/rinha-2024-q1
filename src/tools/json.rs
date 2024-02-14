use std::time::Duration;

use serde::{Deserialize, Deserializer};

pub fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let duration_str: String = Deserialize::deserialize(deserializer)?;
    humantime::parse_duration(&duration_str).map_err(serde::de::Error::custom)
}
