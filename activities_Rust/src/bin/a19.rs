// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock


use std::collections::HashMap;


fn main() {
	let mut store = HashMap::new();
	store.insert("Chairs".to_owned(), 5);
	store.insert("Beds".to_owned(), 3);
	store.insert("Tables".to_owned(), 2);
	store.insert("Couches".to_owned(), 0);

	let mut total_items = 0;
	for (furniture, stock) in store {
		if stock == 0 {
			println!("There are no {} left", furniture)
		} else {
			println!("There are {} {} in the sore", stock, furniture);
		}
		total_items += stock;
	}
	println!("The total of furnitures left in the store is: {}", total_items);

}