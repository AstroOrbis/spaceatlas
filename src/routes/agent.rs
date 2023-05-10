use crate::tools::apikey::apikey;
use crate::tools::createpath::createpath;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
struct HasAgentData {
	data: Agent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
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

pub fn myagent() -> Agent {
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