#![no_std]
#![no_main]

mod hw;
mod font;
mod delay;
mod lcd;
mod sd;
mod gui;
mod sample;
mod sampler;
mod decoder;
mod terminus16;
mod terminus16_bold;
mod bytewriter;
mod decoder_spi;
mod decoder_i2c;
mod decoder_onewire;
mod decoder_uart;

use crate::hw::*;
use crate::lcd::*;
use crate::delay::*;
use crate::gui::*;
use crate::sd::*;

use panic_halt as _;

#[cortex_m_rt::entry]
fn start() -> ! {
	let mut hw = hw_init();
	let sd = Sd::init().ok();
	lcd_init(lcd_color(0, 0, 0));
	let mut gui = Gui::init(sd);

	blueinput();
	yellowinput();

	/*let mut data: [Sample; 50_000] = [0; 50_000];
	let mut buf = SampleBuffer {
		sample_rate: 1_000_000,
		samples: &mut data,
		len: 0
	};

	sample_blocking(&mut buf);

	for i in 0..buf.len {
		let sample = buf.samples[i];
		writeln!(hw.tx, "{:02x}", sample);
	}*/

	let mut ticks: [u32; 8] = [0; 8];
	let mut last_check = 0;
	loop {
		let t = timer_get();
		if (t - last_check) >= TICKS_PER_US * 1000 {
			last_check = t;
			let btns = buttons_read();
			for i in 0..8 {
				if btns & (1 << i) != 0 {
					// Released
					if ticks[i] > 0 {
						ticks[i] -= 1;
					}
				}
				else {
					// Pressed
					if ticks[i] == 0 {
						gui.key(i as i32);
					}

					ticks[i] = 50;
				}
			}
		}
	}
}
