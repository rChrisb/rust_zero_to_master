// #[allow(dead_code)]
// enum Direction {
// 	Left,
// 	Right,
// 	Down,
// }

struct GroceryItem {
	stock: i32,
	price: f64,

}

fn main () {
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
		let cereal = GroceryItem{
			stock: 10,
			price: 2.99,
		};
		println!("the cereal stock is {0}", cereal.stock);
		println!("the cereal price is {0}", cereal.price)
}