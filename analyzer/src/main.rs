#![cfg_attr(all(not(test), not(feature = "simulator")), no_main)]
#![cfg_attr(all(not(test), not(feature = "simulator")), no_std)]
#[macro_use]

mod macro_utils;
mod font;
mod gui;
mod sample;
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
mod waveform;
mod bit_reader;

#[cfg_attr(not(feature = "simulator"), path="delay.rs")]
#[cfg_attr(feature = "simulator", path="sim_delay.rs")]
mod delay;

#[cfg_attr(not(feature = "simulator"), path="hw.rs")]
#[cfg_attr(feature = "simulator", path="sim_hw.rs")]
mod hw;

#[cfg_attr(not(feature = "simulator"), path="lcd.rs")]
#[cfg_attr(feature = "simulator", path="sim_lcd.rs")]
mod lcd;

#[cfg_attr(not(feature = "simulator"), path="sampler.rs")]
#[cfg_attr(feature = "simulator", path="sim_sampler.rs")]
mod sampler;

#[cfg_attr(not(feature = "simulator"), path="userflash.rs")]
#[cfg_attr(feature = "simulator", path="sim_userflash.rs")]
mod userflash;

#[cfg_attr(not(feature = "simulator"), path="decoder_storage.rs")]
#[cfg_attr(feature = "simulator", path="sim_decoder_storage.rs")]
mod decoder_storage;

#[cfg(feature = "simulator")]
mod sim_main;

#[cfg(feature = "simulator")]
mod graphics;

#[cfg(any(test, feature = "simulator"))]
mod test_utils;

#[cfg(not(feature = "simulator"))]
use crate::hw::*;

#[cfg(not(feature = "simulator"))]
use crate::lcd::*;

#[cfg(not(feature = "simulator"))]
use crate::gui::*;

#[cfg(feature = "simulator")]
use crate::sim_main::simulator;

#[cfg(all(not(test), not(feature = "simulator")))]
use panic_halt as _;

#[cfg(all(not(test), not(feature = "simulator")))]
use cortex_m_rt::entry;

#[allow(clippy::empty_loop)]
#[cfg(all(not(test), not(feature = "simulator")))]
#[cfg_attr(any(not(test), not(feature = "simulator")), entry)]
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

#[cfg(feature = "simulator")]
fn main() -> Result<(), String>
{
	simulator()
}
