use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSetting {
    pub version: String,
    pub use_cookies: bool,
    pub default_to_flat_view: bool,
    pub resolution_default: f32,
    pub min_zoom_distance: u32,
    pub max_zoom_distance: u32,
    pub hires_slider_max: u32,
    pub hires_slider_default: u32,
    pub hires_slider_min: u32,
    pub lowres_slider_max: u32,
    pub lowres_slider_default: u32,
    pub lowres_slider_min: u32,
    pub map_data_root: String,
    pub live_data_root: String,
    pub maps: Vec<String>,
    pub scripts: Vec<String>,
    pub styles: Vec<String>,
}

impl MapSetting {
    pub async fn get(location: &str) -> Result<MapSetting> {
    let url = format!("{}/settings.json", location.trim_end_matches('/'));
    
    let response = loop {
        let res = reqwest::get(url.as_str()).await?;
        if res.status().is_success() {
            let body = res.text().await?;
            break body;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    let groups: MapSetting = serde_json::from_str(&response)?;
    Ok(groups)
    }
}