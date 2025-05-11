use std::fs;
use image::GenericImageView;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::error::Error;
use image::Pixel;

const DIR: &str = "../icons/";
const ICON_PREFIX: &str = "ICON_";
const SIZE: u32 = 16;
const START_IDX: u32 = 134;

struct State
{
	font: String,
	consts: String,
	idx: u32
}

fn is_set(pixel: [u8; 4]) -> bool
{
	!(pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0)
}

fn handle_path(path: PathBuf, state: &mut State) -> Result<(), Box<dyn Error>>
{
	let filename = path.display().to_string();
	let iconname = ICON_PREFIX.to_string() +
		&(path.file_stem().ok_or("No file stem")?
		.to_str().ok_or("Conversion Error")?
		.to_uppercase());

	let constant = format!("pub const {}: u32 = {};\n", iconname, state.idx);

	println!("> {} {}", filename, iconname);
	let img = image::open(filename)?;

	println!("Dimensions {:?}, {:?}", img.dimensions(), img.color());
	if img.dimensions() != (SIZE, SIZE)
	{
		return Err("Unsupported size".into());
	}

	let mut fontstr = format!("\n\t/* {} part 1 */\n", iconname);
	for y in 0..SIZE
	{
		let mut byte = 0;
		let mut i = 7;
		for x in 0..8
		{
			let pixel = img.get_pixel(x, y).to_rgba().0;
			if is_set(pixel) {
				byte |= 1 << i;
			}

			i -= 1;
		}

		fontstr += &format!("\t{:#04x},\n", byte);
	}

	fontstr += &format!("\n\t/* {} part 2 */\n", iconname);
	for y in 0..SIZE
	{
		let mut byte = 0;
		let mut i = 7;
		for x in 8..SIZE
		{
			let pixel = img.get_pixel(x, y).to_rgba().0;
			if is_set(pixel)
			{
				byte |= 1 << i;
			}

			i -= 1;
		}

		fontstr += &format!("\t{:#04x},\n", byte);
	}

	state.font += &fontstr;
	state.consts += &constant;
	state.idx += 2;

	println!("");
	Ok(())
}

fn handle_direntry(entry: Result<DirEntry, std::io::Error>, state: &mut State)
{
	if let Ok(direntry) = entry
	{
		_ = handle_path(direntry.path(), state);
	}
	else {
		println!("IO Error");
	}
}

fn main()
{
	let mut state = State
	{
		font: String::new(),
		consts: String::new(),
		idx: START_IDX
	};

	println!("Listing icon directory `{}`", DIR);
	let paths = fs::read_dir(DIR).unwrap();
	for path in paths
	{
		handle_direntry(path, &mut state);
	}

	println!("{}\n", state.consts);
	println!("{}\n", state.font);
}
