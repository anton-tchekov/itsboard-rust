use core::ops::{Add, Mul};

use crate::decoder::TIMER_TICKS_PER_US;

// TODO: use crate instead
fn ceilf(x: f32) -> u32 {
    let xi = x as u32;
    if x > xi as f32 {
        xi + 1
    } else {
        xi
    }
}

#[derive(Copy, Clone)]
pub struct Range<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy
{
    pub min: T,
    pub max: T,
}

impl<T> Range<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy
{
    fn scale(&self, factor: T) -> Self
    where
    {
        Self {
            min: self.min * factor,
            max: self.max * factor,
        }
    }
}

impl Range<f32> {
	fn as_u32(&self) -> Range<u32> {
		Range {
			min: self.min as u32,
			max: ceilf(self.max) as u32,
		}
	}
}


// TODO: link to source
#[derive(Copy, Clone)]
pub struct Timings<T> 
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
	// Low time before read/write
	pub wr_init: Range<T>,
	// Low time total
	pub wr_slot: Range<T>,
	// Line recovery time after read/write
	pub line_recover_min: T,
	// Reset time
	pub reset: Range<T>,
	// Reset response sample time
	pub response: Range<T>,
	// Reset recovery time
	pub reset_recover_min: T,
}

impl <T>Timings<T>
where 
	T: Add<Output = T> + Mul<Output = T> + Copy,
{
	fn scale(&self, factor: T) -> Self {
		Self {
			wr_init: self.wr_init.scale(factor),
			wr_slot: self.wr_slot.scale(factor),
			line_recover_min: self.line_recover_min * factor,
			reset: self.reset.scale(factor),
			response: self.response.scale(factor),
			reset_recover_min: self.reset_recover_min * factor,
		}
	}
}

impl Timings<u32> {
	pub fn standard() -> Self {
		Timings {
			wr_init: Range { min: 5, max: 15 },
			wr_slot: Range { min: 52, max: 120 },
			line_recover_min: 8,
			response: Range { min: 63, max: 78 },
			reset: Range { min: 480, max: 640 },
			reset_recover_min: 473,
		}
		.scale(TIMER_TICKS_PER_US)
	}
	
	pub fn overdrive() -> Self {
		Timings {
			wr_init: Range { min: 1.0, max: 1.85},
			wr_slot: Range { min: 7.0, max: 14.0 },
			line_recover_min: 2.5,
			response: Range { min: 7.2, max: 8.8 },
			reset: Range { min: 68.0, max: 80.0 },
			reset_recover_min: 46.7,
		}
		.scale(TIMER_TICKS_PER_US as f32)
		.as_u32()
	}
}

impl Timings<f32> {
	fn as_u32(&self) -> Timings<u32> {
		Timings {
			wr_init: self.wr_init.as_u32(),
			wr_slot: self.wr_slot.as_u32(),
			line_recover_min: self.line_recover_min as u32,
			reset: self.reset.as_u32(),
			response: self.response.as_u32(),
			reset_recover_min: self.reset_recover_min as u32,
		}
	}
}