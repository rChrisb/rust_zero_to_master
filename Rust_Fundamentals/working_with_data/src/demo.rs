#[allow(dead_code)]
enum Direction {
	Left,
	Right,
	Down,
}

fn main () {
	let go = Direction::Right;
	// let mut go = Direction::Left;
	// let mut go = Direction::Right;
	// let gone = Direction::Right;
	// let going = Direction::Left;

	match go {
		Direction::Left => println!("Go to the left"),
		Direction::Right => println!("Go to the Right"),
		Direction::Down => println!("Go Down"),
	}
}