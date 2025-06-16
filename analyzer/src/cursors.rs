use crate::durationindicator::DurationIndicator;
use crate::font::{lcd_str, lcd_str_undraw};
use crate::lcd::{lcd_color, lcd_vline, LCD_WHITE, LCD_BLACK, LCD_WIDTH};
use crate::tinyfont::TINYFONT;
use crate::gui::Action;
use crate::macro_utils;
use crate::waveform::{WaveformBuffer, WAVEFORM_SPACING, CHANNEL_LABEL_WIDTH, WAVEFORMS_Y, WAVEFORM_W};
use crate::sample::SampleBuffer;

const H: u32 = 8 * WAVEFORM_SPACING;
const COLOR_DEF: u16 = lcd_color(192, 192, 192);
const COLOR_SEL: u16 = lcd_color(0, 128, 255);

const LARGE_MOVE: u32 = 10;

pub struct Cursors
{
	durationindicator: DurationIndicator,
	x: [u32; 2],
	ts: [u32; 2],
	pub en: bool,
	t_start: u32,
	t_end: u32
}

impl Cursors
{
	pub fn new() -> Cursors
	{
		Cursors
		{
			durationindicator: DurationIndicator::new(),
			x: [ 0, 0 ],
			ts: [ 0, 0 ],
			en: false,
			t_start: 0,
			t_end: 0
		}
	}

	fn tline(&self, x: u32, color: u16)
	{
		lcd_vline(CHANNEL_LABEL_WIDTH + x, WAVEFORMS_Y, H, color);
	}

	fn render_duration(&mut self)
	{
		let x0 = u32::min(self.x[0], self.x[1]);
		let x1 = u32::max(self.x[0], self.x[1]);

		let t = self.t_end - self.t_start;

		let t0 = (x0 as f64) / (WAVEFORM_W as f64) * (t as f64);
		let t1 = (x1 as f64) / (WAVEFORM_W as f64) * (t as f64);

		let dt = t1 - t0;

		self.durationindicator.show(dt as u32);
	}

	fn render(&mut self, x: [u32; 2], wf: &WaveformBuffer)
	{
		if self.x[0] == self.x[1]
		{
			/* If prev eq */
			if x[0] != x[1]
			{
				/* If no longer eq: Redraw both */
				self.tline(x[0], COLOR_SEL);
				self.tline(x[1], COLOR_DEF);
			}
		}
		else
		{
			/* If prev not eq */
			if x[0] == x[1]
			{
				/* If now eq: Undraw prev x0, Blue new x0 */
				wf.redraw_vline(self.x[0]);
				self.tline(x[0], COLOR_SEL);
			}
			else
			{
				/* If still not eq */
				if x[0] == self.x[1] && x[1] == self.x[0]
				{
					/* If swap */
					self.tline(x[0], COLOR_SEL);
					self.tline(x[1], COLOR_DEF);
				}
				else if x[0] != self.x[0]
				{
					/* If moved */
					self.tline(x[0], COLOR_SEL);
					wf.redraw_vline(self.x[0]);
				}
			}
		}

		self.x = x;
		self.render_duration();
	}

	fn get_t(&self) -> u32
	{
		if self.ts[0] != u32::MAX
		{
			return self.ts[0];
		}

		let x0 = self.x[0];
		let t = self.t_end - self.t_start;
		let t0 = (x0 as f64) / (WAVEFORM_W as f64) * (t as f64);
		(t0 as u32) + self.t_start
	}

	fn to_x(&self, t: u32) -> u32
	{
		if t <= self.t_start
		{
			return 0;
		}

		if t >= self.t_end
		{
			return WAVEFORM_W - 1;
		}

		let t_off = (t - self.t_start) as f64;
		let t_whole = self.t_end - self.t_start;

		if t_whole == 0
		{
			return 0;
		}

		((t_off / (t_whole as f64)) * ((WAVEFORM_W - 1) as f64)) as u32
	}

	pub fn action(&mut self, action: Action, wf: &WaveformBuffer, buf: &SampleBuffer)
	{
		let mut new_x: [u32; 2] = self.x;
		match action
		{
			Action::PrevEdge =>
			{
				let t = self.get_t();
				let idx = buf.find_prev(t);
				let nt = buf.timestamps[idx];
				self.ts[0] = nt;
				new_x[0] = self.to_x(nt);
			}
			Action::NextEdge =>
			{
				let t = self.get_t();
				let idx = buf.find_next(t);
				let nt = buf.timestamps[idx];
				self.ts[0] = nt;
				new_x[0] = self.to_x(nt);
			}
			Action::LeftFast =>
			{
				limit_dec_by!(new_x[0], LARGE_MOVE);
				self.ts[0] = u32::MAX;
			}
			Action::RightFast =>
			{
				limit_inc_by!(new_x[0], LARGE_MOVE, WAVEFORM_W - 1);
				self.ts[0] = u32::MAX;
			}
			Action::Left =>
			{
				limit_dec_by!(new_x[0], 1);
				self.ts[0] = u32::MAX;
			}
			Action::Right =>
			{
				limit_inc_by!(new_x[0], 1, WAVEFORM_W - 1);
				self.ts[0] = u32::MAX;
			}
			Action::Escape =>
			{
				self.hide(wf);
				return;
			}
			Action::Cycle =>
			{
				swap!(new_x[0], new_x[1]);
				swap!(self.ts[0], self.ts[1]);
			}
			_ => {}
		};

		self.render(new_x, wf);
	}

	fn hide(&mut self, wf: &WaveformBuffer)
	{
		self.en = false;
		self.durationindicator.hide();

		wf.redraw_vline(self.x[0]);
		if self.x[0] != self.x[1]
		{
			wf.redraw_vline(self.x[1]);
		}
	}

	pub fn show(&mut self, t_start: u32, t_end: u32)
	{
		if !self.en
		{
			self.en = true;
		}

		self.t_start = t_start;
		self.t_end = t_end;

		self.ts[0] = u32::MAX;
		self.ts[1] = u32::MAX;

		self.x[0] = WAVEFORM_W / 4;
		self.x[1] = WAVEFORM_W / 4 * 3;

		self.tline(self.x[0], COLOR_SEL);
		self.tline(self.x[1], COLOR_DEF);

		self.render_duration();
	}
}
