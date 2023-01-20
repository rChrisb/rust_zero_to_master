#[allow(dead_code)]
enum Discount {
	Percent(i32),
	Flat(i32)
}

#[allow(dead_code)]
struct Ticket {
	event: String,
	price: i32
}


fn main(){
	let number = 3;
	match number {
		3 => println!("three"),
		other => println!("{}", other)
	}
	let discount = Discount::Flat(78);
	match discount {
		Discount::Flat(2) => println!("flat 2"),
		Discount::Flat(amount) => println!("{}", amount),
		_ => ()
	}
	let concert = Ticket {
		event: "concert".to_owned(),
		price: 45
	};
	match concert {
		Ticket {price, ..} => println!("You are going to a concert, the price is {}", price),
		Ticket {price: 45, event} => println!("the price of your event is {}", concert.price)
	}



}