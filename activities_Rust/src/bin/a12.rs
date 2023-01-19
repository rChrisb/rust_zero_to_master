// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct ShippingBox {
	dimensions: (f64, f64, f64),
	weight: i32,
	color: Color
}
enum Color {
	Red,
	Blue,
	Green
}

impl ShippingBox {
	fn create_box(dimensions: (f64, f64, f64), weight: i32, color: Color) -> Self{
		Self {
			dimensions,
			weight,
			color
		}
	}
	fn print_characteristics(&self) {
		println!("dimensions: {:?}, weight: {:?}", self.dimensions, self.weight);
		match self.color {
			Color::Red => println!("Red"),
			Color::Blue => println!("Blue"),
			Color::Green => println!("Green"),
		}
	}
}



fn main() {
	let shipbox = ShippingBox::create_box((2.5, 4.0, 10.0), 3, Color::Red);

	shipbox.print_characteristics();
}
