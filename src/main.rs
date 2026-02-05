#![allow(dead_code)]

mod dynmap;
use dynmap::players::get_online_players;
use dynmap::settings::get_setting;

mod cavern;
use cavern::towny_info::get_towny_info;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let data = get_online_players().await?;
    println!("{:#?}", &data);

    let data = get_setting().await?;
    println!("{:#?}", &data);
    
    let data= get_towny_info().await?;
    println!("{:#?}", &data);

    Ok(())
}
