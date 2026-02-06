#![allow(dead_code)]

use anyhow::{Result, anyhow, bail};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeyserProfile {
    #[serde(deserialize_with = "u64_to_hex_string")]
    pub xuid: String,
}

#[derive(Debug, Deserialize)]
pub struct GeyserError {
    pub message: String,
}

use serde::de::{Deserializer};

fn u64_to_hex_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;
    Ok(format!("{:032x}", value))
}

pub async fn get_geyser_profile(name: &str) -> Result<GeyserProfile> {
    let url = format!("https://api.geysermc.org/v2/xbox/xuid/{}", name);

    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    println!("{}", body);

    if let Ok(profile) = serde_json::from_str::<GeyserProfile>(&body) {
        Ok(profile)
    } else if let Ok(err) = serde_json::from_str::<GeyserError>(&body) {
        bail!(err.message)
    } else {
        bail!("Unknown response format".to_string())
    }
}