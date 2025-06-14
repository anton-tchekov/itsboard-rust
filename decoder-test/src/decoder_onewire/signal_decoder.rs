
use std::env::temp_dir;

use crate::decoder::{Section, SectionContent, SectionBuffer};
use crate::sample::{PulsewiseIterator};
use crate::decoder_onewire::onewire_error::OneWireError;
use crate::decoder_onewire::timings::Timings;

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

#[derive(Clone, Copy)]
struct ErrorBit {
    error: OneWireError,
    start: u32,
    end: u32,
}

#[derive(Clone, Copy)]
struct ReadBitsResult {
	value: u64,
	bits_read: u8,
	start: u32,
	end: u32,
}

impl Default for ReadBitsResult {
	fn default() -> Self {
		Self {
			value: 0,
			bits_read: 0,
			start: 0,
			end: 0,
		}
	}
}

struct SignalDecoder<'a> {
	output: &'a mut SectionBuffer,
	signal: PulsewiseIterator<'a>,
}

impl<'a> SignalDecoder<'a> {

	fn read_bits<'b, const N: u8>(
		&mut self,
		timing: &Timings<u32>,
	) -> Option<(ReadBitsResult, Result<(), ErrorBit>)>
	where
    	BitsAmount<N>: Sized,
	{
		let mut result = ReadBitsResult::default();
		let mut bit_iter = OneWireBit::new(&mut self.signal, timing).peekable();
		
		result.end = bit_iter.peek()?.end;

		for i in 0..N {
			let bit_result = match bit_iter.next() {
				Some(b) => b,
				None => { return Some((result, Ok(()))); }
			};

			self.output.push(bit_result.to_section()).ok()?;

			match bit_result.high {
				Ok(b) => {
					result.value |= (b as u64) << i;
					result.bits_read += 1;
					result.end = bit_result.end;
				},
				Err(err) => {
					let error = Err({
                        ErrorBit {
                            start: bit_result.start,
                            end: bit_result.end,
                            error: err,
                        }
                    });
					return Some((result, error));
				}
			}
		}
		Some((result, Ok(())))
	}

	fn process_bits<'b, const N: u8, F>(
		&'b mut self,
		timing: &'b Timings<u32>,
		value_to_content: F,
	) -> Option<Result<(), ErrorBit>>
	where
		BitsAmount<N>: Sized,
		F: FnOnce(u64) -> SectionContent,
        'b: 'a,
	{
		let (read, result) = self.read_bits::<N>(timing)?;

		self.push(Section {
			start: read.start,
			end: read.end,
			content: value_to_content(read.value),
		});

		Some(result)
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