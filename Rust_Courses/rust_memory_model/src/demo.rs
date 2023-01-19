// struct Book {
// 	pages: i32,
// 	rating:i32
// }

// fn display_page_count(book:&Book){
// 	println!("pages = {:?}", book.pages);
// }
// fn display_rating(book:&Book){
// 	println!("rating = {:?}", book.rating);
// }
struct Temperature {
	degree: f64,
}
impl Temperature {
	fn show_temperature(&self){
		println!("It's {} degree F outside", self.degree);
	}
	
}
fn main () {
	let temp = Temperature {
		degree : 77.8
	};
	temp.show_temperature();

	/* let book = Book {
		pages: 5,
		rating: 9
	};
	display_page_count(&book);
	display_rating(&book); */
	
}