// fn add(a:i32, b:i32) -> i32 {
// 	a + b
// }
// fn maybe_num() -> Option<i32> {...}
// fn maybe_word() -> Option<String> {...}
fn main () {
	
	let a: Option<i32> = Some(7);
	let a_is_some = a.is_some();
	let a_is_none = a.is_none();
	let a_mapped = a.map(|num| num + 1);
	let a_or_else = a.or_else(|| Some(10));
	let unwrapped = a.unwrap_or_else(|| 0);
	let a_filtered = a.filter(|num| num == &4);

	
	// let plus_one = match maybe_num() {
	// 	Some(num) => Some(nu + 1),
	// 	None => None
	// };
	// let pluss__one = maybe_num.map(|num| num + 1);
	// let world_length = maybe_word.map(|word| word.len());
	// let sum_1 = add(3,5);
	// println!("{sum_1}");

	// let mul = |a:i32, b:i32 | ->i32 {
	// 	a * b
	// };
	// let div = |a, b| a/b;
	// println!("{:?}", mul(5, 4));
	// println!("{:?}", div(8, 2));
}