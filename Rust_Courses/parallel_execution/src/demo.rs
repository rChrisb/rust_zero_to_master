use std::thread;
use std::time::Duration;
fn main() {
    let data = vec!['a', 'b', 'c', 'd'];
    let caps = thread::spawn(move || {
        let data: Vec<_> = data
            .iter()
            .map(|character| character.to_ascii_uppercase())
            .collect();
        data
    });
    println!("Waiting on thread");

    match caps.join() {
        Ok(num) => println!("{:?}", num),
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