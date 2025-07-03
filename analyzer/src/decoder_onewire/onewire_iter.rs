use crate::decoder_onewire::onewire_error::OneWireError;
use crate::decoder_onewire::timings::Timings;
use crate::sample::{Edge, EdgeWiseIterator};

pub struct OnewireIter<'a> {
	iter: EdgeWiseIterator<'a>,
	last_idx: usize,
	timing: Timings<u32>,
}

impl <'a>OnewireIter<'a> {

	fn check_bit_timings(&self, duration: u32) -> Result<(), OneWireError> {
		match duration {
			d if d < self.timing.wr_init.min => Err(OneWireError::BitInitTooShort),
			d if d > self.timing.wr_slot.max && d >= self.timing.reset.min => {
				Err(OneWireError::UnexpectedReset)
			}
			d if d > self.timing.wr_slot.max && d < self.timing.reset.min => {
				Err(OneWireError::BitSlotTooLong)
			}
			d if d > self.timing.wr_init.max && d < self.timing.wr_slot.min => {
				Err(OneWireError::BitInitTooLong)
			}
			_ => Ok(()),
		}
	}

	pub fn next_bit(&mut self) -> Option<(u32, Result<bool, OneWireError>)> {
		self.last_idx = self.iter.current_index();

		let start_time = self.current_time();
		let init = self.iter.next()?;
		let mut end_time = self.current_time();

		match init {
			Edge::Falling => return Some((end_time, Err(OneWireError::UnexpectedFallingEdge))),
			_ => {}
		}

		let mut duration = end_time - start_time;

		if let Err(err) = self.check_bit_timings(duration) {
			return Some((end_time, Err(err)));
		}

		self.iter.next()?;

		// bit is high
		if duration <= self.timing.wr_init.max {
			// we don't want to stretch the time if the line is idle after bit
			duration = (self.current_time() - start_time).min(self.timing.wr_slot.max);
			end_time = start_time + duration;

			if duration < self.timing.wr_slot.min + self.timing.line_recover_min {
				return Some((end_time, Err(OneWireError::BitSlotTooShort)));
			}

			return Some((end_time, Ok(true)));
		}
		// bit was low
		let recovery_duration = self.current_time() - end_time;

		if recovery_duration < self.timing.line_recover_min {
 			return Some((self.current_time(), Err(OneWireError::LineRecoveryTooShort)));
		}

		let end_duration = recovery_duration.min(self.timing.wr_slot.max - self.timing.wr_init.max);
		Some((end_time + end_duration, Ok(false)))
	}

	pub fn next_reset(&mut self) -> Option<Result<(), OneWireError>> {
		self.last_idx = self.iter.current_index();

		let start_time = self.current_time();
		let reset = self.iter.next()?;
		let end_time = self.current_time();
		match reset {
			Edge::Falling => return Some(Err(OneWireError::UnexpectedFallingEdge)),
			_ => {}
		}

		let duration = end_time - start_time;

		if duration < self.timing.reset.min {
			return Some(Err(OneWireError::ResetTooShort));
		}

		if duration > self.timing.reset.max {
			return Some(Err(OneWireError::ResetTooLong))
		}

		Some(Ok(()))
	}

	pub fn next_response(&mut self) -> Option<(u32, u32, Result<bool, OneWireError>)> {
		self.last_idx = self.iter.current_index();

		let mut start_time = self.current_time();
		let reset = self.iter.next()?;
		let mut end_time = self.current_time();
		match reset {
			Edge::Rising => return Some((start_time, end_time, Err(OneWireError::UnexpectedRisingEdge))),
			_ => {}
		}

		let mut duration = end_time - start_time;

		end_time = start_time + self.timing.response.max;
		let mut device_responded = false;

		if duration < self.timing.response.max {
			let response_start = self.current_time();
			self.iter.next()?;

			end_time = self.current_time();
			duration = end_time - start_time;

			if duration < self.timing.response.min {
				return Some((start_time, end_time, Err(OneWireError::ResponseTooShort)));
			}

			if duration > self.timing.reset_recover_min {
				return Some((start_time, end_time, Err(OneWireError::ResponseTooLong)));
			}

			start_time = response_start;
			device_responded = true;
			self.iter.next()?;
		}

		Some((start_time, end_time, Ok(device_responded)))
	}

	pub fn next_reset_recovery(&mut self,  response_start: u32) -> Option<Result<u32, OneWireError>> {
		if (self.current_time() - response_start) < self.timing.reset_recover_min {
			return Some(Err(OneWireError::ResetRecoveryTooShort))
		}

		Some(Ok(response_start + self.timing.reset_recover_min))
	}

	// forwards self, so self.next_reset() will be a 'valid' reset
	pub fn forward_to_reset(&mut self) -> Option<()> {
		self.last_idx = self.iter.current_index();

		loop {
			let idx = self.iter.current_index();
			let start = self.current_time();
			let next = self.iter.next()?;

			if (self.current_time() - start) >= self.timing.reset.min && next == Edge::Rising {
				self.iter.set_index(idx).unwrap();
				return Some(())
			}
		}
	}

	pub fn discard_last(&mut self) {
		self.iter.set_index(self.last_idx).unwrap();
	}

	pub fn current_time(&self) -> u32 {
		self.iter.current_time()
	}

	pub fn set_timing(&mut self, timing: Timings<u32>) {
		self.timing = timing;
	}
}

impl <'a>From<EdgeWiseIterator<'a>> for OnewireIter<'a> {
	fn from(iter: EdgeWiseIterator<'a>) -> Self {
		OnewireIter {
			last_idx: iter.current_index(),
			timing: Timings::standard(),
			iter,
		}
	}
}