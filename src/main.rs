#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]

pub mod routes;
pub mod tools;

use routes::agent::*;
use routes::system::*;
use tools::apikey::apikey;
use tools::easyselect::easyselect;

fn main() {
	apikey(); // check if the API key is saet


	let menu: Vec<String> = vec![
		"Agent".to_string(), 
		"Systems".to_string()
	];

	let choice: String = easyselect(
		"What category would you like to query?",
		menu.to_owned(),
	);

	if choice == menu[0] {
		agentmenu();
	} else if choice == menu[1] {
		systemsmenu()
	}
}




fn agentmenu() {
	let menu: Vec<String> = vec![
		"Agent information".to_string()
	];

	let choice: String = easyselect(
		"Which endpoint do you want to use?",
		menu.to_owned()
	);

	if choice == menu[0] {
		println!("{}", myagent());
	}
}


fn systemsmenu() {
	let menu: Vec<String> = vec![
		"List systems".to_string(),
		"Get system".to_string(),
		"List waypoints".to_string(),
		"Get waypoint".to_string()
	];

	let choice: String = easyselect(
		"Which endpoint do you want to use?",
		menu.to_owned(),
	);

	if choice == menu[0] {
		println!("{:#?}", listsystems());
	} else if choice == menu[1] {
		println!("{:#?}", getsystem());
	} else if choice == menu[2] {
		println!("{:#?}", listwaypoints());
	} else if choice == menu[3] {
		println!("{:#?}", getwaypoint());
	}
}
