// #[allow(dead_code)]
// enum Direction {
// 	Left,
// 	Right,
// 	Down,
// }

// struct GroceryItem {
// 	stock: i32,
// 	price: f64,

// }
#[allow(dead_code)]
enum AccesLevel {
	Admin,
	User,
	Guest
}
fn main () {
	let user_access = AccesLevel::User;
	let can_user_access_secret_files = match user_access {
		AccesLevel::Admin => true,
		_ => false
	};
	println!("Is it true that a user can access secret files? : {}", can_user_access_secret_files);


	// let favourites = ("football", "messi", "barça", "argentina");
	// let (fav_sport, fav_player, fav_team, fav_country) = favourites;

	// println!("{fav_player}");
	// println!("{fav_country}");
	// println!("{fav_sport}");
	// println!("{fav_team}");

	// println!("{}", favourites.3)
	// let go = Direction::Right;
	// let mut go = Direction::Left;
	// let mut go = Direction::Right;
	// let gone = Direction::Right;
	// let going = Direction::Left;

	// match go {
	// 	Direction::Left => println!("Go to the left"),
	// 	Direction::Right => println!("Go to the Right"),
	// 	Direction::Down => println!("Go Down"),
	// }
		/* let cereal = GroceryItem{
			stock: 10,
			price: 2.99,
		};
		println!("the cereal stock is {0}", cereal.stock);
		println!("the cereal price is {0}", cereal.price) */

}