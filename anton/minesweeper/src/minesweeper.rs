use std::fmt;
use std::io;
use std::fs::read_to_string;

#[derive(Debug)]
pub enum MinesweeperError {
	IoError(io::Error),
	Jagged(usize),
	InvalidChar((usize, usize))
}

impl fmt::Display for MinesweeperError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MinesweeperError::Jagged(row) => {
				write!(f, "Line {} is not the same length as the first", row)
			}
			MinesweeperError::InvalidChar((row, col)) => {
				write!(f, "Invalid character on Line {}:{}", row, col)
			}
			MinesweeperError::IoError(err) => {
				write!(f, "{}", err)
			}
		}
	}
}

fn get_byte(minefield: &[&str], x: usize, y: usize) -> u8 {
	minefield[y].as_bytes()[x]
}

fn get_field(minefield: &[&str], x: i32, y: i32) -> u32 {
	if y < 0 || y >= minefield.len() as i32 ||
		x < 0 || x >= minefield[y as usize].len() as i32 {
		return 0;
	}

	if get_byte(minefield, x as usize, y as usize) == b'*' { 1 } else { 0 }
}

fn verify(minefield: &[&str]) -> Result<(), MinesweeperError> {
	let mut len = 0;
	for y in 0..minefield.len() {
		let cur_len = minefield[y].len();
		if y == 0 {
			len = cur_len;
		}
		else if len != cur_len {
			return Err(MinesweeperError::Jagged(y + 1));
		}

		for x in 0..minefield[y].len() {
			let c = get_byte(minefield, x, y);
			if c != b'*' && c != b' ' {
				return Err(MinesweeperError::InvalidChar((y + 1, x)));
			}
		}
	}

	Ok(())
}

pub fn annotate(minefield: &[&str]) -> Result<Vec<String>, MinesweeperError> {
	verify(minefield)?;

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

pub fn annotate_file(filename: &str) -> Result<Vec<String>, MinesweeperError> {
	let content: String = read_to_string(filename)
		.map_err(MinesweeperError::IoError)?;

	let lines: Vec<&str> = content.lines().collect();
	annotate(&lines)
}

#[cfg(test)]
pub fn remove_numbers(minefield: &[&str]) -> Vec<String> {
	let mut v = Vec::new();
	for line in minefield {
		let mut s = String::new();
		for c in line.as_bytes() {
			s.push(if *c >= b'0' && *c <= b'9' { ' ' } else { *c as char });
		}

		v.push(s);
	}

	v
}

#[cfg(test)]
fn run_test(expected: Vec<&str>) {
	let test = remove_numbers(&expected);
	let input: Vec<&str> = test.iter().map(String::as_str).collect();
	let result = annotate(&input).unwrap();
	assert_eq!(expected, result);
}

#[test]
fn test_simple() {
	run_test(vec![
		"***",
		"*8*",
		"***"
	]);
}

#[test]
fn test_file() {
	let result = annotate_file("data/simple.txt").unwrap();
	let expected = vec![
		"      2*",
		"2333335*",
		"********",
		"2333335*",
		"      3*",
		"      2*"
	];

	assert_eq!(result, expected);
}

#[test]
fn test_file_missing() {
	assert!(matches!(annotate_file("data/missing.txt"),
		Err(MinesweeperError::IoError(..))));
}

#[test]
fn test_jagged() {
	assert!(matches!(
		annotate(&vec![
			"   ",
			"  ",
			" "
		]),
		Err(MinesweeperError::Jagged(2))
	));
}

#[test]
fn test_invalid() {
	assert!(matches!(
		annotate(&vec![
			"  *",
			"  *",
			"  A"
		]),
		Err(MinesweeperError::InvalidChar((3, 2)))
	));
}

#[test]
fn test_large() {
	run_test(vec![
		"******************************",
		"*53333333333333333333333333332",
		"*3                            ",
		"*3                  1221      ",
		"*3                  2**2      ",
		"*3                  2**2      ",
		"*3                  1221      ",
		"*3                            ",
		"*3           111              ",
		"*3          13*31             ",
		"*3          1***1             ",
		"*3          13*31             ",
		"*3           111              ",
		"*2                            "
	]);
}

#[test]
fn test_full() {
	run_test(vec![
		"********",
		"********",
		"********",
		"********",
		"********",
		"********",
		"********"
	]);
}

#[test]
fn test_empty() {
	run_test(vec![]);
}

#[test]
fn test_onerow() {
	run_test(vec![
		"1*2*2*2*2*2*1"
	]);
}

#[test]
fn test_onecol() {
	run_test(vec![
		" ",
		"1",
		"*",
		"1",
		"1",
		"*",
		"1",
		" "
	]);
}
