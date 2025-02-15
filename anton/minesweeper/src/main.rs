use std::env;

fn get_field(minefield: &[&str], x: i32, y: i32) -> u32 {
	if y < 0 || y >= minefield.len() as i32 ||
		x < 0 || x >= minefield[y as usize].len() as i32 {
		return 0;
	}

	if minefield[y as usize].as_bytes()[x as usize] == b'*' { 1 } else { 0 }
}

pub fn verify(minefield: &[&str]) -> bool {
	let mut len = 0;
	for y in 0..minefield.len() {
		for x in 0..minefield[y].len() {
			if get_field()
		}
	}

	false
}

pub fn annotate(minefield: &[&str]) -> Result<Vec<String>, ()> {
	if !verify(minefield) {
		return Err(());
	}

	let mut result: Vec<String> = Vec::new();
	static OFFSETS: [(i32, i32); 8] = [
		(-1, -1),
		( 0, -1),
		( 1, -1),
		(-1,  0),
		( 1,  0),
		(-1,  1),
		( 0,  1),
		( 1,  1)
	];

	for y in 0..minefield.len() {
		let mut line = String::from("");
		for x in 0..minefield[y].len() {
			let ox = x as i32;
			let oy = y as i32;
			let nc = if get_field(minefield, ox, oy) == 0 {
				let count = OFFSETS.iter().fold(0, |acc, x| {
					acc + get_field(minefield, ox - x.0, oy - x.1)
				});

				if count == 0 { ' ' }
					else { char::from_digit(count, 10).unwrap() }
			}
			else { '*' };

			line.push(nc);
		}

		result.push(line);
	}

	Ok(result)
}

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
		println!("Usage ./minesweeper filename");
		return;
	}

	println!("Hello, world!");
}
