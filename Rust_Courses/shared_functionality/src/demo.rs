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
// trait CheckIn {
//     fn check_in(&self);
//     fn process(&self);
// }
// fn process_item<T: CheckIn>(item: T) {
//     item.check_in();
//     item.process()
// }

// struct Pilot;
// impl CheckIn for Pilot {
//     fn check_in(&self) {
//         println!("checked in as pilot")
//     }
//     fn process(&self) {
//         println!("Pilot enters the cockpit");
//     }
// }
// struct Passenger;
// impl CheckIn for Passenger {
//     fn check_in(&self) {
//         println!("checked in as passenger")
//     }
//     fn process(&self) {
//         println!("passenger takes a seat");
//     }
// }
// struct Cargo;
// impl CheckIn for Cargo {
//     fn check_in(&self) {
//         println!("cargo checked in")
//     }
//     fn process(&self) {
//         println!("cargo moved to storage");
//     }
// }
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}
struct ConveyorBelt<T: Convey> {
    items: Vec<T>,
}
impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}
struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}
impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_owned(),
        }
    }
}
impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}
fn main() {
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
    belt.add(CarPart::default());
    // let mut belt: ConveyorBelt = ConveyorBelt { items: vec![] };
    // belt.add(5)

    // let paul = Pilot;
    // let matthiew = Passenger;
    // let khloe = Cargo;
    // process_item(paul);
    // process_item(matthiew);
    // process_item(khloe)
    // fall(Vase);
    // fall(Cat)
}