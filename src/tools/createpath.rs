pub fn createpath(path: &str) -> String {
	let mut url = String::from("https://api.spacetraders.io/v2/");
	url.push_str(path);
	url
}