// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
	let code = 45;
	let is_big = code > 100;
	message(is_big);
}
fn message(comparator: bool) {
	match comparator {
		false => println!("it's small"),
		true => println!("it's big")
	}

}