mod timings;
mod onewire_error;
mod onewire_iter;
mod onewire_output;
pub mod rom_cmd;

use timings::Timings;

use crate::bit_reader::BitReader;
use crate::decoder::{SectionBuffer, SectionContent, Section, Decoder, DecoderPin};
use crate::decoder_onewire::onewire_output::OneWireOutput;
use crate::decoder_onewire::onewire_iter::OnewireIter;
use crate::decoder_onewire::rom_cmd::ROMCmd;
use crate::sample::SampleBuffer;
use onewire_error::OneWireError;

const NO_SLAVE_ON_BRANCH: u8 = 0b11;

#[derive(Default, Clone, Copy)]
enum ProcessorMode {
	#[default]
	Default,
	Search,
}

#[derive(Default, Clone, Copy)]
struct BitProcessor {
	mode: ProcessorMode
}

impl BitProcessor {
	fn process_bits<G>(
		&self,
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
		let (end, result) = self.read_bits(iter, output, &mut reader);

		let value = reader.get_value()?;
		let content = value_to_content(value);
		output.push(Section { start, end, content })?;

		if reader.is_finished() {
			return Some(success_state)
		}

		if let Err(err) = result {
			output.push_err(iter, end, err)?;
			return Some(OneWireState::Reset(ResetState));
		};

		None
	}

	fn read_bits(
		&self,
		iter: &mut OnewireIter,
		output: &mut OneWireOutput,
		reader: &mut BitReader,
	) -> (u32, Result<(), OneWireError>) 
	{
		let mut end_time = iter.current_time();

		while let Some((end, value)) = self.read_bit(iter, output) {
			end_time = end;
			match value {
				Ok(bit) => { if reader.read_bit(bit) { break; } },
				Err(err) => return (end_time, Err(err)),
			};
		}
		(end_time, Ok(()))
	}

	fn read_bit_default(
		&self,
		iter: &mut OnewireIter, 
		output: &mut OneWireOutput
	) -> Option<(u32, Result<bool, OneWireError>)>
	{
		let start = iter.current_time();
		let (end, result) = iter.next_bit()?;

		match result {
			Ok(bit) => {
				output.push(Section {
					start,
					end,
					content: SectionContent::Bit(bit)
				})?;
				Some((end, Ok(bit)))
			},
			Err(err) => Some((end, Err(err))),
		}
	}

	fn read_bit_search(
		&self,
		iter: &mut OnewireIter,
		output: &mut OneWireOutput,
	) -> Option<(u32, Result<bool, OneWireError>)>
	{
		let mut slave_reader = BitReader::lsb(2);
		let (end, result) = BitProcessor::default().read_bits(iter, output, &mut slave_reader);

		match result {
			Ok(_) => {},
			Err(e) => {
				return Some((end, Err(e)))
			}
		}

		let value = slave_reader.get_value()? as u8;
		if value == NO_SLAVE_ON_BRANCH {
			return Some((end, Err(OneWireError::EmptySearchBranch)))
		}

		let (end, result) = self.read_bit_default(iter, output)?;
		match result {
			Ok(bit) => Some((end, Ok(bit))),
			Err(e) => Some((end, Err(e)))
		}
	}

	fn read_bit(
		&self,
		iter: &mut OnewireIter, 
		output: &mut OneWireOutput
	) -> Option<(u32, Result<bool, OneWireError>)> 
	{
		match self.mode {
			ProcessorMode::Default => self.read_bit_default(iter, output),
			ProcessorMode::Search => self.read_bit_search(iter, output)
		}
	}
}

enum OneWireState {
	Reset(ResetState),
	ROMCmd(ROMCommandState),
	FamilyCode(FamilyCodeState),
	SensorID(SensorIDState),
	CRC(CRCState),
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
			OneWireState::FunctionCmd(state) => state.process(iter, output),
			OneWireState::Data(state) => state.process(iter, output),
		}
	}

	pub fn from_rom(iter: &mut OnewireIter, cmd: ROMCmd) -> OneWireState {
		match cmd {
			ROMCmd::SearchROM => OneWireState::FamilyCode(FamilyCodeState { processor: BitProcessor { mode: ProcessorMode::Search } }),
			ROMCmd::SkipROM => OneWireState::FunctionCmd(FunctionCmdState),
			ROMCmd::OverdriveSkipROM => {
				iter.set_timing(Timings::overdrive());
				OneWireState::FunctionCmd(FunctionCmdState)
			},
			ROMCmd::OverdriveMatchROM => {
				iter.set_timing(Timings::overdrive());
				OneWireState::FamilyCode(FamilyCodeState::default())
			},
			_ => OneWireState::FamilyCode(FamilyCodeState::default()),
		}
	}
}

#[derive(Debug)]
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
					start,
					end,
					content: SectionContent::Reset,
				})?;
			}
		};

		let recovery_start = iter.current_time();
		let (start, end, response) = iter.next_response()?;
		match response {
			Err(err) => {
				output.push_err(iter, start, err)?;
				return Some(OneWireState::Reset(ResetState));
			}

			Ok(responded) => {
				output.push(Section {
					start,
					end,
					content: SectionContent::ResetResponse(responded)
				})?;
			}
		};

		let recovery = iter.next_reset_recovery(recovery_start)?;
		match recovery {
			Err(err) => {
				output.push_err(iter, end, err)?;
				return Some(OneWireState::Reset(ResetState));
			}

			Ok(rec_end) => {
				output.push(Section {
					start: end,
					end: rec_end,
					content: SectionContent::ResetRecovery
				})?;
			}
		}

		let reset = iter.next_reset()?;
		iter.discard_last();

		if reset.is_ok() {
			return Some(OneWireState::Reset(ResetState))
		}

		Some(OneWireState::ROMCmd(ROMCommandState))
	}
}

#[derive(Debug)]
struct ROMCommandState;

impl ROMCommandState {
	fn process(&self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		let mut reader = BitReader::lsb(8);

		let start = iter.current_time();
		let (end, mut result) = BitProcessor::default().read_bits(iter, output, &mut reader);

		let value = reader.get_value()?;
		let rom_cmd = ROMCmd::try_from(value as u8);
		match (rom_cmd, result) {
			(Err(code), Err(_)) => {
				output.push(Section { start, end, content: SectionContent::Err(code.to_string())})?;
			},

			(Err(code), Ok(_)) => {
				result = Err(code)
			},

			(Ok(cmd), _) => {
				output.push(Section { start, end, content: SectionContent::ROMCmd(cmd) })?;

				if reader.is_finished() {
					return Some(OneWireState::from_rom(iter, cmd));
				}
			},
		};

		if let Err(err) = result {
			output.push_err(iter, end, err)?;
			return Some(OneWireState::Reset(ResetState));
		};

		None
	}
}

#[derive(Default)]
struct FamilyCodeState {
	processor: BitProcessor,
}
impl FamilyCodeState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		self.processor.process_bits(iter, output, 8, OneWireState::SensorID(SensorIDState {processor: self.processor}), |value| {
			SectionContent::FamilyCode(value as u8)
		})
	}
}

struct SensorIDState {
	processor: BitProcessor,
}
impl SensorIDState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		self.processor.process_bits(iter, output, 48, OneWireState::CRC(CRCState {processor: self.processor}), |value| {
			SectionContent::SensorID(value)
		})
	}
}

struct CRCState {
	processor: BitProcessor
}
impl CRCState {
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		let result = self.processor.process_bits(iter, output, 8, OneWireState::FunctionCmd(FunctionCmdState), |value| {
			SectionContent::CRC(value as u8)
		})?;

		let reset = iter.next_reset()?;
		iter.discard_last();

		if reset.is_ok() {
			return Some(OneWireState::Reset(ResetState));
		}
		Some(result)
	}
}

#[derive(Debug)]
struct FunctionCmdState;
impl FunctionCmdState{
	fn process(self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		BitProcessor::default().process_bits(iter, output, 8, OneWireState::Data(DataState), |value| {
			SectionContent::FunctionCmd(value as u8)
		})
	}
}

#[derive(Debug)]
struct DataState;
impl DataState {
	fn process(&self, iter: &mut OnewireIter, output: &mut OneWireOutput) -> Option<OneWireState> {
		let mut reader = BitReader::lsb(8);

		let start = iter.current_time();
		let (end, result) = BitProcessor::default().read_bits(iter, output, &mut reader);

		if let Some(value) = reader.get_value() {
			let content = SectionContent::Byte(value as u8);
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DecoderOneWire {
	pub onewire_pin: DecoderPin
}

impl Decoder for DecoderOneWire {
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		let mut iter = OnewireIter::from(samples.edge_iter(self.onewire_pin));
		let mut output = OneWireOutput::from(output);

		let mut state = OneWireState::Reset(ResetState);
		if iter.forward_to_reset().is_none() {return Ok(())}

		while let Some(next) = state.process(&mut iter, &mut output) {
			state = next;
		}

		Ok(())
	}

	fn is_valid(&self) -> bool
	{
		true
	}

	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>
	{
		match idx
		{
			0 => Some(("OW", self.onewire_pin)),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::decoder_onewire::{DecoderOneWire, ROMCmd};
	use crate::decoder::{SectionContent};
	use crate::test_utils::{decode_sections, assert_top_layer_eq,
		assert_bit_layer_no_time_overlap, assert_top_layer_no_time_overlap};

	fn decoder() -> DecoderOneWire {
		DecoderOneWire {
			onewire_pin: 0
		}
	}

	#[test]
	fn test_measure_temp() {
		let onewire = decoder();
		let sections = decode_sections("1Wire/OneWireReadROM_MeasureTemp.csv", onewire);

		assert_top_layer_eq(&sections, &[
			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,
			SectionContent::ROMCmd(ROMCmd::ReadROM), SectionContent::FamilyCode(40), SectionContent::SensorID(162321683), SectionContent::CRC(165),

			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,
			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,

			SectionContent::ROMCmd(ROMCmd::MatchROM), SectionContent::FamilyCode(40), SectionContent::SensorID(162321683), SectionContent::CRC(165), 
			SectionContent::FunctionCmd(68),

			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,
			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,

			SectionContent::ROMCmd(ROMCmd::MatchROM), SectionContent::FamilyCode(40), SectionContent::SensorID(162321683), SectionContent::CRC(165), 
			SectionContent::FunctionCmd(190), SectionContent::Byte(107), SectionContent::Byte(1), SectionContent::Byte(75), SectionContent::Byte(70), 
			SectionContent::Byte(127), SectionContent::Byte(255), SectionContent::Byte(5), SectionContent::Byte(16), SectionContent::Byte(73)
		])
	}

	#[test]
	fn test_time_overlap() {
		let onewire = decoder();
		let sections = decode_sections("1Wire/OneWireReadROM_MeasureTemp.csv", onewire);

		assert_bit_layer_no_time_overlap(&sections);
		assert_top_layer_no_time_overlap(&sections);
	}

	#[test]
	fn onewire_search_rom() {
		let onewire = decoder();
		let sections = decode_sections("1Wire/OneWireSearchROM.csv", onewire);

		assert_top_layer_eq(&sections, &[
			// TODO: anpassen mit richtigen werten - diese sind platzhalter
			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,
			SectionContent::ROMCmd(ROMCmd::ReadROM), SectionContent::FamilyCode(0), SectionContent::SensorID(0), SectionContent::CRC(0),
			SectionContent::FunctionCmd(0), SectionContent::Data(0),

			SectionContent::Reset, SectionContent::ResetResponse(true), SectionContent::ResetRecovery,
			SectionContent::ROMCmd(ROMCmd::MatchROM), SectionContent::FamilyCode(0), SectionContent::SensorID(0), SectionContent::CRC(0),
			SectionContent::ROMCmd(ROMCmd::MatchROM), SectionContent::FamilyCode(0), SectionContent::SensorID(0), SectionContent::CRC(0),
			SectionContent::FunctionCmd(0), SectionContent::Data(0),
		])
	}
}
