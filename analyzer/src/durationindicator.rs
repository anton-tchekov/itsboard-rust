use crate::timeindicator::TimeIndicator;
use crate::font::{lcd_str, lcd_str_undraw};
use crate::lcd::{LCD_WHITE, LCD_BLACK, LCD_HEIGHT};
use crate::terminus16::TERMINUS16;

pub struct DurationIndicator
{
	timeindicator: TimeIndicator
}

const X: u32 = 105;
const Y: u32 = LCD_HEIGHT - 30 + 7;
const LABEL: &'static str = "ΔT:";

impl DurationIndicator
{
	pub fn new() -> DurationIndicator
	{
		DurationIndicator
		{
			timeindicator: TimeIndicator::new(X + 30, Y)
		}
	}

	pub fn hide(&mut self)
	{
		self.timeindicator.hide();
		lcd_str_undraw(X, Y, LABEL.chars().count(), &TERMINUS16);
	}

	pub fn show(&mut self, t: u32)
	{
		if !self.timeindicator.visible
		{
			lcd_str(X, Y, LABEL, LCD_WHITE, LCD_BLACK, &TERMINUS16);
		}

		self.timeindicator.show(t);
	}
}
