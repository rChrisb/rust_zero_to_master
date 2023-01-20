fn add(a:i32, b:i32) -> i32 {
	a + b
}

fn main () {
	let sum_1 = add(3,5);
	println!("{sum_1}");

	let mul = |a:i32, b:i32 | ->i32 {
		a * b
	};
	let div = |a, b| a/b;
	println!("{:?}", mul(5, 4));
	println!("{:?}", div(8, 2));
}