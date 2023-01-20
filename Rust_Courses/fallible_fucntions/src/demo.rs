#[derive(Debug)]
enum MenuChoice {
	MainMenu,
	Starrt,
	Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
	match input {
		"mainmenu" => Ok(MenuChoice::MainMenu),
		"start" => Ok(MenuChoice::Starrt),
		"quit" => Ok(MenuChoice::Quit),
		_ => Err("menu choice not found.".to_owned())
	}
}
fn pick_choice(input: &str) -> Result<(), String> {
	let choice = get_choice(input)?;
	println!("choice: {:?}", choice);
	Ok(())
}
fn main () {
	pick_choice("start");
}