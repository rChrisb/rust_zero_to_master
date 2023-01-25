// trait Fall {
//     fn hit_ground(&self);
// }
// struct Vase;
// impl Fall for Vase {
//     fn hit_ground(&self) {
//         println!("the vase broke")
//     }
// }
// struct Cat;

// impl Fall for Cat {
//     fn hit_ground(&self) {
//         println!("the cat walked away")
//     }
// }

// fn fall(thing: impl Fall) {
//     thing.hit_ground()
// }
trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}
fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process()
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("checked in as pilot")
    }
    fn process(&self) {
        println!("Pilot enters the cockpit");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("checked in as passenger")
    }
    fn process(&self) {
        println!("passenger takes a seat");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo checked in")
    }
    fn process(&self) {
        println!("cargo moved to storage");
    }
}
fn main() {
    let paul = Pilot;
    let matthiew = Passenger;
    let khloe = Cargo;
    process_item(paul);
    process_item(matthiew);
    process_item(khloe)
    // fall(Vase);
    // fall(Cat)
}