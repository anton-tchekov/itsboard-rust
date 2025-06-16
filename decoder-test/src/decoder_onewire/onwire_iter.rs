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

		// bit is high
		if duration <= self.timing.wr_init.max {
			self.iter.next()?;
			// we don't want to stretch the time if the line is idle after bit
			duration += (self.current_time() - start_time).min(self.timing.wr_slot.max);
			end_time = start_time + duration;

			if duration < self.timing.wr_slot.min + self.timing.line_recover_min {
				return Some((end_time, Err(OneWireError::BitInitTooShort)));
			}

			return Some((end_time, Ok(true)));
		}

		// bit was low
		self.iter.next()?;
		let recovery_duration = self.current_time() - end_time;

		if recovery_duration < self.timing.line_recover_min {
 			return Some((self.current_time(), Err(OneWireError::LineRecoveryTooShort)));
		}

		Some((end_time + self.timing.line_recover_min, Ok(false)))
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

    pub fn next_response(&mut self) -> Option<(u32, Result<(u32, bool), OneWireError>)> {
		self.last_idx = self.iter.current_index();

		let start_time = self.current_time();
		let reset = self.iter.next()?;
		let end_time = self.current_time();
		match reset {
			Edge::Rising => return Some((start_time, Err(OneWireError::UnexpectedRisingEdge))),
			_ => {}
		}

		let mut duration = end_time - start_time;
		
		let mut response_start = start_time;
		let mut response_end = start_time + self.timing.response.max;
		let mut device_responded = false;

		if duration < self.timing.response.max {
			self.iter.next()?;

			response_start = end_time;
			response_end = self.current_time();

			duration = response_start - start_time;

			if duration < self.timing.response.min {
				return Some((response_start, Err(OneWireError::ResponseTooShort)));
			}

			if duration > self.timing.reset_recover_min {
				return Some((response_start, Err(OneWireError::ResponseTooLong)));
			}

			device_responded = true;
			self.iter.next()?;
		} else {
			self.discard_last();
		}

		Some((response_start, Ok((response_end, device_responded))))
    }

	pub fn next_reset_recovery(&mut self, response_start: u32) -> Option<Result<u32, OneWireError>> {
		self.last_idx = self.iter.current_index();

		let start_time = self.current_time();
		let reset = self.iter.next()?;
		match reset {
			Edge::Rising => return Some(Err(OneWireError::UnexpectedRisingEdge)),
			_ => {}
		}

		if (self.current_time() - response_start) < self.timing.reset_recover_min {
			return Some(Err(OneWireError::ResetRecoveryTooShort))
		}

		Some(Ok(start_time + self.timing.reset_recover_min))
	}

	// forwards self, so self.next_reset() will be a 'valid' reset
    pub fn forward_to_reset(&mut self) -> Option<_> {
		self.last_idx = self.iter.current_index();

		loop {
			let mut idx = self.iter.current_index();
			let mut start = self.current_time();
			let mut next = self.iter.next()?;

			if (start - self.current_time()) >= self.timing.reset.min && next == Edge::Rising {
				self.iter.set_index(idx);
				return Some(())
			}
		}
    }

    pub fn discard_last(&mut self) {
		self.iter.set_index(self.last_idx);
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