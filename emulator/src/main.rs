mod graphics;
mod lcd;
mod sample;
mod gui;
mod terminus16;
mod terminus16_bold;
mod font;
mod decoder;
mod decoder_uart;
mod decoder_spi;
mod decoder_i2c;
mod decoder_onewire;

use crate::font::*;
use crate::graphics::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use lcd::*;
use gui::*;
use crate::terminus16_bold::TERMINUS16_BOLD;

fn main() -> Result<(), String> {
	let gui = Gui::init();
	gui.base();
	lcd_str(10, 10, "ITS Board Rust LCD Emulator funktioniert!", LCD_WHITE, LCD_BLACK, &TERMINUS16_BOLD);

	lcd_rect(50, 50, 50, 50, LCD_RED);
	lcd_rect(70, 70, 50, 50, LCD_GREEN);
	lcd_rect(90, 90, 50, 50, LCD_BLUE);

	let mut gfx = Graphics::init()?;
	'running: loop {
		for event in gfx.events.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running;
				},
				_ => {}
			}
		}

		gfx.clear();
		gfx.blit();
		gfx.present();
	}

	Ok(())
}
