use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldSetting {
    pub name: String,
    pub sorting: i32,

    pub hires: HiresConfig,
    pub lowres: LowresConfig,

    pub start_pos: [i32; 2],
    pub sky_color: [f32; 4],
    pub void_color: [f32; 4],

    pub ambient_light: f32,
    pub sky_light: f32,

    pub perspective_view: bool,
    pub flat_view: bool,
    pub free_flight_view: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HiresConfig {
    pub tile_size: [u32; 2],
    pub scale: [f32; 2],
    pub translate: [i32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowresConfig {
    pub tile_size: [u32; 2],
    pub lod_factor: u32,
    pub lod_count: u32,
}
impl WorldSetting {
    pub async fn get(location: &str, world: &str) -> Result<WorldSetting> {
        let url = format!("{}/maps/{}/settings.json", location.trim_end_matches('/'), world);
        
        let response = loop {
            let res = reqwest::get(url.as_str()).await?;
            if res.status().is_success() {
                let body = res.text().await?;
                break body;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        };

        let groups: WorldSetting = serde_json::from_str(&response)?;
        Ok(groups)
    }
}