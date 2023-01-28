// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum MenuItem {
//     Drink,
//     Salad,
// }

// #[derive(Debug)]
// struct ItemOrder {
//     item: MenuItem,
//     quantity: u32,
// }

// #[derive(Debug)]
// struct TableOrder {
//     items: Vec<ItemOrder>,
// }

// fn new_table_order() -> TableOrder {
//     TableOrder {
//         items: vec![ItemOrder {
//             item: MenuItem::Drink,
//             quantity: 1,
//         }],
//     }
// }

// type Order = Rc<RefCell<Vec<TableOrder>>>;

// #[derive(Debug)]
// struct Chef(Order);
// #[derive(Debug)]
// struct WaitStaff(Order);
// #[derive(Debug)]
// struct Accounting(Order);
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}

impl DigitalSignBoard {
    fn update(&self) {
        let data = self.display.lock();
        println!("sign data= '{}'", data);
    }
}
fn spawn_display_thread(display_data: SharedSignData) {
    thread::spawn(|| {
        let board = DigitalSignBoard {
            display: display_data,
        };
        loop {
            board.update();
            thread::sleep(Duration::from_millis(200));
        }
    });
}

fn change_data(display_data: SharedSignData, new_data: &str) {
    let mut data = display_data.lock();
    *data = new_data.to_owned();
    println!("----------updated: {}", new_data)
}

fn main() {
    let display_data = Arc::new(Mutex::new("initial".to_owned()));
    spawn_display_thread(Arc::clone(&display_data));

    thread::sleep(Duration::from_millis(100));
    change_data(Arc::clone(&display_data), "message 1");
    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "another message");
    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "goodbye");
    thread::sleep(Duration::from_millis(600));

    // let orders = Rc::new(RefCell::new(vec![]));
    // let chef = Chef(Rc::clone(&orders));
    // let wait_staff = WaitStaff(Rc::clone(&orders));
    // let account = Accounting(Rc::clone(&orders));

    // let order = new_table_order();

    // {
    //     orders.borrow_mut().push(order);
    // }

    // dbg!(chef.0.borrow());
    // drop(chef);
    // dbg!(wait_staff.0.borrow());
    // drop(wait_staff);
    // dbg!(account.0.borrow());
}