use std::thread;
use std::time::Duration;
fn main() {
    let value = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        65
    });
    println!("Waiting on thread");

    match value.join() {
        Ok(num) => println!("{}", num),
        Err(num) => println!("{:?}", num),
    }
    // let iterations = 10;
    // let a = thread::spawn(move || {
    //     for i in 1..=iterations {
    //         println!("A:{}", i);
    //     }
    // });
    // let b = thread::spawn(move || {
    //     for i in 1..=iterations {
    //         println!("        B:{}", i);
    //     }
    // });
    // a.join();
    // b.join();
}