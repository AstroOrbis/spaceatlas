#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]

pub mod routes;
pub mod tools;

use routes::agent::*;
use routes::system::*;
use tools::apikey::apikey;


fn main() {
	apikey();
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
