// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
enum MenuAction {
    Add,
    View,
    Remove,
    Update,
    GoBack,
}
use std::io;
use std::collections::HashMap;

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please type your data again");
    }
    let input = buffer.trim().to_owned();
    if input == "" {
        None
    } else {
        Some(input)
    }
}
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => {
                return None;
            }
        };
        if input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => {
                return Some(amount);
            }
            Err(_) => println!("Please enter a number"),
        }
    }
}
impl MenuAction {
    fn show() {
        println!(
            "\n =====BILL MANAGER=====\n\n1. Add a new Bill\n2. Views your Bills\n3. Remove a Bill\n4. Edit a Bill\n\nEnter your choice"
        )
    }
    fn from_str(input: &str) -> Option<MenuAction> {
        match input {
            "1" => Some(Self::Add),
            "2" => Some(Self::View),
            "3" => Some(Self::Remove),
            "4" => Some(Self::Update),
            _ => None,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}
mod menu {
    use crate::{ Bill, Bills, get_input, get_bill_amount };
    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        let name = match get_input() {
            Some(input) => input,
            None => {
                return;
            }
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => {
                return;
            }
        };
        let bill = Bill {
            name,
            amount,
        };
        bills.add(bill);
        println!("Bill has been added")
    }
    pub fn view_all_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }
    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove");
        let name = match get_input() {
            Some(name) => name,
            None => {
                return;
            }
        };
        if bills.remove(&name) {
            println!("Bill removed")
        } else {
            println!("Bill not found")
        }
    }
    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter the bill you want to update:");
        let name = match get_input() {
            Some(name) => name,
            None => {
                return;
            }
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => {
                return;
            }
        };
        if bills.update(&name, amount) {
            println!("Bill updated")
        } else {
            println!("Bill not found")
        }
    }
}

// pub fn remove_bill(name: &str, amount: f64, mut bills: Vec<Bill>) -> Vec<Bill> {
//     bills.retain(|bill| bill.name != name && bill.amount_owed != amount);
//     println!("----------Bills have been updated------\n");
//     return bills;
// }
// pub fn edit_bill(
//     mut bills: Vec<Bill>,
//     name: &str,
//     amount: f64,
//     new_name: &str,
//     new_amount: f64
// ) -> Vec<Bill> {
//     for bill in bills.iter_mut() {
//         if bill.name == name && bill.amount_owed == amount {
//             bill.name = new_name.to_owned();
//             bill.amount_owed = new_amount;
//         }
//     }
//     println!("----------Bills have been updated------\n");
//     return bills;
// }

// pub fn see_bills(bills: &Vec<Bill>) {
//     let mut bill_number = 1;
//     for bill in bills {
//         println!(
//             "Bill#{:?}\nname:{:?}, amount:{:?}\n\n",
//             bill_number,
//             bill.name,
//             bill.amount_owed
//         );
//         bill_number += 1;
//     }
// }
fn run_program() -> Option<()> {
    let mut bills = Bills::new();
    loop {
        MenuAction::show();
        let input = get_input()?;
        match MenuAction::from_str(input.as_str()) {
            Some(MenuAction::Add) => menu::add_bill(&mut bills),
            Some(MenuAction::View) => menu::view_all_bills(&bills),
            Some(MenuAction::Remove) => menu::remove_bill(&mut bills),
            Some(MenuAction::Update) => menu::update_bill(&mut bills),
            Some(MenuAction::GoBack) => (),

            None => {
                break;
            }
        }
    }
    None
}
fn main() {
    run_program();

    /* let mut bills: Vec<Bill> = Vec::new();
    bills = add_bill("Victor", 2000.0, bills);
    bills = add_bill("Hector", 452.23, bills);
    bills = add_bill("Chris", 1200.25, bills);
    see_bills(&bills);
    bills = remove_bill("Hector", 425.23, bills);
    see_bills(&bills);
    bills = edit_bill(bills, "Victor", 2000.0, "Maxime", 4000.0);
    see_bills(&bills);
    bills = add_bill("Rubie", 45200.25, bills);
    bills = add_bill("Bikoy", 3455.5, bills);
    see_bills(&bills) */
}