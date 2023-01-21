// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

//go to creates.io and search for the crate, then copy the Cargo.toml dependency, ang go to the crate doc to copy function
use chrono::prelude::*;
fn main() {
    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    println!("{}\n\n\n", utc.format("%a %b %e %T %Y").to_string());
    println!("{}", local.format("%a %b %e %T %Y").to_string())
}