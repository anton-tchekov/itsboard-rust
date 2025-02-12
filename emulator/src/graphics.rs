use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Graphics {
	pub canvas: sdl2::render::WindowCanvas,
	pub events: sdl2::EventPump
}

impl Graphics {
	pub fn init() -> Result<Graphics, String> {
		let sdl_context = sdl2::init()?;
		let video_subsystem = sdl_context.video()?;
		let window = video_subsystem.window("Emulator", 480, 320)
			.position_centered()
			.build()
			.expect("could not initialize video subsystem");

		let canvas = window.into_canvas().build()
			.expect("could not make a canvas");

		let events = sdl_context.event_pump()?;

		Ok(Graphics { canvas, events })
	}

	pub fn clear(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}
}
