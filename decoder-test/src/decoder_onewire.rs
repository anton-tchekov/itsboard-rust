use num_traits::One;

use crate::decoder::*;
use crate::sample::*;

use core::iter::Peekable;
use core::convert::TryFrom;
use core::ops::{Add, Mul};

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
struct Range<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy
{
    min: T,
    max: T,
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
struct Timings<T> 
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
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

impl <T>Timings<T>
where 
	T: Add<Output = T> + Mul<Output = T> + Copy,
{
	fn max_bit_length(&self) -> T {
		self.wr_slot.max + self.line_recover_min
	}

	fn scale(&self, factor: T) -> Self {
		Self {
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

impl Timings<u32> {
	fn standard() -> Self {
		Timings {
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
	
	fn overdrive() -> Self {
		Timings {
			wr_init: Range { min: 1.0, max: 1.85},
			bit_sample: 0.75,
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
			bit_sample: self.bit_sample as u32,
			wr_slot: self.wr_slot.as_u32(),
			line_recover_min: self.line_recover_min as u32,
			reset: self.reset.as_u32(),
			response: self.response.as_u32(),
			reset_recover_min: self.reset_recover_min as u32,
		}
	}
}

#[derive(Copy, Clone)]
enum OneWireError {
	ResponseTooShort,
	ResponseTooLong,
	ResetTooShort,
	ResetTooLong,
	BitInitTooShort,
	BitInitTooLong,
	BitSlotTooShort,
	BitSlotTooLong,
	LineRecoveryTooShort,
	UnexpectedReset,
}

impl OneWireError {
	fn to_string(&self) -> &'static str {
		match self {
			OneWireError::ResponseTooShort => "Response too short",
			OneWireError::ResponseTooLong => "Response too long",
			OneWireError::ResetTooShort => "Reset too short",
			OneWireError::ResetTooLong => "Reset too long",
			OneWireError::BitInitTooShort => "Bit initialization too short",
			OneWireError::BitInitTooLong => "Bit initialization too long",
			OneWireError::BitSlotTooShort => "Bit slot too short",
			OneWireError::BitSlotTooLong => "Bit slot too long",
			OneWireError::LineRecoveryTooShort => "Line recovery too short",
			OneWireError::UnexpectedReset => "Unexpected reset pulse",
		}
	}
}

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

struct OneWireBit<'a, T: Iterator<Item = BitData>> {
    signal: &'a mut T,
	timings: &'a Timings<u32>
}

impl<'a, T: Iterator<Item = BitData>> OneWireBit<'a, T> {

    pub fn new(signal: &'a mut T, timings: &'a Timings<u32>) -> Self {
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

impl<'a, T: Iterator<Item = BitData>> Iterator for OneWireBit<'a, T> {
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

struct DecodeState<'a> {
	output: &'a mut SectionBuffer,
	signal: &'a mut PulsewiseIterator<'a>,
}

impl<'a> DecodeState<'a> {

	fn read_bits<const N: u8>(
		&mut self,
		timing: &Timings<u32>,
	) -> Option<ReadBitsResult>
	where
    	BitsAmount<N>: Sized
	{
		let mut result = ReadBitsResult::default();
		let mut bit_iter = OneWireBit::new(self.signal, timing).peekable();
		
		result.end = bit_iter.peek()?.end;

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

impl Iterator for DecodeState<'_> {
	type Item = BitData;

	fn next(&mut self) -> Option<Self::Item> {
		self.signal.next()
	}
}

enum DecoderOneWireState {
	Reset(ResetState),
	ROMCmd(ROMCommandState),
	FamilyCode(FamilyCodeState),
	SensorID(SensorIDState),
	CRC(CRCState),
	SearchROM(SearchROMState),
	FunctionCmd(FunctionCmdState),
	Data(DataState),
}
impl DecoderOneWireState {
	pub fn process(&self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		match self {
			DecoderOneWireState::Reset(state) => state.process(decode),
			DecoderOneWireState::ROMCmd(state) => state.process(decode),
			DecoderOneWireState::CRC(state) => state.process(decode),
			DecoderOneWireState::FamilyCode(state) => state.process(decode),
			DecoderOneWireState::SensorID(state) => state.process(decode),
			DecoderOneWireState::SearchROM(state) => state.process(decode),
			DecoderOneWireState::FunctionCmd(state) => state.process(decode),
			DecoderOneWireState::Data(state) => state.process(decode),
		}
	}

	pub fn from_rom(cmd: ROMCommand) -> DecoderOneWireState {
		match cmd {
			ROMCommand::SearchROM => DecoderOneWireState::SearchROM(SearchROMState),
			ROMCommand::SkipROM => DecoderOneWireState::FunctionCmd(FunctionCmdState(Timings::standard())),
			ROMCommand::OverdriveSkipROM => DecoderOneWireState::FunctionCmd(FunctionCmdState(Timings::overdrive())),
			ROMCommand::OverdriveMatchROM => DecoderOneWireState::FamilyCode(FamilyCode(Timings::overdrive())),
			_ => DecoderOneWireState::FamilyCode(FamilyCode(Timings::standard())),
		}
	}
}

struct ResetState;

impl ResetState {
	pub fn process(&self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		let mut next = decode.next()?;
		let timing = Timings::standard();

		if next.duration() < timing.reset.min {
			decode.push(Section::from_bit(&next, SectionContent::Err("Reset too short")))?;
			return Some(DecoderOneWireState::Reset(ResetState));
		}
		
		if next.duration() > timing.reset.max {
			decode.push(Section::from_bit(&next, SectionContent::Err("Reset too long")))?;
			return Some(DecoderOneWireState::Reset(ResetState));
		}

		decode.push(Section::from_bit(&next, SectionContent::Reset))?;

		// Check for response
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

		decode.push(Section {
			start: next.start,
			end: recovery_end,
			content: SectionContent::ResetRecovery
		})?;

		decode.push(Section {
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
	fn process(&self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		decode.process_bits::<8, _>(&Timings::standard(), |value| {
			let rom_cmd = ROMCommand::try_from(value as u8);
			let content = match rom_cmd {
				Ok(cmd) => SectionContent::ROMCmd(cmd),
				Err(err) => SectionContent::Err(err),
			};

			match rom_cmd {
				Ok(cmd) => (content, Some(DecoderOneWireState::from_rom(cmd))),
				Err(_) => (content, None),
			}
		})
	}
}

struct FamilyCode(Timings<u32>);
impl FamilyCode {
	fn process(self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		decode.process_bits::<8, _>(&self.0, |value| {
			let content: SectionContent = SectionContent::FamilyCode(value as u8);
			(content, Some(DecoderOneWireState::SensorID(SensorIDState(self.0))))
		})
	}
}

struct SensorIDState(Timings<u32>);
impl SensorIDState {
	fn process(self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		decode.process_bits::<48, _>(&self.0, |value| {
			let content = SectionContent::SensorID(value);
			(content, Some(DecoderOneWireState::CRC(CRCState(self.0))))
		})
	}
}

struct CRCState(Timings<u32>);
impl CRCState {
	fn process(self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		decode.process_bits::<8, _>(&self.0, |value| {
			let content = SectionContent::CRC(value as u8);
			(content, Some(DecoderOneWireState::FunctionCmd(FunctionCmdState(self.0))))
		})
	}
}

struct FunctionCmdState(Timings<u32>);
impl FunctionCmdState{
	fn process(self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		decode.process_bits::<8, _>(&self.0, |value| {
			let content = SectionContent::FunctionCmd(value as u8);
			(content, Some(DecoderOneWireState::Data(DataState(self.0))))
		})
	}
}

// TODO: decode more information from SearchROM
struct SearchROMState {
	iteration: u8,
}
impl SearchROMState {
	fn process(self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		let timing = Timings::standard();
		if self.iteration >= 3 {
			return Some(DecoderOneWireState::FunctionCmd(FunctionCmdState(timing)));
		}

		decode.process_bits::<64, _>(&timing, |value| {
			let content = SectionContent::Data(value);
			(content, Some(DecoderOneWireState::SearchROM(SearchROMState {
				iteration: self.iteration + 1,
			})))
		})
	}
}

struct DataState(Timings<u32>);
impl DataState {
	fn process(&self, decode: &mut DecodeState) -> Option<DecoderOneWireState> {
		// we don't know how many bits we will read, so we read bit for bit until we reach the end of the signal or a reset pulse
		let timing = self.0;
		let peeked = decode.peekable().peek()?;

		let mut value = 0;
		let mut bits_read = 0;
		let mut start_time = peeked.start;
		let mut end_time = peeked.end;
		loop {
			if bits_read >= 64 {
				self.push(Section {
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
			if !pulse.high && pulse.duration() >= timing.reset.min {
				return Some(DecoderOneWireState::Reset(ResetState));
			}

			let mut bit_iter = OneWireBit::new(signal, &timing);

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
