pub fn apikey() -> String {
	let key: String = std::env::var("SPACETRADERSKEY").unwrap_or("".to_string());
	if key.is_empty() {
		panic!("The SPACETRADERSKEY environment variable is not set. Please follow the instructions at https://docs.spacetraders.io/quickstart/new-game to create a new Agent, and put your key in the aforementioned environment variable.")
	}
	key
}