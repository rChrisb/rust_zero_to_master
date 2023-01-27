#[derive(Debug)]
struct Uppercase(String);
impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}
impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

fn main() {
    let upper = Uppercase::from("lowercase");
    println!("{}", upper.0);
    let upper: Uppercase = "lowercase".into();
    println!("{}", upper.0)
}