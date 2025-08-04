use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use reqwest::Error;

use crate::dynmap::markers::{Marker, Point, get_marker_groups};

#[derive(Debug)]
pub struct OnlinePlayer {
    armor: i8,
    health: i8,
    location: Location,
    name: String,
    uuid: String,
    yaw: i8,
}

#[derive(Debug, Default)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
    world: World,
}

#[derive(Debug)]
pub enum World {
    Mainworld, // minecraft_overworld
    Nether,    // minecraft_the_nether
    Spawn,     // minecraft_spawn
    Resource,  // minecraft_resource
    End,       // minecraft_the_end
    Dungeon,   // minecraft_dungeon
    Unknown,
}
impl Default for World {
    fn default() -> Self {
        World::Unknown
    }
}

fn round_to_nearest_16(value: f64) -> f64 {
    (value / 16.0).round() * 16.0
}

pub fn ring_area(ring: &[Point]) -> f64 {
    let n = ring.len();
    if n < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;

        let x1 = round_to_nearest_16(ring[i].x);
        let z1 = round_to_nearest_16(ring[i].z);
        let x2 = round_to_nearest_16(ring[j].x);
        let z2 = round_to_nearest_16(ring[j].z);

        area += x1 * z2;
        area -= x2 * z1;
    }

    area.abs() * 0.5 / 256.0
}

fn polygon_area(polygon: &Vec<Vec<Point>>) -> i32 {
    if polygon.is_empty() {
        return 0;
    }

    let outer_area = ring_area(&polygon[0]);
    let holes_area: f64 = polygon.iter().skip(1).map(|ring| ring_area(ring)).sum();

    (outer_area - holes_area) as i32
}

fn multipolygon_area(multipolygon: &[Vec<Vec<Point>>]) -> i32 {
    multipolygon.iter().map(|poly| polygon_area(&poly)).sum()
}

#[derive(Debug, Default)]
pub struct TownInfo {
    name: String,
    nation: String,
    mayor: String,
    pvp: bool,
    assistants: Vec<String>,
    residents: Vec<String>,
    spawn: Location,
    is_capital: bool,
    size: i32,
}

#[derive(Debug)]
pub struct PopupInfo {
    name: String,
    nation: String,
    mayor: String,
    pvp: bool,
    assistants: Vec<String>,
    residents: Vec<String>,
}

impl From<PopupInfo> for TownInfo {
    fn from(popup: PopupInfo) -> Self {
        TownInfo {
            name: popup.name,
            nation: popup.nation,
            mayor: popup.mayor,
            pvp: popup.pvp,
            assistants: popup.assistants,
            residents: popup.residents,
            spawn: Location {
                x: 0.0,
                y: 64.0,
                z: 0.0,
                world: World::Unknown,
            },
            is_capital: false,
            size: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct NationInfo {
    name: String,
    capital: String,
    towns: Vec<String>,
}

#[derive(Debug)]
pub struct TownyInfo {
    town_infos: HashMap<String, TownInfo>,
    nation_infos: HashMap<String, NationInfo>,
}

fn unescape_html_unicode(s: &str) -> String {
    s.replace(r"\u003c", "<")
        .replace(r"\u003e", ">")
        .replace(r"\u003d", "=")
        .replace(r"\u0026", "&")
        .replace(r"\u0027", "'")
        .replace(r"\u0022", "\"")
}

fn parse_dynmap_popup(tooltip: &str) -> Option<PopupInfo> {
    let decoded = unescape_html_unicode(tooltip);

    let re_town =
        Regex::new(r#"<span[^>]*font-size[^>]*>\s*([^\(<]+?)(?:\s*\(([^)]+)\))?\s*</span>"#)
            .ok()?;
    let caps = re_town.captures(&decoded)?;
    let name = caps.get(1)?.as_str().trim().to_string();
    let nation = caps
        .get(2)
        .map_or("".to_string(), |m| m.as_str().trim().to_string());

    let re_mayor = Regex::new(r"Mayor:\s*<span[^>]*>\s*([^<]+)\s*</span>").ok()?;
    let mayor = re_mayor
        .captures(&decoded)?
        .get(1)?
        .as_str()
        .trim()
        .to_string();

    let re_pvp = Regex::new(r"PVP:\s*<span[^>]*>\s*(true|false)\s*</span>").ok()?;
    let pvp_str = re_pvp.captures(&decoded)?.get(1)?.as_str().trim();
    let pvp = pvp_str.eq_ignore_ascii_case("true");

    let re_assistants = Regex::new(r"Assistants:\s*<span[^>]*>\s*([^<]+)\s*</span>").ok()?;
    let assistants_raw = re_assistants.captures(&decoded)?.get(1)?.as_str().trim();
    let assistants = if assistants_raw.eq_ignore_ascii_case("none") {
        vec![]
    } else {
        assistants_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    let re_residents = Regex::new(r"Residents:\s*</bold>\s*<span>\s*([^<]+)\s*</span>").ok()?;
    let residents_raw = re_residents.captures(&decoded)?.get(1)?.as_str().trim();
    let residents = residents_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Some(PopupInfo {
        name,
        nation,
        mayor,
        pvp,
        assistants,
        residents,
    })
}

fn parse_dynmap_tooltip(tooltip: &str) -> Option<(String, String)> {
    let decoded = unescape_html_unicode(tooltip);
    let re: Regex = Regex::new(r"(?i)<bold>\s*([^\(<]+?)(?:\s*\(([^)]+)\))?\s*</bold>").unwrap();

    if let Some(caps) = re.captures(&decoded) {
        let town = caps.get(1)?.as_str().trim().to_string();
        let nation = caps
            .get(2)
            .map_or("".to_string(), |m| m.as_str().trim().to_string());
        Some((town, nation))
    } else {
        None
    }
}

pub async fn get_towny_info() -> Result<TownyInfo, Error> {
    let mut towny_info: TownyInfo = TownyInfo {
        town_infos: HashMap::new(),
        nation_infos: HashMap::new(),
    };
    match get_marker_groups().await {
        Ok(groups) => {
            for group in groups {
                for marker in group.markers {
                    match marker {
                        Marker::Icon(m) => {
                            let is_capital: i8;
                            match m.icon.as_str() {
                                "towny_capital_icon" => {
                                    is_capital = 1;
                                }
                                "towny_town_icon" => {
                                    is_capital = 0;
                                }
                                _ => {
                                    is_capital = -1;
                                }
                            }
                            if is_capital != -1 {
                                if let Some(tooltip_text) = m.tooltip {
                                    if let Some((town_name, nation_name)) =
                                        parse_dynmap_tooltip(&tooltip_text)
                                    {
                                        match towny_info.town_infos.entry(town_name.clone()) {
                                            Entry::Occupied(mut entry) => {
                                                let town = entry.get_mut();
                                                town.spawn = Location {
                                                    x: m.point.x,
                                                    y: 64.0,
                                                    z: m.point.z,
                                                    world: World::Mainworld,
                                                };
                                                town.is_capital = is_capital == 1;
                                            }
                                            Entry::Vacant(entry) => {
                                                entry.insert(TownInfo {
                                                    spawn: Location {
                                                        x: m.point.x,
                                                        y: 0.0,
                                                        z: m.point.z,
                                                        world: World::Mainworld,
                                                    },
                                                    is_capital: is_capital == 1,
                                                    ..Default::default()
                                                });
                                            }
                                        }

                                        if !nation_name.is_empty() {
                                            match towny_info.nation_infos.entry(nation_name.clone())
                                            {
                                                Entry::Occupied(mut entry) => {
                                                    let nation = entry.get_mut();
                                                    if is_capital == 1 {
                                                        nation.name = nation_name.clone();
                                                        nation.capital = nation_name.clone();
                                                    }
                                                    nation.towns.push(town_name);
                                                }
                                                Entry::Vacant(entry) => {
                                                    entry.insert(NationInfo {
                                                        name: {
                                                            if is_capital == 1 {
                                                                nation_name.clone()
                                                            } else {
                                                                String::from("")
                                                            }
                                                        },
                                                        capital: {
                                                            if is_capital == 1 {
                                                                town_name.clone()
                                                            } else {
                                                                String::from("")
                                                            }
                                                        },
                                                        towns: {
                                                            if is_capital == 1 {
                                                                vec![town_name.clone()]
                                                            } else {
                                                                vec![]
                                                            }
                                                        },
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Marker::Polygon(m) => {
                            if let Some(popup_text) = &m.popup {
                                if let Some(popup_info) = parse_dynmap_popup(&popup_text) {
                                    let town_size = multipolygon_area(&m.points);
                                    match towny_info.town_infos.entry(popup_info.name.clone()) {
                                        Entry::Occupied(mut entry) => {
                                            let town = entry.get_mut();
                                            town.name = popup_info.name;
                                            town.nation = popup_info.nation;
                                            town.mayor = popup_info.mayor;
                                            town.pvp = popup_info.pvp;
                                            town.assistants = popup_info.assistants;
                                            town.residents = popup_info.residents;
                                            town.size = town_size;
                                        }
                                        Entry::Vacant(entry) => {
                                            entry.insert(TownInfo {
                                                name: popup_info.name,
                                                nation: popup_info.nation,
                                                mayor: popup_info.mayor,
                                                pvp: popup_info.pvp,
                                                assistants: popup_info.assistants,
                                                residents: popup_info.residents,
                                                size: town_size,
                                                ..Default::default()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        Marker::Polyline(_m) => {}
                    }
                }
            }
            Ok(towny_info)
        }
        Err(err) => Err(err),
    }
}
