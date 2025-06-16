use crate::tinyfont::TINYFONT;
use crate::hw;
use crate::font::{lcd_str_undraw, lcd_char, lcd_str};
use crate::lcd::{LCD_WHITE, LCD_BLACK};
use crate::bytewriter::ByteMutWriter;
use core::fmt::Write;

const LABELS: [&str; 4] = [ "s", "ms", "µs", "ns" ];

const NUM_DIGITS: usize = 12;
const PI_Y_OFF: u32 = 8;

pub struct TimeIndicator
{
	x: u32,
	y: u32,
	pub visible: bool,
	digits: [u8; NUM_DIGITS]
}

impl TimeIndicator
{
	pub fn new(x: u32, y: u32) -> TimeIndicator
	{
		TimeIndicator
		{
			x,
			y,
			visible: false,
			digits: [0; NUM_DIGITS]
		}
	}

	fn label_x(len: usize) -> u32
	{
		(3 - (len as u32)) * (TINYFONT.width + 1)
	}

	pub fn hide(&mut self)
	{
		self.visible = false;
		let mut x = self.x;
		for label in LABELS
		{
			let len = label.chars().count();
			lcd_str_undraw(x + Self::label_x(len), self.y + PI_Y_OFF, len, &TINYFONT);
			x += (TINYFONT.width + 1) * 4;
		}

		for i in 0..self.digits.len()
		{
			self.digits[i] = 0;
		}

		lcd_str_undraw(self.x, self.y, NUM_DIGITS + 3, &TINYFONT);
	}

	pub fn show(&mut self, start: u32)
	{
		if !self.visible
		{
			self.visible = true;
			let mut x = self.x;
			for (i, label) in LABELS.iter().enumerate()
			{
				let len = label.chars().count();
				lcd_str(x + Self::label_x(len), self.y + PI_Y_OFF, label,
					LCD_WHITE, LCD_BLACK, &TINYFONT);

				if i < 3
				{
					lcd_char(x + (TINYFONT.width + 1) * 3, self.y, '.' as u32,
						LCD_WHITE, LCD_BLACK, &TINYFONT);
				}

				x += (TINYFONT.width + 1) * 4;
			}
		}

		let mut digits: [u8; NUM_DIGITS] = [0x30; NUM_DIGITS];
		let mut buf = ByteMutWriter::new(&mut digits);
		let ns = (start as u64) * 1000 / hw::TICKS_PER_US as u64;
		write!(buf, "{:0>12}", ns).unwrap();

		let mut x = self.x;
		for i in 0..digits.len()
		{
			let nd = digits[i];
			if self.digits[i] != nd
			{
				self.digits[i] = nd;
				lcd_char(x, self.y, nd.into(),
					LCD_WHITE, LCD_BLACK, &TINYFONT);
			}

			/* Gap for dot */
			if i % 3 == 2
			{
				x += TINYFONT.width + 1;
			}

			x += TINYFONT.width + 1;
		}
	}
}
