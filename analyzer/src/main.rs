#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#[macro_use]

mod macro_utils;
mod hw;
mod font;
mod delay;
mod lcd;
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
mod tinyfont;
mod timeindicator;
mod timeline;
mod durationindicator;
mod cursors;
mod positionindicator;
mod decoder_framebuffer;
mod decoder_storage;
mod waveform;
mod userflash;
mod bit_reader;

#[cfg(test)]
mod test_utils;

use crate::hw::*;
use crate::lcd::*;
use crate::delay::*;
use crate::gui::*;

#[cfg(not(test))]
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
	pac,
	prelude::*,
};

#[allow(clippy::empty_loop)]
#[cfg_attr(not(test), entry)]
fn start() -> !
{
	let hw = hw_init();
	lcd_init(lcd_color(0, 0, 0));
	let mut gui = Gui::init(hw);
	blueinput();
	yellowinput();

	let mut ticks: [u32; 8] = [0; 8];
	let mut last_check = 0;
	loop
	{
		let t = timer_get();
		if (t - last_check) >= TICKS_PER_US * 1000
		{
			last_check = t;
			let btns = buttons_read();
			for i in 0..8
			{
				if btns & (1 << i) != 0
				{
					// Released
					if ticks[i] > 0
					{
						ticks[i] -= 1;
					}
				}
				else
				{
					// Pressed
					if ticks[i] == 0
					{
						gui.key(i as i32);
					}

					ticks[i] = 50;
				}
			}
		}
	}
}
