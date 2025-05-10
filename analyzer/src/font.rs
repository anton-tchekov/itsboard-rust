use crate::lcd::*;
use crate::terminus16::*;

pub struct Font
{
	pub width: u32,
	pub height: u32,
	pub bitmap: &'static [u8]
}

impl Font
{
	pub fn width(&self, s: &str) -> u32
	{
		self.width * s.len() as u32
	}
}

pub fn lcd_font(x: u32, y: u32, o: u32, fg: u16, bg: u16, font: &Font)
{
	lcd_window_start(x, y, font.width, font.height);
	let stride = (font.width + 7) >> 3;
	let offset = (o - 32) * font.height * stride;
	let cs = offset as usize;
	let char_bitmap = &font.bitmap[cs..font.bitmap.len()];

	let mut y0 = 0;
	while y0 < font.height
	{
		let mut x0 = 0;
		while x0 < font.width
		{
			let byte = ((y0 * stride) + (x0 >> 3)) as usize;
			let bit = 1 << (7 - (x0 & 0x7));
			lcd_emit(if (char_bitmap[byte] & bit) != 0 { fg } else { bg });
			x0 += 1;
		}

		y0 += 1;
	}

	lcd_window_end();
}

pub fn lcd_char(x: u32, y: u32, c: u32, fg: u16, bg: u16, font: &Font)
{
	let mut o = c;
	if o < 32
	{
		o = 127;
	}

	lcd_font(x, y, o, fg, bg, font);
}

pub fn lcd_str(x: u32, y: u32, s: &str, fg: u16, bg: u16, font: &Font)
{
	let mut x0 = x;
	for c in s.chars()
	{
		let mc = match c as u32
		{
			0xB5 => CHAR_MICRO,
			_ => c as u32
		};

		lcd_char(x0, y, mc, fg, bg, font);
		x0 += font.width;
	}
}

pub fn lcd_str_center(x: u32, y: u32, s: &str, fg: u16, bg: u16, font: &Font)
{
	lcd_str(x - font.width(s) / 2,
		y - font.height / 2,
		s, fg, bg, font);
}

pub fn lcd_icon_color(x: u32, y: u32, icon: u32, fg: u16, bg: u16)
{
	lcd_font(x, y, icon, fg, bg, &TERMINUS16);
	lcd_font(x + TERMINUS16.width, y, icon + 1, fg, bg, &TERMINUS16);
}

pub fn lcd_icon_bw(x: u32, y: u32, icon: u32)
{
	lcd_icon_color(x, y, icon, LCD_WHITE, LCD_BLACK);
}

pub fn lcd_icon_undraw(x: u32, y: u32)
{
	lcd_rect(x, y, 2 * TERMINUS16.width, TERMINUS16.height, LCD_BLACK);
}

pub fn lcd_rect_border(x: u32, y: u32, w: u32, h: u32, border: u32, color: u16)
{
	lcd_rect(x, y, w, border, color);
	lcd_rect(x, y + border, border, h - 2 * border, color);
	lcd_rect(x + w - border, y + border, border, h - 2 * border, color);
	lcd_rect(x, y + h - border, w, border, color);
}
