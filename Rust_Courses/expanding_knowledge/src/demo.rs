#[allow(dead_code)]
// enum Discount {
// 	Percent(i32),
// 	Flat(i32)
// }

// #[allow(dead_code)]
// struct Ticket {
// 	event: String,
// 	price: i32
// }




struct Survery {
	q1: Option<i32>,
	q2: Option<bool>,
	q3: Option<String>
}
fn main(){
	let response = Survery {
		q1: Some(12),
		q2: None,
		q3:(Some("I am not sure".to_owned()))
	};
	match response.q1 {
		Some(answer) => println!("You responded {} to the question 1", answer),
		None => println!("No response to the question 1")
	}
	match response.q2 {
		Some(answer) => println!("You responded {} to the question 2", answer),
		None => println!("No response to the question 2")
	}
	match response.q3 {
		Some(answer) => println!("You responded {} to the question 3", answer),
		None => println!("No response to the question 3")
	}
	
	
	// let number = 3;
	// match number {
	// 	3 => println!("three"),
	// 	other => println!("{}", other)
	// }
	// let discount = Discount::Flat(78);
	// match discount {
	// 	Discount::Flat(2) => println!("flat 2"),
	// 	Discount::Flat(amount) => println!("{}", amount),
	// 	_ => ()
	// }
	// let concert = Ticket {
	// 	event: "concert".to_owned(),
	// 	price: 45
	// };
	// match concert {
	// 	Ticket {price, ..} => println!("You are going to a concert, the price is {}", price),
	// 	Ticket {price: 45, event} => println!("the price of your event is {}", concert.price)
	// }
}