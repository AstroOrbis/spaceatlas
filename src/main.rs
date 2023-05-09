use reqwest::header::AUTHORIZATION;
use std::env;
use serde::{Serialize, Deserialize};
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
struct HasData {
    data: Agent 
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
        write!(f, "Agent {{ accountId: {}, symbol: {}, headquarters: {}, credits: {} }}", self.accountId, self.symbol, self.headquarters, self.credits)
    }
}

fn myagent() -> Agent {
    let res: HasData = reqwest::blocking::Client::new()
        .get(createpath("my/agent"))
        .header(AUTHORIZATION, format!("Bearer {}", apikey())).send().unwrap().json().unwrap();
    
    Agent {
        accountId: res.data.accountId,
        symbol: res.data.symbol,
        headquarters: res.data.headquarters,
        credits: res.data.credits
    }
}






fn main() {
    clearscreen::clear().unwrap();
    
    let choice: String = inquire::Select::new("What would you like to query?", vec![
        "Agent information".to_string()
    ]).prompt().unwrap();
    
    if choice == *String::from("Agent information") {
        println!("{}", myagent());
    }
}

