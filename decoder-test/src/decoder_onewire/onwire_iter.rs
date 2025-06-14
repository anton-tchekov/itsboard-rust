struct BitResult {
	high: Result<bool, OneWireError>,
	start: u32,
	end: u32,
}

impl BitResult {
	fn to_section(&self) -> Section {
		Section {
			start: self.start,
			end: self.end,
			content: match self.high {
				Ok(value) => SectionContent::Bit(value),
				Err(err) => SectionContent::Err(err.to_string()),
			},
		}
	}
}

struct OnewireIter<'a> {
    iter: EdgewiseIterator<'a>,
    last_idx: usize,
    timing: Timings<u32>,
}

impl <'a>OnewireIter<'a> {

	fn check_timings(&self, duration: u32) -> Result<(), OneWireError> {
        match duration {
            d if d < self.timings.wr_init.min => Err(OneWireError::BitInitTooShort),
            d if d > self.timings.wr_slot.max && d >= self.timings.reset.min => {
				Err(OneWireError::UnexpectedReset)
			}
			d if d > self.timings.wr_slot.max && d < self.timings.reset.min => {
				Err(OneWireError::BitSlotTooLong)
			}
            d if d > self.timings.wr_init.max && d < self.timings.wr_slot.min => {
                Err(OneWireError::BitInitTooLong)
            }
            _ => Ok(()),
        }
    }

    fn next_bit() {
		self.iter.idx = self.last_idx;

		let init = self.signal.next()?;
		let mut duration = init.duration();

		let mut result = BitResult {
			high: Ok(false),
			start: init.start,
			end: init.end,
		}; 

		if let Err(err) = self.check_timings(duration) {
			result.high = Err(err);
			return Some(result);
		}

		// bit is high
		if duration <= self.timings.wr_init.max {
			result.high = Ok(true);

			let high = self.signal.next()?; 
			// we don't want to stretch the time if the line is idle after bit
			duration += high.duration().min(self.timings.wr_slot.max);

			if duration < self.timings.wr_slot.min + self.timings.line_recover_min {
				result.high = Err(OneWireError::BitSlotTooShort);
				result.end = high.end;
				return Some(result);
			}

			return Some(result);
		}

		// bit was low
		let recovery = self.signal.next()?;
		result.end = recovery.start;

		if recovery.duration() < self.timings.line_recover_min {
			result.high = Err(OneWireError::LineRecoveryTooShort);
 			return Some(result);
		}

		Some(result)
    }

    fn next_reset() {
		self.iter.idx = last_idx;

		let mut next = iter.next()?;

		if next.duration() < timing.reset.min {
			decode.push(Section::from_bit(&next, SectionContent::Err("Reset too short")))?;
			return Some(DecoderOneWireState::Reset(ResetState));
		}
		
		if next.duration() > timing.reset.max {
			decode.push(Section::from_bit(&next, SectionContent::Err("Reset too long")))?;
			return Some(DecoderOneWireState::Reset(ResetState));
		}

		decode.push(Section::from_bit(&next, SectionContent::Reset))?;
    }

    fn next_response() {
		self.iter.idx = last_idx;

		let response_start = next.end;
		next = decode.next()?;

		let mut response_duration = next.start - response_start;
		let mut  device_responded = false;

		if response_duration > timing.response.max {
			decode.push(Section {
				start: response_start,
				end: response_start + timing.response.max,
				content: SectionContent::NoDeviceResponse,
			})?;
		}
		else {
			next = decode.next()?;
			response_duration = next.end - response_start;

			if response_duration < timing.response.min {
				decode.push(Section::from_bit(&next, SectionContent::Err("Response too short/early")))?;
				return Some(DecoderOneWireState::Reset(ResetState));
			}

			if response_duration > timing.reset_recover_min {
				decode.push(Section::from_bit(&next, SectionContent::Err("Response too long")))?;
				return Some(DecoderOneWireState::Reset(ResetState));
			}

			decode.push(Section::from_bit(&next, SectionContent::DeviceResponse))?;
			device_responded = true;
			next = decode.next()?;
			response_duration = next.end - response_start;
		}

		if response_duration < timing.reset_recover_min {
			decode.push(Section::from_bit(&next, SectionContent::Err("Response recovery too short")))?;
			return Some(DecoderOneWireState::Reset(ResetState));
		}

		let recovery_end  = response_start + timing.reset_recover_min;
    }

    fn forward_to_reset() {
    }

    fn consume_last() {
    }

    fn current_time() {
    }

    fn set_timing() {
    }
}

impl <'a>From<EdgewiseIterator<'a>> for OnewireIter<'a> {
    fn from(iter: EdgewiseIterator<'a>) -> Self {
        OnewireIter {
            iter,
            last_idx: iter.idx,
        }
    }
}