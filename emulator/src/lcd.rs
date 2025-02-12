use crate::delay::*;
use crate::hw::*;

pub const LCD_WIDTH: u32 = 480;
pub const LCD_HEIGHT: u32 = 320;

pub const LCD_BLACK: u16 = 0x0000;
pub const LCD_WHITE: u16 = 0xFFFF;

pub fn lcd_emit(color: u16) {
	// TODO
}

pub fn lcd_window_start(x: u32, y: u32, w: u32, h: u32) {
	// TODO
}

pub fn lcd_window_end() {
	// TODO
}

pub fn lcd_rect(x: u32, y: u32, w: u32, h: u32, color: u16) {
	let mut count = w * h;
	lcd_window_start(x, y, w, h);
	while count > 0 {
		count -= 1;
		emit(color);
	}

	lcd_window_end();
}

pub fn lcd_callback(x: u32, y: u32, w: u32, h: u32,
	callback: &dyn Fn(u32, u32) -> u16) {
	lcd_window_start(x, y, w, h);
	let mut y0 = 0;
	while y0 < h {
		let mut x0 = 0;
		while x0 < w {
			lcd_emit(callback(x0, y0));
			x0 += 1;
		}

		y0 += 1;
	}

	lcd_window_end();
}

pub fn lcd_clear(color: u16) {
	lcd_rect(0, 0, LCD_WIDTH, LCD_HEIGHT, color);
}

pub fn lcd_init(color: u16) {
	lcd_clear(color);
}

pub const fn lcd_color(r: u8, g: u8, b: u8) -> u16 {
	((r as u16 & 0xF8) << 8) |
		((g as u16 & 0xFC) << 3) |
		(b as u16 >> 3)
}

pub fn lcd_vline(x: u32, y: u32, h: u32, color: u16) {
	lcd_rect(x, y, 1, h, color);
}

pub fn lcd_hline(x: u32, y: u32, w: u32, color: u16) {
	lcd_rect(x, y, w, 1, color);
}
