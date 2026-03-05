use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub shape: Vec<Anchor>,
    pub holes: Vec<Vec<Anchor>>,
    pub shape_y: f32,
    pub depth_test: bool,
    pub line_width: f32,
    pub line_color: Color,
    pub fill_color: Color,
    pub detail: String,
    pub new_tab: bool,
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
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkerType {
    Shape,
    // room to grow if new types appear
}

impl MarkerConfig {
    pub async fn get(location: &str, world: &str) -> Result<MarkerConfig> {
        let url = format!(
            "{}/maps/{}/live/markers.json",
            location.trim_end_matches('/'),
            world
        );

        let res = reqwest::get(url.as_str()).await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let groups: MarkerConfig = serde_json::from_str(&body)?;
            return Ok(groups)
        };
        Err(anyhow::anyhow!("Failed to request {} with error code {}", url, res.status().as_u16()))
    }
}
