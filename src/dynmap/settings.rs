use serde::Deserialize;
use reqwest::Error;

const SETTING_LINK: &str = "https://map.thecavern.net/tiles/minecraft_overworld/settings.json";

#[derive(Debug, Deserialize)]
pub struct MapSetting {
    pub player_tracker: PlayerTracker,
    pub spawn: Spawn,
    pub marker_update_interval: u32,
    pub zoom: Zoom,
    pub tiles_update_interval: u32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerTracker {
    pub default_hidden: bool,
    pub z_index: u32,
    pub update_interval: u32,
    pub show_controls: bool,
    pub nameplates: Nameplates,
    pub label: String,
    pub priority: u32,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Nameplates {
    pub show_health: bool,
    pub heads_url: String,
    pub show_armor: bool,
    pub show_heads: bool,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Spawn {
    pub x: i64,
    pub z: i64,
}

#[derive(Debug, Deserialize)]
pub struct Zoom {
    #[serde(rename = "def")]
    pub default: u32,
    pub max: u32,
    pub extra: u32,
}

pub async fn get_setting() -> Result<MapSetting, Error> {
    let url = SETTING_LINK;

    let response = loop {
        let res = reqwest::get(url).await?;
        if res.status().is_success() {
            let body = res.text().await?;
            break body;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    let setting: MapSetting = serde_json::from_str(&response).unwrap();
    Ok(setting)
}
