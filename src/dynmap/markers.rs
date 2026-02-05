use serde::Deserialize;
use reqwest::Error;

const MARKERS_LINK: &str = "https://map.thecavern.net/tiles/minecraft_overworld/markers.json";

#[derive(Debug, Deserialize)]
pub struct MarkerGroup {
    pub hide: bool,
    pub z_index: i32,
    pub name: String,
    pub control: bool,
    pub id: String,
    pub markers: Vec<Marker>,
    pub order: i32,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Marker {
    #[serde(rename = "icon")]
    Icon(IconMarker),
    #[serde(rename = "polygon")]
    Polygon(PolygonMarker),
    #[serde(rename = "polyline")]
    Polyline(PolylineMarker),
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
pub struct IconMarker {
    pub tooltip_anchor: Option<Point>,
    pub popup: Option<String>,
    pub size: Option<Point>,
    pub anchor: Option<Point>,
    pub tooltip: Option<String>,
    pub icon: String,
    pub point: Point,
}

#[derive(Debug, Deserialize)]
pub struct PolygonMarker {
    pub popup: Option<String>,
    pub tooltip: Option<String>,
    #[serde(rename = "fillColor")]
    pub fill_color: Option<String>,
    pub color: Option<String>,
    pub points: Vec<Vec<Vec<Point>>>,
}

#[derive(Debug, Deserialize)]
pub struct PolylineMarker {
    pub tooltip: Option<String>,
    pub color: Option<String>,
    pub points: Vec<Point>,
}

pub async fn get_marker_groups() -> Result<Vec<MarkerGroup>, Error> {
    let url: &str = MARKERS_LINK;
    
    let response = loop {
        let res = reqwest::get(url).await?;
        if res.status().is_success() {
            let body = res.text().await?;
            break body;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    let groups: Vec<MarkerGroup> = serde_json::from_str(&response).unwrap();
    Ok(groups)
}
