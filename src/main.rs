#![allow(dead_code)]

use reqwest::Error;

mod dynmap;
use dynmap::players::get_online_players;
use dynmap::settings::get_setting;
use dynmap::markers::get_marker_groups;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let data = get_online_players().await?;
    println!("{:#?}", &data);

    let data = get_setting().await?;
    println!("{:#?}", &data);
    
    let data= get_marker_groups().await?;
    println!("{:#?}", &data);

    Ok(())
}
