// fn add(a:i32, b:i32) -> i32 {
// 	a + b
// }
// fn maybe_num() -> Option<i32> {...}
// fn maybe_word() -> Option<String> {...}
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
}
fn main() {
    let color = Color::Red;
    if let Color::Red = color {
        println!("Red is the best color!")
    }

    /* let range = 'a'..'z'; //range DOESN'T include letter 'z'
    let another_range = 'a'..='z'; //range INCLUDES letter 'z'
    for letter in range {
        println!("{}", letter);
    }
    println!(
        "--------------------------------------------\n--------------------------------------------\n"
    );
    for letter in another_range {
        println!("{}", letter);
    } */

    /*   let numbers = vec![1, 2, 3, 4]; */

    /*   let mut plus_one = vec![];
    for num in numbers {
        plus_one.push(num + 1);
    } */
    // let plus_one: Vec<_> = numbers
    //     .iter()
    //     .map(|num| num + 1)
    //     .filter(|num| num % 2 == 0)
    //     /*    .last(); */
    //     /* .take(1) */
    //     .collect();
    // println!("{:?}", plus_one)

    /* let a: Option<i32> = Some(7);
	let a_is_some = a.is_some();
	let a_is_none = a.is_none();
	let a_mapped = a.map(|num| num + 1);
	let a_or_else = a.or_else(|| Some(10));
	let unwrapped = a.unwrap_or_else(|| 0);
	let a_filtered = a.filter(|num| num == &4); */

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