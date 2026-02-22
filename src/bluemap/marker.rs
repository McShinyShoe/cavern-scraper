use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerConfig {
    #[serde(flatten)]
    pub groups: HashMap<String, MarkerGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkerGroup {
    pub label: String,
    pub toggleable: bool,
    pub default_hidden: bool,
    pub sorting: i32,

    pub markers: HashMap<String, Marker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub classes: Vec<String>,
    pub detail: String,
    pub icon: String,
    pub anchor: Anchor,

    pub min_distance: f32,
    pub max_distance: f32,

    #[serde(rename = "type")]
    pub marker_type: MarkerType,

    pub label: String,
    pub position: Position,

    pub sorting: i32,
    pub listed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkerType {
    Poi,
    // room to grow if new types appear
}

impl MarkerConfig {
    pub async fn get(location: &str, world: &str) -> Result<MarkerConfig> {
        let url = format!("{}/maps/{}/live/markers.json", location.trim_end_matches('/'), world);
        
        let response = loop {
            let res = reqwest::get(url.as_str()).await?;
            if res.status().is_success() {
                let body = res.text().await?;
                break body;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        };

        let groups: MarkerConfig = serde_json::from_str(&response)?;
        Ok(groups)
    }
}