
use crate::decoder::{Section, SectionContent, SectionBuffer};
use crate::sample::{PulsewiseIterator};
use crate::decoder_onewire::onewire_error::OneWireError;
use crate::decoder_onewire::timings::Timings;

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

struct OneWireBit<'a> {
    signal: &'a mut PulsewiseIterator<'a>,
	timings: &'a Timings<u32>
}

impl<'a> OneWireBit<'a> {

    pub fn new(signal: &'a mut PulsewiseIterator<'a>, timings: &'a Timings<u32>) -> Self {
        Self { signal, timings }
    }

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
}

impl<'a> Iterator for OneWireBit<'a> {
	type Item = BitResult;

	fn next(&mut self) -> Option<Self::Item> {
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

		// consume the initial pulse
		self.signal.next()?;
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
}

struct ReadBitsResult {
	value: u64,
	bits_read: u8,
	start: u32,
	end: u32,
	result: Result<(), OneWireError>,
}

impl Default for ReadBitsResult {
	fn default() -> Self {
		Self {
			value: 0,
			bits_read: 0,
			start: 0,
			end: 0,
			result: Ok(()),
		}
	}
}

// For compile-time check, that N is in range 1..=64
pub struct BitsAmount<const N: u8>;

impl<const N: u8> BitsAmount<N> {
    pub const VALUE: u8 = N;
}

impl<const N: u8> BitsAmount<N> {
	const ASSERT: () = assert!(N >= 1 && N <= 64, "BitsAmount must be in range 1..=64");
}

struct SignalDecoder<'a> {
	output: &'a mut SectionBuffer,
	signal: PulsewiseIterator<'a>,
}

impl<'a> SignalDecoder<'a> {

	fn read_bits<'b, const N: u8>(
		&'b mut self,
		timing: &'b Timings<u32>,
	) -> Option<ReadBitsResult>
	where
    	BitsAmount<N>: Sized,
        'b: 'a,
	{
		let mut result = ReadBitsResult::default();
		let mut bit_iter = OneWireBit::new(&mut self.signal, timing);
        let mut bit_iterp = bit_iter.peekable();
		
		result.end = bit_iterp.peek()?.end;

		for i in 0..N {
			let bit_result = match bit_iter.next() {
				Some(b) => b,
				None => { return Some(result); }
			};

			self.output.push(bit_result.to_section()).ok()?;

			match bit_result.high {
				Ok(b) => {
					result.value |= (b as u64) << i;
					result.bits_read += 1;
					result.end = bit_result.end;
				},
				Err(err) => {
					result.result = Err(err);
					return Some(result);
				}
			}
		}
		Some(result)
	}

	fn process_bits<const N: u8, F>(
		&mut self,
		timing: &Timings<u32>,
		value_to_content: F,
	) -> Option<DecoderOneWireState>
	where
		BitsAmount<N>: Sized,
		F: FnOnce(u64) -> (SectionContent, Option<DecoderOneWireState>),
	{
		let read = self.read_bits::<N>(timing)?;
		// TODO: this is wrong, needs to be fixed

		let (content, next_state) = match read.result {
			Err(err) => (SectionContent::Err(err), Some(DecoderOneWireState::Reset(ResetState))),
			Ok(_) => {
				let (content, next_state) = value_to_content(read.value);

				if read.bits_read == N {
					(content, next_state)
				} else {
					// we still want to push the partial section (the furthest, we could decode)
					(content, None)
				}
			}
		};

		self.push(Section {
			start: read.start,
			end: read.end,
			content,
		})?;

		next_state
	}

	// Returns the next reset pulse or the end of the signal if there are no further reset pulses
	// and None if there are no further pulses.
	fn next_reset(&mut self) -> Option<BitData> {
		let timing = Timings::standard();
		let mut last_seen = *self.peekable().peek()?;

		while let Some(pulse) = self.peekable().peek() {
			last_seen = *pulse;
			if !pulse.high && pulse.duration() >= timing.reset.min {
				return Some(*pulse);
			}
			self.signal.next();
		}

		Some(last_seen)
	}

	pub fn push(&mut self, mut section: Section) -> Option<()> {
		if let SectionContent::Err(_) = &section.content {
			if let Some(next) = self.next_reset() {
				section.end = next.end;
			}
		}
		self.section_buf.push(section).ok()
	}
}

impl Iterator for SignalDecoder<'_> {
	type Item = BitData;

	fn next(&mut self) -> Option<Self::Item> {
		self.signal.next()
	}
}