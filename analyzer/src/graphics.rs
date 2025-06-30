use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::lcd::*;
use sdl2::render::TextureAccess;
use sdl2::pixels::PixelFormatEnum;

pub struct Graphics
{
	pub canvas: sdl2::render::WindowCanvas,
	pub texture: sdl2::render::Texture,
	pub events: sdl2::EventPump
}

const W: u32 = 480;
const H: u32 = 320;

impl Graphics
{
	pub fn init() -> Result<Graphics, String>
	{
		let sdl_context = sdl2::init()?;
		let video_subsystem = sdl_context.video()?;
		let window = video_subsystem.window("Emulator", W, H)
			.position_centered()
			.build()
			.expect("could not initialize video subsystem");

		let canvas = window.into_canvas().build()
			.expect("could not make a canvas");

		let events = sdl_context.event_pump()?;
		let texture_creator = canvas.texture_creator();
		let texture = texture_creator.create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, W, H).unwrap();

		Ok(Graphics { canvas, texture, events })
	}

	pub fn clear(&mut self)
	{
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	pub fn blit(&mut self)
	{
		let rect = Rect::new(0, 0, W, H);

		{
			// Copy img to framebuffer
			let img = &IMAGE.lock().unwrap().image;
			self.texture.update(rect, &**img, W as usize * 4).unwrap();
		}

		// Blit framebuffer to screen
		self.canvas.copy(&self.texture, rect, rect).unwrap();
	}

	pub fn present(&mut self)
	{
		self.canvas.present();
	}
}
