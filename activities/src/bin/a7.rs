// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// #[allow(dead_code)]
enum Color {
	Red,
	Blue,
}
fn main() {
	let favourite_color = Color::Red;

	my_fav_color(favourite_color);
}
fn my_fav_color(color:Color) {
	match color {
		Color::Red => println!("Your favourite color is Red !"),
		Color::Blue => println!("Your favourite color is Blue"),


	}
}