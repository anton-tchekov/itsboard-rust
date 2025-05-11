/* Specifically made to be at most 16 Pixels tall */

use crate::{font::Font, lcd::{lcd_emit, lcd_rect, lcd_window_end, lcd_window_start, LCD_BLACK, LCD_GREEN}, terminus16::TERMINUS16, tinyfont::TINYFONT};

pub struct DecoderFrameBuffer<const LEN: usize>
{
	pub font: Font,
	pub fg_color: u16,
	pub bg_color: u16,
	pub height: usize,

	pub last_drawn_buf: [u16; LEN],
	pub buf: [u16; LEN],
}

impl<const LEN: usize> DecoderFrameBuffer<LEN>
{
	pub fn default() -> Self
	{
		DecoderFrameBuffer
		{
			font: TINYFONT,
			fg_color: LCD_BLACK,
			bg_color: LCD_GREEN,
			height: 16,

			last_drawn_buf: [0; LEN],
			buf: [0; LEN],
		}
	}

	pub fn new(font: Font, fg_color: u16, bg_color: u16, height: usize) -> Self
	{
		DecoderFrameBuffer
		{
			font: font,
			fg_color: fg_color,
			bg_color: bg_color,
			height: height,

			last_drawn_buf: [0; LEN],
			buf: [0; LEN],
		}
	}

	pub fn set_fg_color(&mut self, color: u16)
	{
		self.fg_color = color;
	}

	pub fn set_bg_color(&mut self, color: u16)
	{
		self.bg_color = color;
	}

	pub fn set_font(&mut self, font: Font)
	{
		self.font = font;
	}

	fn height_bitmask(y: u32, h: u32) -> u16 
	{
		let mut result: u16 = 0;

		for i in 0..y+h
		{
			if i > y && i < (y+h)
			{
				result |= 1 << i;
			}

			if i >= 15
			{
				break;
			}
		}

		result
	}

	pub fn clear(&mut self)
	{
		for i in 0..LEN
		{
			self.buf[i] = 0;
		}
	}

	pub fn add_rect(&mut self, x: u32, y: u32, w: u32, h: u32)
	{
		let height_mask = Self::height_bitmask(y, h);

		for i in x..x+w
		{
			if i >= LEN as u32
			{
				return;
			}

			self.buf[i as usize] = 0xFFFF & height_mask;
		}
	}

	pub fn add_char(&mut self, x: u32, y: u32, c: char)
	{
		if x >= LEN as u32 - self.font.width
		{
			return;
		}

		const CHAR_OFFSET: usize = 32;
		let height_mask = Self::height_bitmask(y, self.font.height);
		let char_index = (c as usize - CHAR_OFFSET) * self.font.width as usize;
		
		for j in 0..self.font.width as usize
		{
			let byte = self.font.bitmap[char_index + j];

			self.buf[x as usize + j] |= height_mask;
			self.buf[x as usize + j] &= !((byte as u16) << y);
		}
	}

	pub fn add_text(&mut self, x: u32, y: u32, s: &str)
	{
		for (i, c) in s.chars().enumerate()
		{
			self.add_char(x + ((self.font.width+1)*i as u32), y, c);
		}
	}

	pub fn draw_vline(&mut self, idx: usize, x: u32, y: u32)
	{
		let height = self.height;
		let vline = self.buf[idx];
		let last_vline = self.last_drawn_buf[idx];

		//if vline == 0 || vline == last_vline
		//{
		//	return;
		//}

		lcd_window_start(x, y, 1, height as u32);
		for i in 0..height
		{
			if vline & (1 << i) > 0
			{
				lcd_emit(self.fg_color);
			}
			else
			{
				lcd_emit(self.bg_color);
			}
		}
		lcd_window_end();
	}

	pub fn draw_buffer(&mut self, x: u32, y: u32)
	{
		for i in 0..LEN
		{
			self.draw_vline(i, x + i as u32, y);
		}

		self.last_drawn_buf = self.buf;
	}
}