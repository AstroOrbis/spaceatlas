use crate::tools::apikey::apikey;
use crate::tools::createpath::createpath;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Waypoint {
    symbol: String,
    r#type: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    symbol: String,
    sectorSymbol: String,
    r#type: String,
    x: i32,
    y: i32,
    waypoints: Vec<Waypoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct hasSystemData {
    data: System,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemsList {
    data: Vec<System>,
}

pub fn listsystems() -> SystemsList {
    let res: SystemsList = reqwest::blocking::Client::new()
        .get(createpath("systems"))
        .header(AUTHORIZATION, format!("Bearer {}", apikey()))
        .send()
        .unwrap()
        .json()
        .unwrap();

    res
}

