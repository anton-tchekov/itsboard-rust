use crate::terminus16::*;
use crate::lcd::*;
use crate::font::*;

pub fn lcd_icon_color(x: u32, y: u32, icon: u32, fg: u16, bg: u16) {
	lcd_font(x, y, icon, fg, bg, &TERMINUS16);
	lcd_font(x + TERMINUS16.width, y, icon + 1, fg, bg, &TERMINUS16);
}

pub fn lcd_icon_bw(x: u32, y: u32, icon: u32) {
	lcd_icon_color(x, y, icon, LCD_WHITE, LCD_BLACK);
}
