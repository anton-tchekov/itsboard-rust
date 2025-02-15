#![no_std]
#![no_main]

mod hw;
mod font;
mod delay;
mod lcd;
mod sd;
mod gui;
mod sample;
mod decoder;
mod terminus16;
mod terminus16_bold;
mod fat;
mod bytewriter;
mod decoder_spi;
mod decoder_i2c;
mod decoder_onewire;
mod decoder_uart;
mod decoders;

use crate::hw::*;
use crate::terminus16::*;
use crate::terminus16_bold::*;
use crate::lcd::*;
use crate::font::*;
use crate::delay::*;
use crate::decoder::*;
use crate::gui::*;
use crate::sample::*;
use crate::sd::*;
use crate::fat::*;
use crate::bytewriter::*;
use crate::decoder_spi::*;
use core::fmt::Write;

use panic_halt as _;

#[cortex_m_rt::entry]
fn start() -> ! {
	let mut hw = hw_init();
	lcd_init(lcd_color(0, 0, 0));

	let gui = Gui::init();

	delay_ms(2000);
	gui.base();

	let mut data: [Sample; 100_000] = [0; 100_000];
	let mut buf = SampleBuffer {
		sample_rate: 1_000_000,
		samples: &mut data,
		len: 0
	};

	sample_blocking(&mut buf);

	for i in 0..buf.len {
		let sample = buf.samples[i];
		writeln!(hw.tx, "{:#08b}", sample);
	}

	/*lcd_rect(10, 10, 100, 100, lcd_color(255, 0, 0));
	lcd_rect(30, 30, 100, 100, lcd_color(0, 255, 0));
	lcd_rect(50, 50, 100, 100, lcd_color(0, 0, 255));

	lcd_str(200, 200, "Hello World from Rust!",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16_BOLD);

	lcd_str(200, 216, "Finally it works ...",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16);*/

	blueclear(0xFF);
	yellowclear(0xFF);

	let mut rx = 100;
	let mut ry = 100;
	loop {
		let ctr = timer_get();
		let seconds = ctr / (TICKS_PER_US * 1000 * 1000);

		let mut buf = [0u8; 20];
		let mut buf = ByteMutWriter::new(&mut buf[..]);

		buf.clear();
		write!(&mut buf, "Seconds: {:>10}", seconds).unwrap();

		lcd_str(0, 0, buf.as_str(),
			lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16);

		let btns = buttons_read();
		if btns & 1 == 0 {
			ry -= 1;
		}

		if btns & 2 == 0 {
			ry += 1;
		}

		if btns & 4 == 0 {
			rx -= 1;
		}

		if btns & 8 == 0 {
			rx += 1;
		}

		lcd_rect(rx, ry, 20, 20, lcd_color(255, 0, 255));
		delay_ms(20);

		/*blueclear(0xFF);
		yellowclear(0xFF);

		blueset(0xAA);
		yellowset(0x55);

		delay_ms(1000);

		yellowclear(0xFF);
		blueclear(0xFF);

		yellowset(0xAA);
		blueset(0x55);

		delay_ms(1000);

		writeln!(hw.tx, "Main loop running\n").unwrap();*/
	}
}
