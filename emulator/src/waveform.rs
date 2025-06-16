use crate::lcd::{LCD_WIDTH, LCD_BLACK, LCD_WHITE, lcd_window_start, lcd_window_end, lcd_emit, lcd_vline};

pub const CHANNEL_LABEL_WIDTH: u32 = 25;
pub const WAVEFORM_W: u32 = LCD_WIDTH - CHANNEL_LABEL_WIDTH;
pub const WAVEFORM_H: u32 = 18;
pub const WAVEFORM_SPACING: u32 = 26;
pub const WAVEFORMS_Y: u32 = 81;
pub const WAVEFORM_PIN_Y: u32 = 15;
pub const WAVEFORM_W_USIZE: usize = WAVEFORM_W as usize;

pub struct WaveformBuffer
{
	cur: [u16; WAVEFORM_W_USIZE],
	prev: [u16; WAVEFORM_W_USIZE]
}

impl WaveformBuffer
{
	pub fn new() -> Self
	{
		WaveformBuffer
		{
			cur: [0; WAVEFORM_W_USIZE],
			prev: [0; WAVEFORM_W_USIZE]
		}
	}

	pub fn line(&mut self, ch: u32, x0: u32, x1: u32, level: bool)
	{
		let bit = 2 * ch + if level { 0 } else { 1 };
		let mask = 1 << bit;
		for x in x0..=x1
		{
			self.cur[x as usize] |= mask;
		}
	}

	fn blit_hline(&self, y: u32, start: u32, len: u32, bit: u32)
	{
		lcd_window_start(CHANNEL_LABEL_WIDTH + start, y, len, 1);
		for x in start..(start + len)
		{
			let set = self.cur[x as usize] & (1 << bit) != 0;
			lcd_emit(if set { LCD_WHITE } else { LCD_BLACK });
		}

		lcd_window_end();
	}

	fn update_hline(&self, y: u32, bit: u32)
	{
		let mut start = 0;
		let mut len = 0;
		for x in 0..WAVEFORM_W
		{
			let c = (self.cur[x as usize] >> bit) & 1;
			let p = (self.prev[x as usize] >> bit) & 1;
			if c == p
			{
				if len != 0
				{
					self.blit_hline(y, start, len, bit);
				}

				start = 0;
				len = 0;
			}
			else
			{
				if len == 0
				{
					start = x;
					len = 1;
				}
				else
				{
					len += 1;
				}
			}
		}

		if len != 0
		{
			self.blit_hline(y, start, len, bit);
		}
	}

	fn update_vline(&self, y: u32, bit: u32)
	{
		for x in 0..WAVEFORM_W
		{
			let c = (self.cur[x as usize] >> bit) & 3 == 3;
			let p = (self.prev[x as usize] >> bit) & 3 == 3;
			if c != p
			{
				lcd_vline(CHANNEL_LABEL_WIDTH + x, y + 1,
					WAVEFORM_H - 1, if c { LCD_WHITE } else { LCD_BLACK });
			}
		}
	}

	pub fn redraw_vline(&self, x: u32)
	{
		lcd_window_start(CHANNEL_LABEL_WIDTH + x, WAVEFORMS_Y,
			1, 8 * WAVEFORM_SPACING);

		for i in 0..8
		{
			let bits = (self.prev[x as usize] >> (2 * i)) & 3;
			match bits
			{
				1 => {
					lcd_emit(LCD_WHITE);
					for _j in 0..WAVEFORM_H { lcd_emit(LCD_BLACK); }
				}
				2 => {
					for _j in 0..WAVEFORM_H { lcd_emit(LCD_BLACK); }
					lcd_emit(LCD_WHITE);
				}
				3 => {
					for _j in 0..=WAVEFORM_H { lcd_emit(LCD_WHITE); }
				}
				_ => {
					for _j in 0..=WAVEFORM_H { lcd_emit(LCD_BLACK); }
				}
			}

			for _j in 0..(WAVEFORM_SPACING - WAVEFORM_H - 1)
			{
				lcd_emit(LCD_BLACK);
			}
		}

		lcd_window_end();
	}

	fn update_one(&self, y: u32, n: u32)
	{
		self.update_hline(y, 2 * n);
		self.update_vline(y, 2 * n);
		self.update_hline(y + WAVEFORM_H, 2 * n + 1);
	}

	pub fn update(&mut self)
	{
		let mut y = WAVEFORMS_Y;
		for i in 0..8
		{
			self.update_one(y, i);
			y += WAVEFORM_SPACING;
		}

		self.prev = self.cur;
		self.cur = [0; WAVEFORM_W_USIZE];
	}

	pub fn undraw(&mut self)
	{
		// Crazy hack, aka. the simplest solution is the best solution
		self.update();
	}
}
