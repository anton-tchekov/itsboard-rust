mod timings;
mod signal_decoder;
mod onewire_error;

use timings::*;
use signal_decoder::*;
use onewire_error::*;

use crate::decoder::*;
use crate::sample::*;

use core::iter::Peekable;

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
	pub fn process(&self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		let reset = iter.next_reset();
		// TODO maybe more general push
		match reset.result {
			Err(err) => {
				output.push_err(iter, err, reset.start, reset.end)?;
				return Some(DecoderOneWireState::Reset(ResetState))
			}

			Ok => {
				output.push(Section {
					start: reset.start,
					end: reset.end,
					content: SectionContent::Reset,
				});
			}
		};

		let response = iter.next_response();
		match response.result {
			Err(err) => {
				output.push_err(iter, err, response.start, response.end)?;
				return Some(DecoderOneWireState::Reset(ResetState));
			}

			Ok(responded) => {
				output.push(Section {
					start: response.start,
					end: response.end,
					content: SectionContent::ResetResponse(responded) 
				});
			}
		};
		
		Some(DecoderOneWireState::ROMCmd(ROMCommandState))
	}
}

struct ROMCommandState;

impl ROMCommandState {
	fn process(&self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
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
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
			let content: SectionContent = SectionContent::FamilyCode(value as u8);
			(content, Some(DecoderOneWireState::SensorID(SensorIDState(self.0))))
		})
	}
}

struct SensorIDState(Timings<u32>);
impl SensorIDState {
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		process_bits_reset_on_err(48, iter, output, |value| {
			let content = SectionContent::SensorID(value);
			(content, Some(DecoderOneWireState::CRC(CRCState(self.0))))
		})
	}
}

struct CRCState(Timings<u32>);
impl CRCState {
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
			let content = SectionContent::CRC(value as u8);
			(content, Some(DecoderOneWireState::FunctionCmd(FunctionCmdState(self.0))))
		})
	}
}

struct FunctionCmdState(Timings<u32>);
impl FunctionCmdState{
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
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
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		if self.iteration >= 3 {
			return Some(DecoderOneWireState::FunctionCmd(FunctionCmdState(timing)));
		}

		process_bits_reset_on_err(64, iter, output, |value| {
			let content = SectionContent::Data(value);
			(content, Some(DecoderOneWireState::SearchROM(SearchROMState {
				iteration: self.iteration + 1,
			})))
		})
	}
}

struct DataState(Timings<u32>);
impl DataState {
	fn process(&self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<DecoderOneWireState> {
		process_bits(64, iter, output, |value| {
			OK(Some())
		}, |err| {
			match err {
				DecoderOneWireError::UnexpectedReset => {
					Ok(Some())
				}
			}
		}
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
