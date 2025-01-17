const LCD_RST: u32 = 12;
const LCD_DC: u32 = 13;
const LCD_CS: u32 = 14;

use stm32f4::stm32f429::*;

pub fn lcd_rst_0() {
	unsafe { (*GPIOF::ptr()).bsrr.write(|w| w.bits(1 << (LCD_RST + 16))); }
}

pub fn lcd_rst_1() {
	unsafe { (*GPIOF::ptr()).bsrr.write(|w| w.bits(1 << LCD_RST)); }
}

pub fn lcd_dc_0() {
	unsafe { (*GPIOF::ptr()).bsrr.write(|w| w.bits(1 << (LCD_DC + 16))); }
}

pub fn lcd_dc_1() {
	unsafe { (*GPIOF::ptr()).bsrr.write(|w| w.bits(1 << LCD_DC)); }
}

pub fn lcd_cs_0() {
	unsafe { (*GPIOD::ptr()).bsrr.write(|w| w.bits(1 << (LCD_CS + 16))); }
}

pub fn lcd_cs_1() {
	unsafe { (*GPIOD::ptr()).bsrr.write(|w| w.bits(1 << LCD_CS)); }
}
