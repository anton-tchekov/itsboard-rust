use std::env;
use std::error::Error;
mod minesweeper;
use crate::minesweeper::*;
use std::fs::File;
use std::io::Write;

struct Config
{
	filename: String
}

impl Config
{
	fn build(mut args: env::Args) -> Result<Config, Box<dyn Error>>
	{
		if args.len() == 1 {
			let filename = args.next().unwrap();
			Ok(Config { filename })
		}
		else {
			Err("Wrong number of arguments".into())
		}
	}
}

fn main() {

	/*let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
		eprintln!("Usage ./minesweeper filename");
		return;
	}

	let filename = &args[1];*/

	let mut arg_iter = env::args();
	let _program_name = arg_iter.next().unwrap();
	let config = Config::build(arg_iter).unwrap_or_else(|err| {
		eprintln!("Error parsing command line args: {err}");
		eprintln!("Usage ./minesweeper filename");
		std::process::exit(1);
	});

	match annotate_file(&config.filename) {
		Ok(result) => {
			/*for line in &result {
				println!("{}", line);
			}*/

			let path = config.filename.split(".").collect::<Vec<_>>()[0].to_owned() + ".out";
			let mut output = File::create(path).expect("Failed to open file for writing");
			for line in &result {
				write!(output, "{}\n", line).expect("Fail");
			}
		}
		Err(err) => {
			println!("{}", err);
			std::process::exit(1);
		}
	};
}
