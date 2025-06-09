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
mod bytewriter;
mod sampler;
mod tinyfont;
mod waveform;
mod positionindicator;
mod decoder_framebuffer;
mod delay;
mod hw;
mod userflash;
mod decoder_storage;

use crate::hw::HW;
use crate::graphics::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use gui::*;

fn main() -> Result<(), String> {
	let mut hw = HW::new();
	let mut gui = Gui::init(hw);

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
				Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
					gui.action(Action::Cycle);
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
