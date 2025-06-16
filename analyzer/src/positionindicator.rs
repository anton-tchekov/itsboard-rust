use crate::timeindicator::TimeIndicator;
use crate::timeline::TimeLine;

pub struct PositionIndicator
{
	timeindicator: TimeIndicator,
	timeline: TimeLine
}

impl PositionIndicator
{
	pub fn new() -> PositionIndicator
	{
		PositionIndicator
		{
			timeindicator: TimeIndicator::new(290, 6),
			timeline: TimeLine::new(160, 14, 120, 5)
		}
	}

	pub fn hide(&mut self)
	{
		self.timeindicator.hide();
		self.timeline.hide();
	}

	pub fn show(&mut self, start: u32, end: u32, max: u32)
	{
		self.timeindicator.show(start);
		self.timeline.show(start, end, max);
	}
}
