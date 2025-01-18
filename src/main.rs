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
use core::fmt;
use core::str;
use panic_halt as _;

pub struct ByteMutWriter<'a> {
	buf: &'a mut [u8],
	cursor: usize,
}

impl<'a> ByteMutWriter<'a> {
	pub fn new(buf: &'a mut [u8]) -> Self {
		ByteMutWriter { buf, cursor: 0 }
	}

	pub fn as_str(&self) -> &str {
		str::from_utf8(&self.buf[0..self.cursor]).unwrap()
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.buf.len()
	}

	pub fn clear(&mut self) {
		self.cursor = 0;
	}

	pub fn len(&self) -> usize {
		self.cursor
	}

	pub fn empty(&self) -> bool {
		self.cursor == 0
	}

	pub fn full(&self) -> bool {
		self.capacity() == self.cursor
	}
}

impl fmt::Write for ByteMutWriter<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let cap = self.capacity();
		for (i, &b) in self.buf[self.cursor..cap]
			.iter_mut()
			.zip(s.as_bytes().iter())
		{
			*i = b;
		}
		self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());
		Ok(())
	}
}

#[cortex_m_rt::entry]
fn start() -> ! {
	let mut hw = hw_init();
	lcd_init(lcd_color(0, 0, 0));
	delay_ms(100);
	lcd_rect(10, 10, 100, 100, lcd_color(255, 0, 0));
	lcd_rect(30, 30, 100, 100, lcd_color(0, 255, 0));
	lcd_rect(50, 50, 100, 100, lcd_color(0, 0, 255));

	lcd_str(200, 200, "Hello World from Rust!",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16_BOLD);

	lcd_str(200, 216, "Finally it works ...",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16);

	blueclear(0xFF);
	yellowclear(0xFF);

	let mut rx = 100;
	let mut ry = 100;
	loop {
		let ctr = timer_get();
		let seconds = ctr / (TICKS_PER_US * 1000 * 1000);

		//let msg = format!();

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
