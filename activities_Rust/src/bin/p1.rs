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

#[derive(Debug)]
#[derive(Clone)]
struct Bill {
    name: String,
    amount_owed: f64,
}
fn add_bill(name: &str, amount: f64, mut bills: Vec<Bill>) -> Vec<Bill> {
    let new_bill = Bill {
        name: name.to_owned(),
        amount_owed: amount,
    };
    bills.push(new_bill);
    return bills;
}

fn see_bills(bills: &Vec<Bill>) {
    let mut bill_number = 1;
    for bill in bills {
        println!("Bill#{:?}\nname:{:?}, amount:{:?}\n\n", bill_number, bill.name, bill.amount_owed);
        bill_number += 1;
    }
}
fn remove_bill(name: &str, amount: f64, mut bills: Vec<Bill>) -> Vec<Bill> {
    bills.retain(|bill| bill.name != name && bill.amount_owed != amount);
    return bills;
}

fn main() {
    let mut bills: Vec<Bill> = Vec::new();
    bills = add_bill("Victor", 2000.0, bills);
    bills = add_bill("Hector", 452.23, bills);
    bills = add_bill("Chris", 1200.25, bills);
    see_bills(&bills);
    bills = remove_bill("Hector", 425.23, bills);
    see_bills(&bills);
}