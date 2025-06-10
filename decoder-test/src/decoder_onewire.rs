use crate::decoder::*;
use crate::sample::*;

use core::iter::Peekable;
use core::convert::TryFrom;

const STANDARD_TIMINGS: Timings<u32> = Timings::standard();
const OVERDRIVE_TIMINGS: Timings<u32> = Timings::overdrive();

const fn ceilf(x: f32) -> u32 {
    let xi = x as u32;
    if x > xi as f32 {
        xi + 1
    } else {
        xi
    }
}

struct Range<T> {
	min: T,
	max: T,
}

impl Range<f32> {
    const fn scale(&self, factor: u32) -> Range<f32> {
        Self {
            min: self.min * factor as f32,
            max: self.max * factor as f32,
        }
    }

	const fn as_u32(&self) -> Range<u32> {
		Range {
			min: self.min as u32,
			max: ceilf(self.max) as u32,
		}
	}
}

impl Range<u32> {
	const fn scale(&self, factor: u32) -> Range<u32> {
		Self {
			min: self.min * factor,
			max: self.max * factor,
		}
	}
}

// TODO: link to source
struct Timings<T> {
	// Low time before read/write
	wr_init: Range<T>,
	// Sample time for read
	bit_sample: T,
	// Low time total
	wr_slot: Range<T>,
	// Line recovery time after read/write
	line_recover_min: T,
	// Reset time
	reset: Range<T>,
	// Reset response sample time
	response: Range<T>,
	// Reset recovery time
	reset_recover_min: T,
}

impl Timings<u32> {
	const fn standard() -> Self {
		Self {
			wr_init: Range { min: 5, max: 15 },
			bit_sample: 9,
			wr_slot: Range { min: 52, max: 120 },
			line_recover_min: 8,
			response: Range { min: 68, max: 80 },
			reset: Range { min: 480, max: 640 },
			reset_recover_min: 473,
		}
		.scale(TIMER_TICKS_PER_US)
	}

	const fn overdrive() -> Self {
		Timings {
			wr_init: Range { min: 1.0, max: 1.85},
			bit_sample: 0.75,
			wr_slot: Range { min: 7.0, max: 14.0 },
			line_recover_min: 2.5,
			response: Range { min: 7.2, max: 8.8 },
			reset: Range { min: 68.0, max: 80.0 },
			reset_recover_min: 46.7,
		}
		.scale(TIMER_TICKS_PER_US)
		.as_u32()
	}

	const fn scale(&self, factor: u32) -> Self {
		Timings {
			wr_init: self.wr_init.scale(factor),
			bit_sample: self.bit_sample * factor,
			wr_slot: self.wr_slot.scale(factor),
			line_recover_min: self.line_recover_min * factor,
			reset: self.reset.scale(factor),
			response: self.response.scale(factor),
			reset_recover_min: self.reset_recover_min * factor,
		}
	}
}

impl Timings<f32> {
	const fn scale(&self, factor: u32) -> Self {
		Timings {
			wr_init: self.wr_init.scale(factor),
			bit_sample: self.bit_sample * factor as f32,
			wr_slot: self.wr_slot.scale(factor),
			line_recover_min: self.line_recover_min * factor as f32,
			reset: self.reset.scale(factor),
			response: self.response.scale(factor),
			reset_recover_min: self.reset_recover_min * factor as f32,
		}
	}

	const fn as_u32(&self) -> Timings<u32> {
		Timings {
			wr_init: self.wr_init.as_u32(),
			bit_sample: self.bit_sample as u32,
			wr_slot: self.wr_slot.as_u32(),
			line_recover_min: self.line_recover_min as u32,
			reset: self.reset.as_u32(),
			response: self.response.as_u32(),
			reset_recover_min: self.reset_recover_min as u32,
		}
	}
}

struct OneWireSectionBuffer<'a> {
	section_buf: &'a mut SectionBuffer,
}

impl <'a>OneWireSectionBuffer<'a> {
	pub fn from_buf(buf: &'a mut SectionBuffer) -> Self {
		Self {
			section_buf: buf,
		}
	}

	pub fn push(&mut self, section: Section) -> Option<()> {
		self.section_buf.push(section).ok()
	}	
}

enum DecoderOneWireState {
	Reset(ResetState),
	ROMCmd(ROMCommandState),
	MatchROM(MatchROMState),
	SearchROM(SearchROMState),
	FunctionCmd(FunctionCmdState),
	Data(DataState),
}
impl DecoderOneWireState {
	pub fn process(&self, pulse: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		match self {
			DecoderOneWireState::Reset(state) => state.process(pulse, output),
			DecoderOneWireState::ROMCmd(state) => state.process(pulse, output),
			DecoderOneWireState::MatchROM(state) => state.process(pulse, output),
			DecoderOneWireState::SearchROM(state) => state.process(pulse, output),
			DecoderOneWireState::FunctionCmd(state) => state.process(pulse, output),
			DecoderOneWireState::Data(state) => state.process(pulse, output),
		}
	}
}

struct ResetState;

impl ResetState {
	// TODO: Do smth else instead of aborting on error e.x. try finding next valid reset
	pub fn process(&self, signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		let mut next = signal.next()?;

		if next.duration() < STANDARD_TIMINGS.reset.min {
			output.push(Section::from_bit(&next, SectionContent::Err("Reset too short")))?;
			return None;
		}
		
		if next.duration() > STANDARD_TIMINGS.reset.max {
			output.push(Section::from_bit(&next, SectionContent::Err("Reset too long")))?;
			return None;
		}

		output.push(Section::from_bit(&next, SectionContent::Reset))?;

		// Check for response
		let response_start = next.end;
		next = signal.next()?;

		let mut response_duration = next.start - response_start;
		let mut  device_responded = false;

		if response_duration > STANDARD_TIMINGS.response.max {
			output.push(Section {
				start: response_start,
				end: response_start + STANDARD_TIMINGS.response.max,
				content: SectionContent::NoDeviceResponse,
			})?;
		}
		else {
			next = signal.next()?;
			response_duration = next.end - response_start;

			if response_duration < STANDARD_TIMINGS.response.min {
				output.push(Section::from_bit(&next, SectionContent::Err("Response too short/early")))?;
				return None;
			}

			if response_duration > STANDARD_TIMINGS.reset_recover_min {
				output.push(Section::from_bit(&next, SectionContent::Err("Response too long")))?;
				return None;
			}

			output.push(Section::from_bit(&next, SectionContent::DeviceResponse))?;
			device_responded = true;
			next = signal.next()?;
			response_duration = next.end - response_start;
		}

		if response_duration < STANDARD_TIMINGS.reset_recover_min {
			output.push(Section::from_bit(&next, SectionContent::Err("Response recovery too short")))?;
			return None;
		}

		let recovery_end  = response_start + STANDARD_TIMINGS.reset_recover_min;

		output.push(Section {
			start: next.start,
			end: recovery_end,
			content: SectionContent::ResetRecovery
		})?;

		output.push(Section {
			start: recovery_end,
			end: next.end,
			content: SectionContent::Empty
		})?;

		if device_responded {
			return Some(DecoderOneWireState::ROMCmd(ROMCommandState))
		}
		Some(DecoderOneWireState::Reset(ResetState))
	}
}

struct ROMCommandState;

impl ROMCommandState {
	fn process(&self, signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		let result = read_bits(signal, output, &STANDARD_TIMINGS, 8);

		result.process_command(output, |value| {
			let rom_cmd = ROMCommand::try_from(value as u8);
			let content = match rom_cmd {
				Ok(cmd) => SectionContent::ROMCmd(cmd),
				Err(err) => SectionContent::Err(err),
			};

			match rom_cmd {
				Ok(cmd) => (content, Some(next_state_from_rom(cmd))),
				Err(_) => (content, None),
			}
		})
	}
}

struct MatchROMState(&'static Timings<u32>);

impl MatchROMState {
	fn process(&self, signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		let mut result = read_bits(signal, output, self.0, 8);
		result.process_command(output, |value| {
			let content = SectionContent::FamilyCode(value as u8);
			(content, None)
		});

		result = read_bits(signal, output, self.0, 48);
		result.process_command(output, |value| {
			let content = SectionContent::SensorID(value);
			(content, None)
		});

		result = read_bits(signal, output, self.0, 8);
		result.process_command(output, |value| {
			let content = SectionContent::CRC(value as u8);
			(content, Some(DecoderOneWireState::FunctionCmd(FunctionCmdState(self.0))))
		})
	}
}

struct FunctionCmdState(&'static Timings<u32>);
impl FunctionCmdState{
	fn process(&self, signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		let result = read_bits(signal, output, self.0, 8);

		result.process_command(output, |value| {
			let content = SectionContent::FunctionCmd(value as u8);
			(content, Some(DecoderOneWireState::Data(DataState(self.0))))
		})
	}
}

struct DataState(&'static Timings<u32>);
impl DataState {
	fn process(&self, signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		// we don't know how many bits we will read, so we read bit for bit until we reach the end of the signal or a reset pulse

		let mut value = 0;
		let mut bits_read = 0;
		let mut start_time = signal.peek()?.start;
		let mut end_time = signal.peek()?.end;
		loop {
			if bits_read >= 64 {
				output.push(Section {
					start: signal.peek()?.start,
					end: signal.peek()?.end,
					content: SectionContent::Data(value),
				})?;

				bits_read = 0;
				value = 0;
				start_time = signal.peek()?.start;
				end_time = signal.peek()?.end;
			}

			let pulse = signal.peek()?;
			if !pulse.high && pulse.duration() >= STANDARD_TIMINGS.reset.min {
				return Some(DecoderOneWireState::Reset(ResetState));
			}

			let mut bit_iter = OneWireBit::new(signal, self.0);

			let bit_result = match bit_iter.next() {
				Some(b) => b,
				None => break,
			};

			output.push(bit_result.to_section())?;

			match bit_result.high {
				Ok(b) => {
					value |= (b as u64) << bits_read;
					bits_read += 1;

					end_time = bit_result.end;
				},

				Err(err) => {
					output.push(Section {
						start: bit_result.start,
						end: bit_result.end,
						content: SectionContent::Err(err),
					})?;
					return Some(DecoderOneWireState::Reset(ResetState));
				}
			}
		}

		if bits_read > 0 {
			output.push(Section {
				start: start_time,
				end: end_time,
				content: SectionContent::Data(value),
			})?;
		}

		None
	}
}

struct SearchROMState;
impl SearchROMState {
	fn process(&self, signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer) -> Option<DecoderOneWireState> {
		for _ in 0..3 {
			let result = read_bits(signal, output, &STANDARD_TIMINGS, 64);

			result.process_command(output, |value| {
				let content = SectionContent::Data(value);
				(content, Some(DecoderOneWireState::Data(DataState(&STANDARD_TIMINGS))))
			})?;
		}

		Some(DecoderOneWireState::FunctionCmd(FunctionCmdState(&STANDARD_TIMINGS)))
	}
}

// TODO: push error function instead of doing it manually

// forward iterator to the next reset pulse or end of the signal if the signal contains no further reset pulses
// returns the next reset pulse if present, returns None if there are no further pulses, 
// the end pulse of the signal if the signal contains no further reset pulses
fn next_reset<T: Iterator<Item = BitData>>(signal: &mut Peekable<T>) -> Option<BitData> {
	// Find next reset pulse
	let mut next = *signal.peek()?;
	while next.high || next.duration() < STANDARD_TIMINGS.reset.min {
		signal.next();

		next = match signal.peek() {
			Some(next) => *next,
			None => return Some(next),
		};
	}
	Some(next)
}

fn next_state_from_rom(cmd: ROMCommand) -> DecoderOneWireState {
	match cmd {
		ROMCommand::SearchROM => DecoderOneWireState::SearchROM(SearchROMState),
		ROMCommand::SkipROM => DecoderOneWireState::FunctionCmd(FunctionCmdState(&STANDARD_TIMINGS)),
		ROMCommand::OverdriveSkipROM => DecoderOneWireState::FunctionCmd(FunctionCmdState(&OVERDRIVE_TIMINGS)),
		ROMCommand::OverdriveMatchROM => DecoderOneWireState::MatchROM(MatchROMState(&OVERDRIVE_TIMINGS)),
		_ => DecoderOneWireState::MatchROM(MatchROMState(&STANDARD_TIMINGS)),
	}
}

struct BitResult {
	high: Result<bool, &'static str>,
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
				Err(err) => SectionContent::Err(err),
			},
		}
	}
}

struct OneWireBit<'a, T: Iterator<Item = BitData>> {
    signal: &'a mut T,
	timings: &'a Timings<u32>
}

impl<'a, T: Iterator<Item = BitData>> OneWireBit<'a, T> {

	fn new(signal: &'a mut T, timings: &'a Timings<u32>) -> Self {
		Self {
			signal,
			timings,
		}
	}

	fn check_timings(&self, duration: u32) -> Result<(), &'static str> {
		let mut result = Ok(());
		if duration < self.timings.wr_init.min {
			result = Err("Bit init too short");
		}
		else if duration > self.timings.wr_slot.max {
			result =  Err("Bit slot too long");
		}
		else if duration > self.timings.wr_init.max && duration < self.timings.wr_slot.min {
			result = Err("Bit init too long or Bit slot too short");
		}
		result
	}

	fn error_bit(&mut self, start_time: u32, duration: u32, error: &'static str) -> BitResult {
		// forward to next reset
		let reset = next_reset(&mut self.signal.peekable());

		match reset {
			Some(next) => {
				BitResult {
					high: Err(error),
					start: start_time,
					end: next.start,
				}
			},
			None => {
				BitResult {
					high: Err(error),
					start: start_time,
					end: start_time + duration,
				}
			}
		}
	}
}

impl<'a, T: Iterator<Item = BitData>> Iterator for OneWireBit<'a, T> {
	type Item = BitResult;

	fn next(&mut self) -> Option<Self::Item> {
		let mut next = self.signal.next()?;

		let mut duration = next.duration();
		let start_time = next.start;

		if let Err(err) = self.check_timings(duration) {
			return Some(self.error_bit(start_time, duration, err));
		}

		let mut bit_value = false;

		if duration <= self.timings.wr_init.max {
			bit_value = true;

			next = self.signal.next()?;
			duration += next.duration();

			if duration < self.timings.wr_slot.min {
				return Some(self.error_bit(start_time, duration, "Bit slot too short"));
			}
		}

		next = self.signal.next()?;

		if next.duration() < self.timings.line_recover_min {
			return Some(self.error_bit(start_time, duration, "Line recovery too short"));
		}

		Some(BitResult {
			high: Ok(bit_value),
			start: start_time,
			end: next.start,
		})
	}
}

struct ReadBitsResult {
	value: u64,
	bits_read: u8,
	start: u32,
	end: u32,
	result: Result<(), &'static str>,
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

impl ReadBitsResult {

	fn process_command<F>(
		&self,
		output: &mut OneWireSectionBuffer,
		value_to_content: F,
	) -> Option<DecoderOneWireState>
	where
		F: Fn(u64) -> (SectionContent, Option<DecoderOneWireState>),
	{
		let (content, next_state) = match self.result {
			Err(err) => (SectionContent::Err(err), Some(DecoderOneWireState::Reset(ResetState))),
			Ok(_) if self.bits_read == 0 => return None,
			Ok(_) => {
				let (content, next_state) = value_to_content(self.value);

				match (self.bits_read == 8, next_state) {
					(true, Some(state)) => (content, Some(state)),
					_ => (content, None),
				}
			}
		};

		output.push(Section {
			start: self.start,
			end: self.end,
			content,
		})?;

		next_state
	}
}

fn read_bits(signal: &mut Peekable<PulsewiseIterator>, output: &mut OneWireSectionBuffer, timing: &Timings<u32>, amount: u8) -> ReadBitsResult
{
	// TODO restrict amount to 1-64 bits
	let mut bit_iter = OneWireBit::new(signal, timing);

	// Read the first bit
	let first_bit = match bit_iter.next() {
		Some(b) => b,
		None => return ReadBitsResult::default(),
	};

	if output.push(first_bit.to_section()).is_none() {
		return ReadBitsResult::default();
	}

	let mut result = match first_bit.high {
		Ok(b) => {
			ReadBitsResult {
				value: b as u64,
				bits_read: 1,
				start: first_bit.start,
				end: first_bit.end,
				result: Ok(()),
			}
		},

		Err(err) => {
			return ReadBitsResult {
				value: 0,
				bits_read: 0,
				start: first_bit.start,
				end: first_bit.end,
				result: Err(err),
			};
		}
	};

	// handle the rest of the bits
	for i in 1..amount {
		let bit_result = match bit_iter.next() {
            Some(b) => b,
			None => { return result; }
        };

		if output.push(bit_result.to_section()).is_none() {
			break
		}

		match bit_result.high {
			Ok(b) => {
				result.value |= (b as u64) << i;
				result.bits_read += i;
				result.end = bit_result.end;
			},
			Err(err) => {
				result.result = Err(err);
				return result;
			}
		}
	}
	result
}

pub struct DecoderOneWire {
	pub onewire_pin: DecoderPin
}

impl Decoder for DecoderOneWire {
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		let pulse_iter = PulsewiseIterator::new(samples, self.onewire_pin as u32);
		let mut signal = pulse_iter.peekable();

		let mut section_buf = OneWireSectionBuffer::from_buf(output);

		let mut state = DecoderOneWireState::Reset(ResetState);

		while let Some(next) = state.process(&mut signal, &mut section_buf) {
			state = next;
		}

		Ok(())
	}
	
	fn num_pins(&self) -> usize {
		1
	}
	
	fn get_pin(&self, idx: usize) -> Option<DecoderPin> {
		match idx
		{
			0 => Some(self.onewire_pin),
			_ => None,
		}
	}
	
	fn get_pin_name(&self, idx: usize) -> Option<&'static str> {
		match idx
		{
			0 => Some("OW"),
			_ => None,
		}
	}
}
