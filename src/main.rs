use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;

fn createpath(path: &str) -> String {
    let mut url = String::from("https://api.spacetraders.io/v2/");
    url.push_str(path);
    url
}

fn apikey() -> String {
    let key: String = env::var("SPACETRADERSKEY").unwrap_or("".to_string());
    if key.is_empty() {
        panic!("The SPACETRADERSKEY environment variable is not set. Please follow the instructions at https://docs.spacetraders.io/quickstart/new-game to create a new Agent, and put your key in the aforementioned environment variable.")
    }
    key
}

#[derive(Serialize, Deserialize, Debug)]
struct HasAgentData {
    data: Agent,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Agent {
    accountId: String,
    symbol: String,
    headquarters: String,
    credits: f64,
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nYour Agent\n\nID: {}\nSymbol: {}\nHeadquarters: {}\nCredits: {}",
            self.accountId, self.symbol, self.headquarters, self.credits
        )
    }
}

fn myagent() -> Agent {
    let res: HasAgentData = reqwest::blocking::Client::new()
        .get(createpath("my/agent"))
        .header(AUTHORIZATION, format!("Bearer {}", apikey()))
        .send()
        .unwrap()
        .json()
        .unwrap();

    Agent {
        accountId: res.data.accountId,
        symbol: res.data.symbol,
        headquarters: res.data.headquarters,
        credits: res.data.credits,
    }
}

/////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Waypoint {
    symbol: String,
    r#type: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct System {
    symbol: String,
    sectorSymbol: String,
    r#type: String,
    x: i32,
    y: i32,
    waypoints: Vec<Waypoint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct hasSystemData {
    data: System,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct SystemsList {
    data: Vec<System>,
}

fn listsystems() -> SystemsList {
    let res: SystemsList = reqwest::blocking::Client::new()
        .get(createpath("systems"))
        .header(AUTHORIZATION, format!("Bearer {}", apikey()))
        .send()
        .unwrap()
        .json()
        .unwrap();

    res
}

fn main() {
    clearscreen::clear().unwrap();

    let choice: String = inquire::Select::new(
        "What category would you like to query?",
        vec!["Agent".to_string(), "Systems".to_string()],
    )
    .prompt()
    .unwrap();

    if choice == *String::from("Agent") {
        agentmenu();
    } else if choice == *String::from("Systems") {
        systemsmenu()
    }
}

fn agentmenu() {
    let choice: String = inquire::Select::new(
        "Which endpoint do you want to use?",
        vec!["Agent information".to_string()],
    )
    .prompt()
    .unwrap();

    if choice == *String::from("Agent information") {
        println!("{}", myagent());
    }
}

fn systemsmenu() {
    let choice: String = inquire::Select::new(
        "Which endpoint do you want to use?",
        vec!["List systems".to_string()],
    )
    .prompt()
    .unwrap();

    if choice == *String::from("List systems") {
        println!("{:#?}", listsystems());
    }
}
