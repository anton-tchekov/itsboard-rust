use crate::lcd::*;
use crate::terminus16::*;

pub struct Font {
	pub width: u32,
	pub height: u32,
	pub bitmap: &'static [u8]
}

pub fn lcd_font(x: u32, y: u32, o: u32, fg: u16, bg: u16, font: &Font) {
	lcd_window_start(x, y, font.width, font.height);
	let stride = (font.width + 7) >> 3;
	let offset = (o - 32) * font.height * stride;
	let cs = offset as usize;
	let char_bitmap = &font.bitmap[cs..font.bitmap.len()];

	let mut y0 = 0;
	while y0 < font.height {
		let mut x0 = 0;
		while x0 < font.width {
			let byte = ((y0 * stride) + (x0 >> 3)) as usize;
			let bit = 1 << (7 - (x0 & 0x7));
			lcd_emit(if (char_bitmap[byte] & bit) != 0 { fg } else { bg });
			x0 += 1;
		}

		y0 += 1;
	}

	lcd_window_end();
}

pub fn lcd_char(x: u32, y: u32, c: char, fg: u16, bg: u16, font: &Font) {
	let mut o = c as u32;
	if o < 32 {
		o = 127;
	}

	lcd_font(x, y, o, fg, bg, font);
}

pub fn lcd_str(x: u32, y: u32, s: &str, fg: u16, bg: u16, font: &Font) {
	let mut x0 = x;
	for c in s.chars() {
		lcd_char(x0, y, c, fg, bg, font);
		x0 += font.width;
	}
}

pub fn lcd_icon_color(x: u32, y: u32, icon: u32, fg: u16, bg: u16) {
	lcd_font(x, y, icon, fg, bg, &TERMINUS16);
	lcd_font(x + TERMINUS16.width, y, icon + 1, fg, bg, &TERMINUS16);
}

pub fn lcd_icon_bw(x: u32, y: u32, icon: u32) {
	lcd_icon_color(x, y, icon, LCD_WHITE, LCD_BLACK);
}
