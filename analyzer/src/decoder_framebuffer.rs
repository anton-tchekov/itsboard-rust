/* Specifically made to be at most 32 Pixels tall */

use crate::{font::{Font, remap_char},
	lcd::{lcd_emit, lcd_window_end, lcd_window_start,
		LCD_BLACK, LCD_WHITE, LCD_GREEN, LCD_RED, LCD_BLUE, LCD_YELLOW}};

use crate::decoder::{SectionBuffer, SectionContent};
use crate::terminus16_bold::TERMINUS16_BOLD;
use crate::bytewriter::ByteMutWriter;
use crate::waveform::CHANNEL_LABEL_WIDTH;
use core::fmt::Write;
use crate::t_to_x;


const COLOR_TABLE: [u16; 16] =
[
	LCD_BLACK,
	LCD_WHITE,
	LCD_RED,
	LCD_BLUE,
	LCD_YELLOW,
	LCD_GREEN,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0
];

pub struct DecoderFrameBuffer<const LEN: usize>
{
	pub fg_color: u16,
	pub bg_color: u16,
	pub height: usize,
	pub last_colors: [u16; LEN],
	pub colors: [u16; LEN],
	pub last_drawn_buf: [u32; LEN],
	pub buf: [u32; LEN]
}

impl<const LEN: usize> DecoderFrameBuffer<LEN>
{
	pub fn default() -> Self
	{
		DecoderFrameBuffer::new(LCD_GREEN, LCD_BLACK, 32)
	}

	pub fn new(fg_color: u16, bg_color: u16, height: usize) -> Self
	{
		DecoderFrameBuffer
		{
			fg_color: fg_color,
			bg_color: bg_color,
			height: height,
			last_colors: [0; LEN],
			colors: [0; LEN],
			last_drawn_buf: [0; LEN],
			buf: [0; LEN]
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

	fn height_bitmask(y: u32, h: u32) -> u32
	{
		((1 << h) - 1) << y
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
			self.buf[i as usize] |= height_mask;
		}
	}

	fn add_char_v(&mut self, x: u32, y: u32, c: u32, font: &Font) -> u32
	{
		const CHAR_OFFSET: usize = 32;
		let height_mask = Self::height_bitmask(y, font.height);
		let char_index = (c as usize - CHAR_OFFSET) * font.width as usize;

		for j in 0..font.width as usize
		{
			let byte = font.bitmap[char_index + j];

			self.buf[x as usize + j] |= height_mask;
			self.buf[x as usize + j] &= !((byte as u32) << y);
		}

		font.width + 1
	}

	fn add_char_h(&mut self, x: u32, y: u32, c: u32, font: &Font) -> u32
	{
		let stride = (font.width + 7) >> 3;
		let offset = (c - 32) * font.height * stride;
		let cs = offset as usize;
		let char_bitmap = &font.bitmap[cs..font.bitmap.len()];

		let mut y0 = 0;
		while y0 < font.height
		{
			let mut x0 = 0;
			while x0 < font.width
			{
				let byte = ((y0 * stride) + (x0 >> 3)) as usize;
				let bit = 1 << (7 - (x0 & 0x7));
				if (char_bitmap[byte] & bit) == 0
				{
					self.buf[(x + x0) as usize] |= 1 << (y + y0);
				}
				else
				{
					self.buf[(x + x0) as usize] &= !(1 << (y + y0));
				}

				x0 += 1;
			}

			y0 += 1;
		}

		font.width
	}

	pub fn add_char(&mut self, x: u32, y: u32, c: u32, font: &Font) -> u32
	{
		let c = remap_char(c);
		if font.horizontal
		{
			self.add_char_h(x, y, c, font)
		}
		else
		{
			self.add_char_v(x, y, c, font)
		}
	}

	pub fn add_text(&mut self, x: u32, y: u32, s: &str, font: &Font)
	{
		let mut x0 = x;
		for c in s.chars()
		{
			x0 += self.add_char(x0, y, c as u32, font);
		}
	}

	pub fn draw_vline(&mut self, idx: usize, x: u32, y: u32)
	{
		let height = self.height;
		let vline = self.buf[idx];
		let last_vline = self.last_drawn_buf[idx];
		if vline == last_vline
		{
			return;
		}

		lcd_window_start(x, y, 1, height as u32);
		for i in 0..height
		{
			lcd_emit(if vline & (1 << i) != 0
				{ self.fg_color } else { self.bg_color });
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

	pub fn render(&mut self, sec_buf: &SectionBuffer, t_start: u32, t_end: u32)
	{
		self.clear();
		let (start, end) = sec_buf.find_view(t_start, t_end);

		/* Draw all Sections which are in our current view */
		for i in start..end
		{
			let cur = sec_buf.sections[i];

			let x0 = t_to_x(cur.start, t_start, t_end);
			let x1 = t_to_x(cur.end, t_start, t_end);
			let w = x1 - x0;

			let mut text: [u8; 64] = [0; 64];
			let mut buf = ByteMutWriter::new(&mut text);

			/* TODO: In decoder.rs auslagern */
			match cur.content
			{
				SectionContent::Empty    => write!(buf, " Empty").unwrap(),
				SectionContent::Byte(v)  => write!(buf, " 0x{:X}", v).unwrap(),
				SectionContent::Bit(v)   => write!(buf, " {}", v).unwrap(),
				SectionContent::StartBit => write!(buf, " Start").unwrap(),
				SectionContent::StopBit  => write!(buf, " Stop").unwrap(),
				SectionContent::I2cAddress(v) => write!(buf, " Addr: {:X}", v).unwrap(),
			};

			let font = &TERMINUS16_BOLD;
			let font_width = font.width + 1;
			let font_height = font.height;

			if w < (buf.as_str().len() as u32 * font_width)
			{
				self.add_rect(x0, 0, w, font_height);
			}
			else
			{
				self.add_rect(x0, 0, w, font_height);
				self.add_text(x0, 0, buf.as_str(), font);
			}
		}

		self.draw_buffer(CHANNEL_LABEL_WIDTH, 32);
	}
}
