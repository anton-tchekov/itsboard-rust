mod timings;
mod onewire_error;
mod onwire_iter;
mod onewire_output;
pub mod rom_cmd;

use timings::*;

use crate::bit_reader::BitReader;
use crate::decoder::*;
use crate::decoder_onewire::onewire_output::OneWireOutput;
use crate::decoder_onewire::onwire_iter::OnewireIter;
use crate::decoder_onewire::rom_cmd::ROMCmd;
use crate::sample::*;
use onewire_error::*;

fn process_bits<G>(
    iter: &mut OnewireIter,
    output: &mut OneWireOutput,
    amount:  u8,
    success_state: OneWireState,
    value_to_content: G,
) -> Option<OneWireState>
where
    G: FnOnce(u64) -> SectionContent,
{
    let start = iter.current_time();
    let mut reader = BitReader::lsb(amount);
    let (end, result) = read_bits(iter, output, &mut reader)?;

    if let Some(value) = reader.get_value() {
        let content = value_to_content(value);
        output.push(Section { start, end, content })?;
    }

    if reader.is_finished() {
        return Some(success_state)
    }

    if let Err(err) = result {
        output.push_err(iter, end, err)?;
        return Some(OneWireState::Reset(ResetState));
    };

    None
}

pub fn read_bits(    
    iter: &mut OnewireIter,
    output: &mut OneWireOutput,
    reader: &mut BitReader,
) -> Option<(u32, Result<(), OneWireError>)> {
    let mut current_time = iter.current_time();
	let mut end_time = iter.current_time();

    while let Some((end, value)) = iter.next_bit() {
		end_time = end;
        match value {
            Ok(bit) => {
                output.push(Section { 
                    start: current_time,
                    end: end_time,
                    content: SectionContent::Bit(bit)
                })?;
                if reader.read_bit(bit) { break; }
            },
            Err(err) => return Some((end_time, Err(err))),
        };

        current_time = iter.current_time();
    }

    Some((end_time, Ok(())))
}

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
	pub fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
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

	pub fn from_rom(iter: &mut OnewireIter, cmd: ROMCmd) -> OneWireState {
		match cmd {
			ROMCmd::SearchROM => OneWireState::SearchROM(SearchROMState {iteration: 0}),
			ROMCmd::SkipROM => OneWireState::FunctionCmd(FunctionCmdState),
			ROMCmd::OverdriveSkipROM => {
				iter.set_timing(Timings::overdrive());
				OneWireState::FunctionCmd(FunctionCmdState)
			},
			ROMCmd::OverdriveMatchROM => {
				iter.set_timing(Timings::overdrive());
				OneWireState::FamilyCode(FamilyCodeState)
			},
			_ => OneWireState::FamilyCode(FamilyCodeState),
		}
	}
}

struct ResetState;

impl ResetState {
	pub fn process(&self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		iter.set_timing(Timings::standard());

		let start = iter.current_time();
		let reset = iter.next_reset()?;
		let end = iter.current_time();
		// TODO maybe more general push
		match reset {
			Err(err) => {
				output.push_err(iter, start, err)?;
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
				output.push_err(iter, start, err)?;
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
				output.push_err(iter, start, err)?;
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
	fn process(&self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		let mut reader = BitReader::lsb(8);

		let start = iter.current_time();
		let (end, mut result) = read_bits(iter, output, &mut reader)?;

		if let Some(value) = reader.get_value() {
			let rom_cmd = ROMCmd::try_from(value as u8);

			match (rom_cmd, result) {
				(Err(code), Err(_)) => {
					output.push(Section { start, end, content: SectionContent::Err(code.to_string())});
				},

				(Err(code), Ok(_)) => {
					result = Err(code)
				},

				(Ok(cmd), _) => {
					output.push(Section { start, end, content: SectionContent::ROMCmd(cmd) });
					
					if reader.is_finished() {
						return Some(OneWireState::from_rom(iter, cmd));
					}
				},
			};
		};

		if let Err(err) = result {
			output.push_err(iter, end, err)?;
			return Some(OneWireState::Reset(ResetState));
		};

		None
	}
}

struct FamilyCodeState;
impl FamilyCodeState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		process_bits(iter, output, 8, OneWireState::SensorID(SensorIDState), |value| {
			SectionContent::FamilyCode(value as u8)
		})
	}
}

struct SensorIDState;
impl SensorIDState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		process_bits(iter, output, 48, OneWireState::CRC(CRCState), |value| {
			SectionContent::SensorID(value)
		})
	}
}

struct CRCState;
impl CRCState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		process_bits(iter, output, 8, OneWireState::FunctionCmd(FunctionCmdState), |value| {
			SectionContent::CRC(value as u8)
		})
	}
}

struct FunctionCmdState;
impl FunctionCmdState{
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		process_bits(iter, output, 8, OneWireState::Data(DataState), |value| {
			SectionContent::FunctionCmd(value as u8)
		})
	}
}

// TODO: decode more information from SearchROM
struct SearchROMState {
	iteration: u8,
}

impl SearchROMState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		if self.iteration >= 3 {
			return Some(OneWireState::FunctionCmd(FunctionCmdState));
		}

		process_bits(iter, output, 64, OneWireState::SearchROM(SearchROMState {iteration: self.iteration + 1 }), |value| {
			SectionContent::Data(value)
		})
	}
}

struct DataState;
impl DataState {
	fn process(&self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		let mut reader = BitReader::lsb(64);

		let start = iter.current_time();
		let (end, result) = read_bits(iter, output, &mut reader)?;

		if let Some(value) = reader.get_value() {
			let content = SectionContent::Data(value);
			output.push(Section { start, end, content })?;
		}

		if reader.is_finished() {
			return Some(OneWireState::Data(DataState))
		}

		if let Err(err) = result {
			match err {
				OneWireError::UnexpectedReset => {
					iter.discard_last();
					return Some(OneWireState::Reset(ResetState))
				},
				_ => {}
			}

			output.push_err(iter, end, err)?;
			return Some(OneWireState::Reset(ResetState))
		}
		None
	}
}

pub struct DecoderOneWire {
	pub onewire_pin: DecoderPin
}

impl Decoder for DecoderOneWire {
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		let mut iter = OnewireIter::from(samples.edge_iter(self.onewire_pin));
		let mut output = OneWireOutput::from(output);

		let mut state = OneWireState::Reset(ResetState);

		while let Some(next) = state.process(&mut iter, &mut output) {
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sample::*;
	use crate::decoder::*;
	use crate::decoder_uart::*;
	use crate::test_utils::*;

	fn decoder() -> DecoderOneWire {
		DecoderOneWire {
			onewire_pin: 1
		}
	}

	#[test]
	fn test_measure_temp() {
		let onewire = decoder();
		let sections = decode_sections("1Wire/OneWireReadROM_MeasureTemp.csv", onewire);
		let mut section_iter = sections.iter();

		assert_top_layer_eq(&sections, &[
			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery, 
			SectionContent::ROMCmd(ROMCmd::ReadROM), SectionContent::FamilyCode(0), SectionContent::SensorID(0), SectionContent::CRC(0), 
			SectionContent::FunctionCmd(0), SectionContent::Data(0),

			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,
			SectionContent::ROMCmd(ROMCmd::MatchROM), SectionContent::FamilyCode(0), SectionContent::SensorID(0), SectionContent::CRC(0), 
			SectionContent::FunctionCmd(0), SectionContent::Data(0),
		])
	}

	#[test]
	fn test_time_overlap() {
		let onewire = decoder();
		let sections = decode_sections("1Wire/OneWireReadROM_MeasureTemp.csv", onewire);

		assert_bit_layer_no_time_overlap(&sections);
		assert_top_layer_no_time_overlap(&sections);
	}
}
