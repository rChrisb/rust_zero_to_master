use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}
enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}
fn main() {
    let (worker_tx, receiver_worker) = unbounded();
    let (main_tx, receiver_main) = unbounded();
    let worker = thread::spawn(move || {
        loop {
            match receiver_worker.recv() {
                Ok(msg) =>
                    match msg {
                        WorkerMsg::PrintData(msg) => println!("Worker:{}", msg),
                        WorkerMsg::Sum(x, y) => {
                            println!("Worker: summing...");
                            main_tx.send(MainMsg::SumResult(x + y));
                        }
                        WorkerMsg::Quit => {
                            println!("end of thread by Worker");
                            main_tx.send(MainMsg::WorkerQuit);
                            break;
                        }
                    }
                Err(msg) => {
                    println!("Worker Disconnecting");
                    main_tx.try_send(MainMsg::WorkerQuit);
                    break;
                }
            }
        }
    });
    worker_tx.send(WorkerMsg::PrintData("Hello World!".to_owned()));
    worker_tx.send(WorkerMsg::Sum(7, 8));
    worker_tx.send(WorkerMsg::Quit);

    while let Ok(msg) = receiver_main.recv() {
        match msg {
            MainMsg::SumResult(answer) => println!("Main answer: {}", answer),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

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