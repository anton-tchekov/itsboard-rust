use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::mpsc;
use std::thread;
use crate::hw::HW;
use crate::graphics::Graphics;
use crate::gui::{Gui, Action};

enum EventMessage
{
	EventKey(i32),
	EventAction(Action)
}

pub fn simulator() -> Result<(), String>
{
	let hw = HW::new();
	let mut gui = Gui::init(hw);
	let (tx, rx) = mpsc::channel();

	thread::spawn(move ||
	{
		loop
		{
			let received = rx.recv();

			match received
			{
				Ok(r) => match r
				{
					EventMessage::EventKey(key) => gui.key(key),
					EventMessage::EventAction(action) => gui.action(action),
				}
				Err(_) => { return; }
			}
		}
	});

	let mut gfx = Graphics::init()?;
	'running: loop {
		for event in gfx.events.poll_iter()
		{
			match event
			{
				Event::Quit {..} =>
				{
					break 'running;
				},
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Escape)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Left)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Up), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Up)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Space) | Some(Keycode::Return), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Enter)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Down), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Down)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Right)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Tab), .. } =>
				{
					tx.send(EventMessage::EventAction(Action::Cycle)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num1), .. } =>
				{
					tx.send(EventMessage::EventKey(7)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num2), .. } =>
				{
					tx.send(EventMessage::EventKey(6)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num3), .. } =>
				{
					tx.send(EventMessage::EventKey(5)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num4), .. } =>
				{
					tx.send(EventMessage::EventKey(4)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num5), .. } =>
				{
					tx.send(EventMessage::EventKey(3)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num6), .. } =>
				{
					tx.send(EventMessage::EventKey(2)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num7), .. } =>
				{
					tx.send(EventMessage::EventKey(1)).unwrap();
				},
				Event::KeyDown { keycode: Some(Keycode::Num8), .. } =>
				{
					tx.send(EventMessage::EventKey(0)).unwrap();
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
