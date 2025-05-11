use crate::tinyfont::TINYFONT;
use crate::hw;
use crate::font::{lcd_str_undraw, lcd_char, lcd_str};
use crate::lcd::{LCD_WHITE, LCD_BLACK, lcd_vline, lcd_hline};
use crate::bytewriter::ByteMutWriter;
use core::fmt::Write;

const LABELS: [&str; 4] = [ "s", "ms", "µs", "ns" ];

const PI_T_X: u32 = 160;
const PI_T_Y: u32 = 14;
const PI_T_W: u32 = 120;
const PI_T_H: u32 = 5;

const PI_D_X: u32 = 290;
const PI_D_Y: u32 = 6;
const PI_L_Y: u32 = 14;
const NUM_DIGITS: usize = 12;

pub struct PositionIndicator
{
	x0: u32,
	x1: u32,
	visible: bool,
	digits: [u8; NUM_DIGITS]
}

impl PositionIndicator
{
	pub fn new() -> PositionIndicator
	{
		PositionIndicator
		{
			x0: u32::MAX,
			x1: u32::MAX,
			visible: false,
			digits: [0; NUM_DIGITS]
		}
	}

	fn label_x(len: usize) -> u32
	{
		(3 - (len as u32)) * (TINYFONT.width + 1)
	}

	fn tline_x(t: u32, max: u32) -> u32
	{
		if max == 0
		{
			return 0;
		}

		let v = ((t as f64) / (max as f64) * ((PI_T_W as f64) - 1.0)) as u32;
		u32::min(v, PI_T_W - 1)
	}

	fn tline(x: u32, color: u16)
	{
		if x >= PI_T_W { return; }

		lcd_vline(PI_T_X + x, PI_T_Y - PI_T_H, PI_T_H, color);
		lcd_vline(PI_T_X + x, PI_T_Y + 1, PI_T_H, color);
	}

	pub fn hide(&mut self)
	{
		self.visible = false;
		let mut x = PI_D_X;
		for label in LABELS
		{
			let len = label.chars().count();
			lcd_str_undraw(x + Self::label_x(len), PI_L_Y, len, &TINYFONT);
			x += (TINYFONT.width + 1) * 4;
		}

		for i in 0..self.digits.len()
		{
			self.digits[i] = 0;
		}

		lcd_str_undraw(PI_D_X, PI_D_Y, NUM_DIGITS + 3, &TINYFONT);

		/* Undraw timeline */
		lcd_hline(PI_T_X, PI_T_Y, PI_T_W, LCD_BLACK);
		Self::tline(self.x0, LCD_BLACK);
		if self.x1 != self.x0 { Self::tline(self.x1, LCD_BLACK); }
		self.x0 = u32::MAX;
		self.x1 = u32::MAX;
	}

	pub fn show(&mut self, start: u32, end: u32, max: u32)
	{
		if !self.visible
		{
			self.visible = true;
			let mut x = PI_D_X;
			for (i, label) in LABELS.iter().enumerate()
			{
				let len = label.chars().count();
				lcd_str(x + Self::label_x(len), PI_L_Y, label,
					LCD_WHITE, LCD_BLACK, &TINYFONT);

				if i < 3
				{
					lcd_char(x + (TINYFONT.width + 1) * 3, PI_D_Y, '.' as u32,
						LCD_WHITE, LCD_BLACK, &TINYFONT);
				}

				x += (TINYFONT.width + 1) * 4;
			}

			lcd_hline(PI_T_X, PI_T_Y, PI_T_W, LCD_WHITE);
		}

		let mut digits: [u8; NUM_DIGITS] = [0x30; NUM_DIGITS];
		let mut buf = ByteMutWriter::new(&mut digits);
		let ns = (start as u64) * 1000 / hw::TICKS_PER_US as u64;
		write!(buf, "{:0>12}", ns).unwrap();

		let mut x = PI_D_X;
		for i in 0..digits.len()
		{
			let nd = digits[i];
			if self.digits[i] != nd
			{
				self.digits[i] = nd;
				lcd_char(x, PI_D_Y, nd.into(),
					LCD_WHITE, LCD_BLACK, &TINYFONT);
			}

			/* Gap for dot */
			if i % 3 == 2
			{
				x += TINYFONT.width + 1;
			}

			x += TINYFONT.width + 1;
		}

		/* Timeline */
		let x0 = Self::tline_x(start, max);
		let x1 = Self::tline_x(end, max);

		/* Undraw x0, only if != to any of new positons */
		if self.x0 != x0 && self.x0 != x1 { Self::tline(self.x0, LCD_BLACK); }

		/* Undraw x1, only if != to any of new positons AND was not already undrawn */
		if self.x1 != x0 && self.x1 != x1 && self.x1 != self.x0 { Self::tline(self.x1, LCD_BLACK); }

		/* Draw x0, only if != to any of prev positions */
		if x0 != self.x0 && x0 != self.x1 { Self::tline(x0, LCD_WHITE); }

		/* Draw x0, only if != to any of prev positions AND was not already drawn */
		if x1 != self.x0 && x1 != self.x1 && x1 != x0 { Self::tline(x1, LCD_WHITE); }

		self.x0 = x0;
		self.x1 = x1;
	}
}
