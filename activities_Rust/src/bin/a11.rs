// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
	quantity:i32,
	id_number:i32
}
fn display_quantity(grocery_item: &Grocery){
	println!("quantity: {}", grocery_item.quantity);
}
fn display_id_number(grocery_item: &Grocery){
	println!("id: {}", grocery_item.id_number);

}

fn main() {
	let item = Grocery {
		quantity: 52,
		id_number: 1552
	};
	display_quantity(&item);
	display_id_number(&item);

}