// fn main() {
//     println!("Hello, world!");
// }

fn main()
{
    let mut add = my_first_function(15, 7);
    println!("{}", add);
    add = my_first_function(add,5);
    println!("{:?}", add);
    add = my_first_function(add,15);
    println!("{add}");
}

fn my_first_function(a: i32, b: i32) -> i32 {
    a + b
}