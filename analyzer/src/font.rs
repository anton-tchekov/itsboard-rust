use crate::lcd::{lcd_rect, lcd_window_start, lcd_emit, lcd_window_end, LCD_BLACK, LCD_WHITE};
use crate::terminus16::{TERMINUS16, Icon, CHAR_DELTA};

pub const CHAR_MISSING: u32 = 127;
pub const CHAR_MICRO: u32 = 128;

pub struct Font
{
	pub horizontal: bool,
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

pub fn lcd_font(x: u32, y: u32, o: u32, fg: u16, bg: u16, font: &Font) -> u32
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
	font.width
}

fn lcd_font_v(x: u32, y: u32, c: u32, fg: u16, bg: u16, font: &Font) -> u32
{
	let offset = font.width * (c - 32);
	lcd_window_start(x, y, font.width, font.height);
	for h in 0..font.height
	{
		for w in 0..font.width
		{
			let byte = font.bitmap[(offset + w) as usize];
			lcd_emit(if ((byte >> h) & 1) != 0 { fg } else { bg });
		}
	}

	lcd_window_end();
	font.width + 1
}

pub fn remap_char(c: u32) -> u32
{
	match c {
		..32 => CHAR_MISSING,
		0xB5 => CHAR_MICRO,
		0x394 => CHAR_DELTA,
		_ => c
	}
}

pub fn lcd_char(x: u32, y: u32, c: u32, fg: u16, bg: u16, font: &Font) -> u32
{
	let c = remap_char(c);
	if font.horizontal
	{
		lcd_font(x, y, c, fg, bg, font)
	}
	else
	{
		lcd_font_v(x, y, c, fg, bg, font)
	}
}

pub fn lcd_str(x: u32, y: u32, s: &str, fg: u16, bg: u16, font: &Font)
{
	let mut x0 = x;
	for c in s.chars()
	{
		x0 += lcd_char(x0, y, c as u32, fg, bg, font);
	}
}

pub fn lcd_str_undraw(x: u32, y: u32, len: usize, font: &Font)
{
	let w = font.width;
	let h = font.height;
	let stride = if font.horizontal { w } else { w + 1 };
	let mut x0 = x;
	for _i in 0..len
	{
		lcd_rect(x0, y, w, h, LCD_BLACK);
		x0 += stride;
	}
}

pub fn lcd_str_center(x: u32, y: u32, s: &str, fg: u16, bg: u16, font: &Font)
{
	lcd_str(x - font.width(s) / 2,
		y - font.height / 2,
		s, fg, bg, font);
}

pub fn lcd_icon_color(x: u32, y: u32, icon: Icon, fg: u16, bg: u16)
{
	let c = icon as u32;
	lcd_font(x, y, c, fg, bg, &TERMINUS16);
	lcd_font(x + TERMINUS16.width, y, c + 1, fg, bg, &TERMINUS16);
}

pub fn lcd_icon_bw(x: u32, y: u32, icon: Icon)
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
