use crossbeam_channel::unbounded;
use std::thread;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}
fn main() {
    let (sender, receiver) = unbounded();
    let handle = thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(msg) =>
                    match msg {
                        ThreadMsg::PrintData(msg) => println!("{}", msg),
                        ThreadMsg::Sum(x, y) => println!("{} + {} = {}", x, y, x + y),
                        ThreadMsg::Quit => {
                            println!("Disconnecting");
                            break;
                        }
                    }
                Err(msg) => {
                    println!("Disconnecting");
                    break;
                }
            }
        }
    });
    sender.send(ThreadMsg::PrintData("Hello World!".to_owned()));
    sender.send(ThreadMsg::Sum(7, 8));
    sender.send(ThreadMsg::Quit);

    handle.join();
    // let data = vec!['a', 'b', 'c', 'd'];
    // let caps = thread::spawn(move || {
    //     let data: Vec<_> = data
    //         .iter()
    //         .map(|character| character.to_ascii_uppercase())
    //         .collect();
    //     data
    // });
    // println!("Waiting on thread");

    // match caps.join() {
    //     Ok(num) => println!("{:?}", num),
    //     Err(num) => println!("{:?}", num),
    // }
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