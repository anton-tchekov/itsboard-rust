use std::env;

mod minesweeper;
use crate::minesweeper::*;

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
		println!("Usage ./minesweeper filename");
		return;
	}

	let filename = &args[1];
	match annotate_file(filename) {
		Ok(result) => {
			for line in &result {
				println!("{}", line);
			}
		}
		Err(err) => {
			println!("{}", err);
		}
	};
}
