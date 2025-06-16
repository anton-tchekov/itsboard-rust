use crate::lcd::{LCD_WHITE, LCD_BLACK, lcd_vline, lcd_hline};

pub struct TimeLine
{
	x: u32,
	y: u32,
	w: u32,
	h: u32,
	x0: u32,
	x1: u32,
	pub visible: bool
}

impl TimeLine
{
	pub fn new(x: u32, y: u32, w: u32, h: u32) -> TimeLine
	{
		TimeLine
		{
			x,
			y,
			w,
			h,
			x0: u32::MAX,
			x1: u32::MAX,
			visible: false
		}
	}

	fn tline_x(&self, t: u32, max: u32, def: u32) -> u32
	{
		if max == 0
		{
			return def;
		}

		let v = ((t as f64) / (max as f64) * ((self.w as f64) - 1.0)) as u32;
		u32::min(v, self.w - 1)
	}

	fn tline(&self, x: u32, color: u16)
	{
		if x >= self.w { return; }

		lcd_vline(self.x + x, self.y - self.h, self.h, color);
		lcd_vline(self.x + x, self.y + 1, self.h, color);
	}

	pub fn hide(&mut self)
	{
		self.visible = false;

		lcd_hline(self.x, self.y, self.w, LCD_BLACK);
		self.tline(self.x0, LCD_BLACK);
		if self.x1 != self.x0 { self.tline(self.x1, LCD_BLACK); }
		self.x0 = u32::MAX;
		self.x1 = u32::MAX;
	}

	pub fn show(&mut self, start: u32, end: u32, max: u32)
	{
		if !self.visible
		{
			self.visible = true;
			lcd_hline(self.x, self.y, self.w, LCD_WHITE);
		}

		/* Timeline */
		let x0 = self.tline_x(start, max, 0);
		let x1 = self.tline_x(end, max, self.w - 1);

		/* Undraw x0, only if != to any of new positons */
		if self.x0 != x0 && self.x0 != x1 { self.tline(self.x0, LCD_BLACK); }

		/* Undraw x1, only if != to any of new positons AND was not already undrawn */
		if self.x1 != x0 && self.x1 != x1 && self.x1 != self.x0 { self.tline(self.x1, LCD_BLACK); }

		/* Draw x0, only if != to any of prev positions */
		if x0 != self.x0 && x0 != self.x1 { self.tline(x0, LCD_WHITE); }

		/* Draw x0, only if != to any of prev positions AND was not already drawn */
		if x1 != self.x0 && x1 != self.x1 && x1 != x0 { self.tline(x1, LCD_WHITE); }

		self.x0 = x0;
		self.x1 = x1;
	}
}
