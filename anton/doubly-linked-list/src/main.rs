mod list;

use crate::list::*;

fn main() {
	println!("Hello, world!");

	let mut list = List::new();

	assert_eq!(list.pop_min(), None);


	list.insert(3);

	list.insert(7);
	list.insert(5);

	for elem in list.into_iter() {
		println!("{}", elem);
	}
}
