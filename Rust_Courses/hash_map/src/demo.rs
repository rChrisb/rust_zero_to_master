use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Contents {
	content: String,
}

fn main () {
	let mut lockers = HashMap::new();
	lockers.insert(1, Contents {content: "stuff".to_owned()});
	lockers.insert(7, Contents {content: "things".to_owned()});
	lockers.insert(3, Contents {content: "choses".to_owned()});
	lockers.insert(4, Contents {content: "machin".to_owned()});
	for (locker_number, contents) in lockers.iter() {
		println!("locker number:{:?}, content:{:?}", locker_number, contents.content);
	}

}