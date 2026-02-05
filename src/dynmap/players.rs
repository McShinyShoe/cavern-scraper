#![allow(dead_code)]

use anyhow::Result;
use serde::Deserialize;

const PLAYERS_LINK: &str = "https://map.thecavern.net/tiles/players.json";

#[derive(Debug, Deserialize)]
pub struct OnlinePlayers {
    pub max: u32,
    pub players: Vec<Player>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub name: String,
    pub uuid: String,
    pub world: String,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub yaw: i32,
    pub armor: u8,
    pub health: u8,
}

pub async fn get_online_players() -> Result<OnlinePlayers> {
    let url = PLAYERS_LINK;

    let response = loop {
        let res = reqwest::get(url).await?;
        if res.status().is_success() {
            let body = res.text().await?;
            break body;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    let data: OnlinePlayers = serde_json::from_str(&response).unwrap();    
    Ok(data)
}
