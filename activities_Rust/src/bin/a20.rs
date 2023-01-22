// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
enum Keywords {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}
use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn power_message(keyword: Keywords) {
    match keyword {
        Keywords::Off => println!("Turning OFF"),
        Keywords::Sleep => println!("Sleep mode"),
        Keywords::Reboot => println!("Rebooting"),
        Keywords::Shutdown => println!("Shutting Donw"),
        Keywords::Hibernate => println!("Hibernate mode"),
    }
}

fn main() {
    let user_input = get_input().unwrap().to_lowercase();

    match user_input.as_str() {
        "off" => power_message(Keywords::Off),
        "sleep" => power_message(Keywords::Sleep),
        "reboot" => power_message(Keywords::Reboot),
        "shutdown" => power_message(Keywords::Shutdown),
        "hibernate" => power_message(Keywords::Hibernate),
        _ => println!("Error: Not a keyword"),
    }
}