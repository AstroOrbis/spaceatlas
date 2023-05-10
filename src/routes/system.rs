use crate::tools::{apikey::apikey, createpath::createpath};
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

pub fn getsystem() -> System {

	let system = inquire::Text::new("Please enter the system identifier.").prompt().unwrap();

	let res: hasSystemData = reqwest::blocking::Client::new()
		.get(createpath(&format!("systems/{}", system)))
		.send()
		.unwrap()
		.json()
		.unwrap();

	System {
		symbol: res.data.symbol,
		sectorSymbol: res.data.sectorSymbol,
		r#type: res.data.r#type,
		x: res.data.x,
		y: res.data.y,
		waypoints: res.data.waypoints,
	}

}