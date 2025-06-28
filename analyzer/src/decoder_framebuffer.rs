/* Specifically made to be at most 32 Pixels tall */

use crate::{font::{Font, remap_char},
	lcd::{lcd_emit, lcd_window_end, lcd_window_start,
		LCD_BLACK, LCD_WHITE, LCD_GREEN, LCD_RED, LCD_BLUE, LCD_YELLOW, LCD_ORANGE}};

use crate::decoder::{SectionBuffer, SectionContent};
use crate::terminus16_bold::TERMINUS16_BOLD;
use crate::bytewriter::ByteMutWriter;
use crate::waveform::CHANNEL_LABEL_WIDTH;
use core::fmt::Write;
use crate::t_to_x;

const HEIGHT: u32 = 16;

const COLOR_TABLE: [u16; 16] =
[
	LCD_BLACK,
	LCD_WHITE,
	LCD_RED,
	LCD_BLUE,
	LCD_YELLOW,
	LCD_GREEN,
	LCD_ORANGE,
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

pub struct DecoderLine<const LEN: usize>
{
	pub last_colors: [u8; LEN],
	pub colors: [u8; LEN],
	pub last_drawn_buf: [u16; LEN],
	pub buf: [u16; LEN]
}

impl<const LEN: usize> DecoderLine<LEN>
{
	pub fn new() -> Self
	{
		DecoderLine
		{
			last_colors: [0; LEN],
			colors: [0; LEN],
			last_drawn_buf: [0; LEN],
			buf: [0; LEN]
		}
	}

	fn height_bitmask(y: u32, h: u32) -> u16
	{
		(((1 << h) - 1) << y) as u16
	}

	pub fn clear(&mut self)
	{
		self.buf = [0; LEN];
		self.colors = [0; LEN];
	}

	fn set_bg(&mut self, x: u32, w: u32, fg: u32)
	{
		for i in x..x + w
		{
			self.colors[i as usize] &= 0xF0;
			self.colors[i as usize] |= fg as u8;
		}
	}

	fn add_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32)
	{
		let height_mask = Self::height_bitmask(y, h);
		for i in x..x + w
		{
			self.buf[i as usize] |= height_mask;
			self.colors[i as usize] &= 0xF;
			self.colors[i as usize] |= (color << 4) as u8;
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
			self.buf[x as usize + j] &= !((byte as u16) << y);
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
				if (char_bitmap[byte] & bit) != 0
				{
					self.buf[(x + x0) as usize] &= !(1 << (y + y0));
				}

				x0 += 1;
			}

			y0 += 1;
		}

		font.width
	}

	fn add_char(&mut self, x: u32, y: u32, c: u32, font: &Font) -> u32
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

	fn add_text(&mut self, x: u32, y: u32, s: &str, color: u32, font: &Font)
	{
		let mut x0 = x;
		for c in s.chars()
		{
			let w = self.add_char(x0, y, c as u32, font);
			self.set_bg(x0, w, color);
			x0 += w;
		}
	}

	fn draw_vline(&mut self, idx: usize, x: u32, y: u32)
	{
		let vline = self.buf[idx];
		let last_vline = self.last_drawn_buf[idx];
		let color = self.colors[idx];
		let last_color = self.last_colors[idx];
		if vline == last_vline && color == last_color
		{
			return;
		}

		lcd_window_start(x, y, 1, HEIGHT);
		for i in 0..HEIGHT
		{
			lcd_emit(if vline & (1 << i) != 0
				{ COLOR_TABLE[((color >> 4) & 0xF) as usize] }
				else { COLOR_TABLE[(color & 0xF) as usize] });
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
		self.last_colors = self.colors;
	}
}

pub struct DecoderFrameBuffer<const LEN: usize>
{
	lines: [DecoderLine<LEN>; 2]
}

fn format_byte(buf: &mut ByteMutWriter, byte: u64)
{
	if (32..=126).contains(&byte)
	{
		write!(buf, " ${:02X} {}", byte, (byte as u8) as char).unwrap();
	}
	else
	{
		write!(buf, " ${:02X}", byte).unwrap();
	}
}

impl<const LEN: usize> DecoderFrameBuffer<LEN>
{
	pub fn new() -> Self
	{
		DecoderFrameBuffer
		{
			lines: [DecoderLine::new(), DecoderLine::new()]
		}
	}

	fn clear_intenal(&mut self)
	{
		self.lines[0].clear();
		self.lines[1].clear();
	}

	pub fn clear(&mut self)
	{
		self.clear_intenal();
		self.draw();
	}

	pub fn draw(&mut self)
	{
		self.lines[0].draw_buffer(CHANNEL_LABEL_WIDTH, 33);
		self.lines[1].draw_buffer(CHANNEL_LABEL_WIDTH, 50);
	}

	pub fn render(&mut self, sec_buf: &SectionBuffer, t_start: u32, t_end: u32)
	{
		self.clear_intenal();

		/* Draw all Sections which are in our current view */
		let (start, end) = sec_buf.find_view(t_start, t_end);
		for i in start..end
		{
			let cur = sec_buf.sections[i];

			let x0 = t_to_x(cur.start, t_start, t_end);
			let x1 = t_to_x(cur.end, t_start, t_end);
			let w = x1 - x0;

			let mut text: [u8; 64] = [0; 64];
			let mut buf = ByteMutWriter::new(&mut text);

			let mut bg = 2;
			let mut fg = 0;
			let mut line = &mut self.lines[0];

			match cur.content
			{
					SectionContent::TxByte(v) => {
									fg = 1; /* Black */
									bg = 3; /* Blue */
									line = &mut self.lines[1];

									format_byte(&mut buf, v.into());
								},
					SectionContent::RxByte(v) => { format_byte(&mut buf, v.into()) },
					SectionContent::Byte(v) => { format_byte(&mut buf, v.into()) },
					SectionContent::Empty     => write!(buf, " Empty").unwrap(),
					SectionContent::Bit(v)    => {
						line = &mut self.lines[1];
						write!(buf, " {}", if v { 1 } else { 0 }).unwrap()
					},
					SectionContent::StartBit  => {
									fg = 1; /* Black */
									bg = 5; /* Green */
									write!(buf, " Start").unwrap()
					},
					SectionContent::StopBit   => {
									fg = 1; /* Black */
									bg = 6; /* Orange */
									write!(buf, " Stop").unwrap()
					},
					SectionContent::I2cAddress(v) => {
									fg = 1; /* Black */
									bg = 4; /* Yellow */
									write!(buf, " Addr: {:X}", v).unwrap()
					},
					SectionContent::Err(v) => write!(buf, " {}", v).unwrap(),
					SectionContent::ParityBit(v) => write!(buf, " {}", v).unwrap(),
					SectionContent::Data(v) => format_byte(&mut buf, v.into()),
					SectionContent::RepeatedStart => {
									fg = 1; /* Black */
									bg = 5; /* Green */
									write!(buf, " RS").unwrap()
					},
					SectionContent::Ack => {
									fg = 1; /* Black */
									bg = 5; /* Green */
									write!(buf, " ACK").unwrap()
					},
					SectionContent::Nak => {
									fg = 1; /* Black */
									bg = 2; /* Red */
									write!(buf, " NAK").unwrap()
					},
					SectionContent::I2cWrite => {
									fg = 1; /* Black */
									bg = 1; /* White */
									write!(buf, " W").unwrap()
					},
					SectionContent::I2cRead => {
						fg = 1; /* Black */
						bg = 1; /* White */
						write!(buf, " R").unwrap()
					},
					SectionContent::Reset => write!(buf, " Reset").unwrap(),
					SectionContent::ResetResponse(v) => write!(buf, " {}", if v {"Response"} else {"No Response"}).unwrap(),
					SectionContent::ResetRecovery => write!(buf, " Reset Recovery").unwrap(),
					SectionContent::ROMCmd(v) => {
						fg = 1; /* Black */
						bg = 3; /* Blue */

						write!(buf, " {}", v.to_string()).unwrap()
					},
					SectionContent::FamilyCode(v) => {
						fg = 1; /* Black */
						bg = 6; /* Orange */

						write!(buf, " ${:02X}", v).unwrap();
					},
					SectionContent::SensorID(v ) => {
						fg = 1; /* Black */
						bg = 6; /* Orange */

						write!(buf, " ${:02X}", v).unwrap();
					}
					SectionContent::CRC(v) => {
						fg = 1; /* Black */
						bg = 5; /* Green */

						write!(buf, " ${:02X}", v).unwrap();
					}
					SectionContent::FunctionCmd(v) => {
						fg = 1; /* Black */
						bg = 3; /* Blue */

						write!(buf, " ${:02X}", v).unwrap();
					}
				};

			let font = &TERMINUS16_BOLD;
			let font_width = font.width + 1;
			let font_height = font.height;

			if w < (buf.as_str().len() as u32 * font_width)
			{
				line.add_rect(x0, 0, w, font_height, bg);
			}
			else
			{
				line.add_rect(x0, 0, w, font_height, bg);
				line.add_text(x0, 0, buf.as_str(), fg, font);
			}
		}

		self.draw();
	}
}
