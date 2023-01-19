// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
	name: String,
	age: i32,
	favourite_color: String
}
fn print_name(name: &str){
	println!("{}:", name);
}
fn print_fav_color(fav_color: &str){
	println!("favourite color: {}", fav_color);
}
fn main() {
	let ramdom_people = vec![
		Person {name: "Mathilde".to_owned(), age: 15, favourite_color: "yellow".to_owned() },
		Person {name: "Léa".to_owned(), age: 5, favourite_color: "pink".to_owned() },
		Person {name: "Camille".to_owned(), age: 10, favourite_color: "purple".to_owned() },

	];
	for kid in ramdom_people {
		if kid.age <= 10{
			print_name(&kid.name);
			print_fav_color(&kid.favourite_color);
		}
	}
}
