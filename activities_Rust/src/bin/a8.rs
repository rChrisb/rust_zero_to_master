// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// #[derive(Debug)]
// enum Flavors {
// 	Chocolate,
// 	Apple,
// 	Orange
// }
// struct Drinks {
// 	flavor: Flavors,
// 	ounce: f64
// }
// fn main() {
// 	let my_drink = Drinks {
// 		flavor: Flavors::Apple,
// 		ounce: 4.2
// 	};

// 	match my_drink.flavor {
// 		Flavors::Chocolate => println!("Chocolate drink"),
// 		Flavors::Apple => println!("Apple juice"),
// 		Flavors::Orange => println!("Orange juice"),
// 	}
// 	print_drink(my_drink);

// }
// fn print_drink(drink: Drinks){
// 	println!("the flavor is {:?}", drink.flavor );
// 	println!("the ounce is {}", drink.ounce);
// }
#[derive(Debug)]
enum Name {
	Messi,
	Maradona
}
struct Player {
	name: Name,
	ballon_dors: i32
}

fn print_player(player:Player) {
	println!("{:?} has {:?} ballon d'ors", player.name, player.ballon_dors)
}
fn main() {
	let the_best = Player {
		name: Name::Messi,
		ballon_dors: 8
	};
	match the_best.name {
		Name::Messi => println!("Messi is the best"),
		Name::Maradona => println!("Maradona is the best")
	}
	print_player(the_best);
}