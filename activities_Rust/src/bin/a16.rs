// Topic: Option
// //
// // Requirements:
// // * Print out the details of a student's locker assignment
// // * Lockers use numbers and are optional for students
// //
// // Notes:
// // * Use a struct containing the student's name and locker assignment
// // * The locker assignment should use an Option<i32>


// #[derive(Debug)]
struct StudentLocker {
	name: String,
	assignment: Option<i32>
}

fn main() {
	let student = StudentLocker {
		name: "Chris".to_owned(),
		assignment : Some(14)
	};

	match student.assignment {
		Some(assignment) => println!("locker info:\nstudent=> {} | assignment=> {:?}", student.name, assignment),
		None => println!("{}'s locker has no assignment yet...", student.name)

	}
}