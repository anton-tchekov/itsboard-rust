struct IconBW {
	width: u32,
	height: u32,
	data: &'static [u8]
}

struct IconRGB565 {
	width: u32,
	height: u32,
	data: &'static [u16]
}

impl IconBW {
	fn render(&self) {

	}
}

impl IconRGB565 {
	fn render(&self) {

	}
}
