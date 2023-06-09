use crate::tools::{apikey::apikey, createpath::createpath};
use reqwest::{header::AUTHORIZATION, blocking as req};
use serde::{Deserialize, Serialize};

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
	let res: SystemsList = req::Client::new()
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

	let res: hasSystemData = req::Client::new()
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointTrait {
	symbol: String,
	name: String,
	description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointOrbital {
	symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chart {
	waypointSymbol: Option<String>, // The docs say this is here, but in practice it sometimes isn't
	submittedBy: String,
	submittedOn: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Faction {
	symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Waypoint {
	symbol: String,
	r#type: String,
	systemSymbol: String,
	x: i32,
	y: i32,
	orbitals: Vec<WaypointOrbital>,
	faction: Option<Faction>, // Docs say it's here, it sometimes isn't
	traits: Vec<WaypointTrait>,
	chart: Option<Chart> // Docs say it's here, it sometimes isn't
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
	total: i32,
	page: i32,
	limit: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct hasWaypointData {
	data: Waypoint
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointsList {
	data: Vec<Waypoint>,
	meta: Meta
}

pub fn listwaypoints() -> WaypointsList {

	let system: String = inquire::Text::new("Please enter the system identifier.").prompt().unwrap();

	let res: WaypointsList = req::Client::new()
		.get(createpath(&format!("systems/{}/waypoints", system)))
		.send()
		.unwrap()
		.json()
		.unwrap();

	res

}

pub fn getwaypoint() -> Waypoint {
	let waypoint: String = inquire::Text::new("Please enter the waypoint identifier.").prompt().unwrap();

	let (system, _) = waypoint.rsplit_once('-').unwrap();


	let res: hasWaypointData = req::Client::new()
		.get(createpath(&format!("systems/{}/waypoints/{}", system, waypoint)))
		.send()
		.unwrap()
		.json()
		.unwrap();

	Waypoint {
		symbol: res.data.symbol,
		r#type: res.data.r#type,
		systemSymbol: res.data.systemSymbol,
		x: res.data.x,
		y: res.data.y,
		orbitals: res.data.orbitals,
		faction: res.data.faction,
		traits: res.data.traits,
		chart: res.data.chart
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct hasMarketData {
	data: Market
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
	symbol: String,
	imports: Option<Vec<Product>>,
	exports: Option<Vec<Product>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
	symbol: String,
	name: String,
	description: String
}

pub fn getmarket() -> Market {

	let waypoint: String = inquire::Text::new("Please enter the waypoint identifier.").prompt().unwrap();

	let (system, _) = waypoint.rsplit_once('-').unwrap();

	let res: hasMarketData = req::Client::new()
		.get(createpath(&format!("systems/{}/waypoints/{}/market", system, waypoint)))
		.send()
		.unwrap()
		.json()
		.unwrap();

	Market {
		symbol: res.data.symbol,
		imports: res.data.imports,
		exports: res.data.exports
	}
}