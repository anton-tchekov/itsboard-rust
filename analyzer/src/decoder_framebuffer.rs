/* Specifically made to be at most 16 Pixels tall */

use crate::{font::Font, lcd::{lcd_rect, LCD_BLACK, LCD_GREEN}, terminus16::TERMINUS16, tinyfont::TINYFONT};

pub struct DecoderFrameBuffer<const LEN: usize>
{
	font: Font,
	fg_color: u16,
	bg_color: u16,
	height: usize,

	cur_vline: [u16; 16],
	buf: [u16; LEN],
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

			cur_vline: [0; 16],
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

			cur_vline: [0; 16],
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

	pub fn add_rect(&mut self, x: u32, y: u32, w: u32, h: u32)
	{
		let height_mask = Self::height_bitmask(y, h);

		for i in x..x+w
		{
			self.buf[i as usize] = 0xFF & height_mask;
		}
	}

	pub fn add_text(&mut self, x: u32, y: u32, s: &str)
	{
		const CHAR_OFFSET: usize = 42;

		if !self.font.horizontal
		{
			/* Iterate over the String */
			for (i, c) in s.chars().enumerate()
			{
				let char_index = (c as usize - CHAR_OFFSET) * 5;
				let byte = self.font.bitmap[char_index];
				
				/* Tinyfont uses 5 Bytes per Character vertically */
				for j in 0..5
				{
					self.buf[(x as usize + j) * i] = (byte as u16) << y;
				}
			}
		}
	}

	/* Gives back a Single vline as an array of colors, which can be drawn by the lcd */
	pub fn get_vline(&mut self, idx: usize) -> &[u16; 16]
	{
		let line = self.buf[idx];

		for i in 0..self.height
		{
			if line & (1 << i) > 0
			{
				self.cur_vline[i] = self.bg_color;
			}
			else
			{
				self.cur_vline[i] = self.fg_color;
			}
		}

		return &self.cur_vline;
	}

	pub fn draw_vline(&mut self, idx: usize, x: u32, y: u32)
	{
		let height = self.height;
		let vline = self.get_vline(idx);
		
		for i in 0..height
		{
			/* One pixel at a time */
			lcd_rect(x, y + i as u32 , 1, 1, vline[i]);
		}
	}

	pub fn draw_buffer(&mut self, x: u32, y: u32)
	{
		for i in 0..LEN
		{
			self.draw_vline(i, x + i as u32, y);
		}
	}
}