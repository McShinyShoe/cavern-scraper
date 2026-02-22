use serde::{Deserialize, Serialize};
use anyhow::Result;

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
        let url = format!("{}/maps/{}/live/players.json", location.trim_end_matches('/'), world);
        
        let response = loop {
            let res = reqwest::get(url.as_str()).await?;
            if res.status().is_success() {
                let body = res.text().await?;
                break body;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        };

        let groups: PlayerList = serde_json::from_str(&response)?;
        Ok(groups)
    }
}