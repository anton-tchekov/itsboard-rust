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
mod sd;
mod bytewriter;

use crate::graphics::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use gui::*;
use crate::sd::*;

fn main() -> Result<(), String> {
	// let mut gui = Gui::init(None);

	let mut gui = Gui::init(Some(
		Sd {
			serial: 0,
			capacity: 0,
			oem: [0; 2],
			product_name: [0; 5],
			manufacturer: 0,
			revision: 0,
			manufacturing_year: 0,
			manufacturing_month: 0,
			card_type: 0
		}));

	let mut gfx = Graphics::init()?;
	'running: loop {
		for event in gfx.events.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running;
				},
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					gui.action(Action::Escape);
				},
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
					gui.action(Action::Left);
				},
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
					gui.action(Action::Up);
				},
				Event::KeyDown { keycode: Some(Keycode::Space) | Some(Keycode::Return), .. } => {
					gui.action(Action::Enter);
				},
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
					gui.action(Action::Down);
				},
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
					gui.action(Action::Right);
				},
				Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
					gui.key(7);
				},
				Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
					gui.key(6);
				},
				Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
					gui.key(5);
				},
				Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
					gui.key(4);
				},
				Event::KeyDown { keycode: Some(Keycode::Num5), .. } => {
					gui.key(3);
				},
				Event::KeyDown { keycode: Some(Keycode::Num6), .. } => {
					gui.key(2);
				},
				Event::KeyDown { keycode: Some(Keycode::Num7), .. } => {
					gui.key(1);
				},
				Event::KeyDown { keycode: Some(Keycode::Num8), .. } => {
					gui.key(0);
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
