// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
	let (x, y) = coordination_parameters();

	if y > 5 {
		println!("y-value of catersian coordinate is greater than 5");
	}
	else if y < 5 {
		println!("y-value of catersian coordinate is less than 5");
	} else {

		println!("y-value of catersian coordinate is equal to 5");
	}

}
fn coordination_parameters() -> (i32, i32) {
	return (10, 2);
}