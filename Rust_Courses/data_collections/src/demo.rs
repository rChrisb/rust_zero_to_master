// struct Temperature {
// 	degree: f64,
// }
// impl Temperature {
// 	fn freezing()-> Self {
// 		Self {
// 			degree: -32.6
// 		}
// 	}

// 	fn show_temperature(&self){
// 		println!("It's {} degree F outside", self.degree);
// 	}
	
// }
struct Test {
	score: i32
}
fn main () {
	let my_score = vec![
		Test {score: 10},
		Test {score: 15},
		Test {score: 18},

	];
	let mut total = 0;
	for test in &my_score {
		total += test.score
	}
	println!("{}", my_score.len());
	let average: f64 = (total / 3).into();
	println!("{:?}", average);




	// let numbers = vec![1, 2 , 3];
	// for num in numbers {
	// 	println!("{:?}", num);
	// }





	// let temp = Temperature {
	// 	degree : 77.8
	// };
	// temp.show_temperature();
	// let another_temp = Temperature::freezing();
}