// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
	let string_to_transform = "Hello bro how are you";
	let second_string_to_transform = "NICE TO MEET YOU !!";
	println!("UPPERCASE TEST: {} ===>>> {}", string_to_transform, string_to_transform.to_uppercase());
	println!("lowercase test: {} ===>>> {}", second_string_to_transform, second_string_to_transform.to_lowercase());

}