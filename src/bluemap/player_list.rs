use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerList {
    pub players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub uuid: String,
    pub name: String,
    pub foreign: bool,
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rotation {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl PlayerList {
    pub async fn get(location: &str, world: &str) -> Result<PlayerList> {
        let url = format!(
            "{}/maps/{}/live/players.json",
            location.trim_end_matches('/'),
            world
        );

        let res = reqwest::get(url.as_str()).await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let groups: PlayerList = serde_json::from_str(&body)?;
            return Ok(groups)
        };
        Err(anyhow::anyhow!("Failed to request {} with error code {}", url, res.status().as_u16()))
    }
}
