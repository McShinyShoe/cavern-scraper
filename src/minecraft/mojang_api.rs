#![allow(dead_code)]

use anyhow::{Result, anyhow, bail};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MojangProfile {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct MojangError {
    pub path: String,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

pub async fn get_mojang_profile(name: &str) -> Result<MojangProfile> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);

    let res = reqwest::get(&url).await?;
    let body = res.text().await?;

    if let Ok(profile) = serde_json::from_str::<MojangProfile>(&body) {
        Ok(profile)
    } else if let Ok(err) = serde_json::from_str::<MojangError>(&body) {
        bail!(err.error_message)
    } else {
        bail!("Unknown response format".to_string())
    }
}
