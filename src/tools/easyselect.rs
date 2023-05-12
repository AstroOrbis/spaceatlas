pub fn easyselect(prompt: &str , choices: Vec<String>) -> String {
	let choice = inquire::Select::new(
		prompt,
		choices,
	).prompt().unwrap();

	choice
}