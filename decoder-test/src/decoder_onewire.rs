mod timings;
mod onewire_error;
mod process_bits;
mod onwire_iter;
mod onewire_output;

use timings::*;

use crate::decoder::*;
use crate::decoder_onewire::onewire_output::OneWireOutput;
use crate::decoder_onewire::onwire_iter::OnewireIter;
use crate::sample::*;

enum OneWireState {
	Reset(ResetState),
	ROMCmd(ROMCommandState),
	FamilyCode(FamilyCodeState),
	SensorID(SensorIDState),
	CRC(CRCState),
	SearchROM(SearchROMState),
	FunctionCmd(FunctionCmdState),
	Data(DataState),
}
impl OneWireState {
	pub fn process(&self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		match self {
			OneWireState::Reset(state) => state.process(iter, output),
			OneWireState::ROMCmd(state) => state.process(iter, output),
			OneWireState::CRC(state) => state.process(iter, output),
			OneWireState::FamilyCode(state) => state.process(iter, output),
			OneWireState::SensorID(state) => state.process(iter, output),
			OneWireState::SearchROM(state) => state.process(iter, output),
			OneWireState::FunctionCmd(state) => state.process(iter, output),
			OneWireState::Data(state) => state.process(iter, output),
		}
	}

	pub fn from_rom(iter: &mut OnewireIter, cmd: ROMCommand) -> OneWireState {
		match cmd {
			ROMCommand::SearchROM => OneWireState::SearchROM(SearchROMState),
			ROMCommand::SkipROM => OneWireState::FunctionCmd(FunctionCmdState),
			ROMCommand::OverdriveSkipROM => {
				iter.set_timing(Timings::overdrive());
				OneWireState::FunctionCmd(FunctionCmdState)
			},
			ROMCommand::OverdriveMatchROM => {
				iter.set_timing(Timings::overdrive());
				OneWireState::FamilyCode(FamilyCode)
			},
			_ => OneWireState::FamilyCode(FamilyCode),
		}
	}
}

struct ResetState;

impl ResetState {
	pub fn process(&self, iter: &mut OnewireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		iter.set_timing(Timings::standard());

		let mut start = iter.current_time();
		let reset = iter.next_reset()?;
		let mut end = iter.current_time();
		// TODO maybe more general push
		match reset {
			Err(err) => {
				output.push_err(iter, err, start, err)?;
				return Some(OneWireState::Reset(ResetState))
			}

			Ok(_) => {
				output.push(Section {
					start: start,
					end: end,
					content: SectionContent::Reset,
				});
			}
		};

		let (start, response) = iter.next_response()?;
		match response {
			Err(err) => {
				output.push_err(iter, err, start, err)?;
				return Some(OneWireState::Reset(ResetState));
			}

			Ok((end, responded)) => {
				output.push(Section {
					start,
					end,
					content: SectionContent::ResetResponse(responded) 
				});
			}
		};

		let start = iter.current_time();
		let recovery = iter.next_reset_recovery(start)?;
		match recovery {
			Err(err) => {
				output.push_err(iter, err, start, err)?;
				return Some(OneWireState::Reset(ResetState));
			}

			Ok(end) => {
				output.push(Section {
					start,
					end,
					content: SectionContent::ResetRecovery 
				});
			}
		}

		
		Some(OneWireState::ROMCmd(ROMCommandState))
	}
}

struct ROMCommandState;

impl ROMCommandState {
	fn process(&self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
			let rom_cmd = ROMCommand::try_from(value as u8);
			let content = match rom_cmd {
				Ok(cmd) => SectionContent::ROMCmd(cmd),
				Err(err) => SectionContent::Err(err),
			};

			match rom_cmd {
				Ok(cmd) => (content, Some(OneWireState::from_rom(cmd))),
				Err(_) => (content, None),
			}
		})
	}
}

struct FamilyCode(Timings<u32>);
impl FamilyCode {
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
			let content: SectionContent = SectionContent::FamilyCode(value as u8);
			(content, Some(OneWireState::SensorID(SensorIDState(self.0))))
		})
	}
}

struct SensorIDState(Timings<u32>);
impl SensorIDState {
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		process_bits_reset_on_err(48, iter, output, |value| {
			let content = SectionContent::SensorID(value);
			(content, Some(OneWireState::CRC(CRCState(self.0))))
		})
	}
}

struct CRCState(Timings<u32>);
impl CRCState {
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
			let content = SectionContent::CRC(value as u8);
			(content, Some(OneWireState::FunctionCmd(FunctionCmdState(self.0))))
		})
	}
}

struct FunctionCmdState(Timings<u32>);
impl FunctionCmdState{
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		process_bits_reset_on_err(8, iter, output, |value| {
			let content = SectionContent::FunctionCmd(value as u8);
			(content, Some(OneWireState::Data(DataState(self.0))))
		})
	}
}

// TODO: decode more information from SearchROM
struct SearchROMState {
	iteration: u8,
}
impl SearchROMState {
	fn process(self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
		if self.iteration >= 3 {
			return Some(OneWireState::FunctionCmd(FunctionCmdState(timing)));
		}

		process_bits_reset_on_err(64, iter, output, |value| {
			let content = SectionContent::Data(value);
			(content, Some(OneWireState::SearchROM(SearchROMState {
				iteration: self.iteration + 1,
			})))
		})
	}
}

struct DataState(Timings<u32>);
impl DataState {
	fn process(&self, iter: &mut OneWireIter, output: &mut SectionBuffer) -> Option<OneWireState> {
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

		let mut state = OneWireState::Reset(ResetState);

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
