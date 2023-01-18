fn main()
{
	let is_major = true;
	let age = 11;
	let name = "Lelouch Lamperouge";
	// if age <= 15 {
	// 	println!("You are very young!")
	// } else{
	// 	println!("You are above 16 years old")
	// }
	match is_major {
		true => println!("You are major!"),
		_ => println!("You are under age!"),
		// 16 => println!("You are 16!"),
		// 17 => println!("You are 17!"), 
	}
	match age {
		/* true => println!("You are major!"),
		false => println!("You are under age!"), */
		16 => println!("You are 16!"),
		17 => println!("You are 17!"), 
		18 => println!("You are 18"),
		_ => println!("You are {}", age)
	}
	match name {
		"CHRIS" => println!("Hello {}", name),
		"BIKOY" => println!("Heyy BIKOY"),
		"RUBIE" => println!("Hi RUBIE"),
		_ => println!("Goodmorning {}", name),
	}
}