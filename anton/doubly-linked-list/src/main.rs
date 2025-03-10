mod list;

use crate::list::*;

fn main() {
	println!("Hello, world!");
	let mut list = List::new();
	list.insert(3);
	list.insert(10);
	list.insert(20);
	list.insert(40);
	list.insert(5);
	list.insert(30);
	list.insert(50);
	println!("len = {}", list.len());
	for elem in list.into_iter() {
		println!("{}", elem);
	}
}
