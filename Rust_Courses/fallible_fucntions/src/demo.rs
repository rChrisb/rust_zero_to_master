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

fn main () {
	let choice = get_choice("damn");
	match choice {
		Ok(menu) => println!("choice : {:?}", menu),
		Err(menu) => println!("{:?}", menu)
	}
}