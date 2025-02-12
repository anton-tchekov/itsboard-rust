mod graphics;
mod lcd;

use crate::graphics::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use lcd::*;

fn main() -> Result<(), String> {
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

		gfx.present();
	}

	Ok(())
}
