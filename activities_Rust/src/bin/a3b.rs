// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
	let age = 8;

	if age < 5 {
		println!("the age {} is lower 5", age)
	}
	else if age > 5 {
		println!("the age {} is greater 5", age)
	}
	else {
		println!("the age {} is equal to 5", age)
	}
}