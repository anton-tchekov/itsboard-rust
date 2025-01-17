#![no_std]
#![no_main]

mod gpio;
mod uart;
mod font;
mod spi;
mod timer;
mod delay;
mod lcd_ll;
mod lcd;
mod terminus16;
mod terminus16_bold;

use crate::terminus16::*;
use crate::terminus16_bold::*;
use crate::lcd::*;
use crate::font::*;
use crate::uart::*;
use crate::timer::*;
use crate::spi::*;
use crate::gpio::*;
use crate::delay::*;

use panic_halt as _;

#[cortex_m_rt::entry]
fn start() -> ! {
	gpio_init();
	timer_init();

	/*
	uart_init(115200);
	uart_tx_str("Hello World");
	*/

	/*
	spi_ll_init();

	lcd_init(lcd_color(0, 0, 0));
	lcd_rect(10, 10, 100, 100, lcd_color(255, 0, 0));
	font_str(200, 200, "Hello World",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16_BOLD);

	font_str(200, 216, "This is a test",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16);
	*/

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
	}
}
