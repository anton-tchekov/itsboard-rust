#![no_std]
#![no_main]

mod hw;
mod font;
mod delay;
mod lcd;
mod terminus16;
mod terminus16_bold;

use crate::hw::*;
use crate::terminus16::*;
use crate::terminus16_bold::*;
use crate::lcd::*;
use crate::font::*;
use crate::delay::*;
use core::fmt::Write;

use panic_halt as _;

#[cortex_m_rt::entry]
fn start() -> ! {
	let mut hw = hw_init();
	lcd_init(&mut hw.spi, lcd_color(0, 0, 0));
	delay_ms(100);
	lcd_rect(&mut hw.spi, 10, 10, 100, 100, lcd_color(255, 0, 0));
	lcd_rect(&mut hw.spi, 30, 30, 100, 100, lcd_color(0, 255, 0));
	lcd_rect(&mut hw.spi, 50, 50, 100, 100, lcd_color(0, 0, 255));

	font_str(&mut hw.spi, 200, 200, "Hello World from Rust!",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16_BOLD);

	font_str(&mut hw.spi, 200, 216, "Finally it works ...",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16);

	loop {
		blueclear(0xFF);
		yellowclear(0xFF);

		blueset(0xAA);
		yellowset(0x55);

		delay_ms(1000);

		yellowclear(0xFF);
		blueclear(0xFF);

		yellowset(0xAA);
		blueset(0x55);

		delay_ms(1000);

		writeln!(hw.tx, "Main loop running\n").unwrap();
	}
}
