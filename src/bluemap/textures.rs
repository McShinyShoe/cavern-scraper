use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    pub resource_path: String,
    pub color: [f32; 4],
    pub half_transparent: bool,
    pub texture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureList(pub Vec<Texture>);

impl TextureList {
    pub async fn get(location: &str, world: &str) -> Result<TextureList> {
        let url = format!(
            "{}/maps/{}/textures.json",
            location.trim_end_matches('/'),
            world
        );

        let response = loop {
            let res = reqwest::get(url.as_str()).await?;
            if res.status().is_success() {
                let body = res.text().await?;
                break body;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        };

        let groups: TextureList = serde_json::from_str(&response)?;
        Ok(groups)
    }
}
