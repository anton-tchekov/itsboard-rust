mod graphics;
mod lcd;

use crate::graphics::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use lcd::*;

fn main() -> Result<(), String> {

	//lcd_init(LCD_BLACK);
	lcd_rect(10, 10, 100, 100, LCD_RED);
	lcd_rect(50, 50, 100, 100, LCD_GREEN);
	lcd_rect(100, 100, 100, 100, LCD_BLUE);

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
