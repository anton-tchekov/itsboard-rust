use crate::sample::*;

pub type DecoderPin = u32;

pub const SECBUF_SIZE: usize = 100;
pub const TIMER_CLOCK_RATE: u32 = 90_000_000;
pub const TIMER_TICKS_PER_US: u32 = TIMER_CLOCK_RATE / 1_000_000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ROMCommand {
	ReadROM,
	SkipROM,
	MatchROM,
	SearchROM,
	OverdriveSkipROM,
	OverdriveMatchROM
}

impl TryFrom<u8> for ROMCommand {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x33 => Ok(ROMCommand::ReadROM),
			0xCC => Ok(ROMCommand::SkipROM),
			0x55 => Ok(ROMCommand::MatchROM),
			0xF0 => Ok(ROMCommand::SearchROM),
			0x3C => Ok(ROMCommand::OverdriveSkipROM),
			0x69 => Ok(ROMCommand::OverdriveMatchROM),
			_ => Err("Invalid ROM command"),
		}
	}
}

impl ROMCommand {
	fn to_string(&self) -> &'static str {
		match self {
			ROMCommand::ReadROM => "Read ROM",
			ROMCommand::SkipROM => "Skip ROM",
			ROMCommand::MatchROM => "Match ROM",
			ROMCommand::SearchROM => "Search ROM",
			ROMCommand::OverdriveSkipROM => "Overdrive Skip ROM",
			ROMCommand::OverdriveMatchROM => "Overdrive Match ROM",
		}
	}
}

// GUI is responsible for choosing representation, colors, etc.
// FIXME: Debug and PartialEq only needed for testing
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum SectionContent {
	// Add more when needed
	#[default]
	Empty,
	Byte(u8),
	// TODO: use u16 instead of u32
	Word(u32),
	Bit(bool),
	ParityBit(bool),
	StartBit,
	StopBit,
	I2cAddress(u8),
	Reset,
	CRC(u8),
	ResetResponse(bool),
	Data(u64),
	ResetRecovery,
	FunctionCmd(u8),
	FamilyCode(u8),
	SensorID(u64),
	ROMCmd(ROMCommand),
	Err(&'static str)
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Section {
	// Which time the section starts on
	pub start: u32,

	// Which time the section ends on
	pub end: u32,

	// Arbitrary Content
	pub content: SectionContent
}

impl Section {
	pub fn from_bit(bit: &BitData, content: SectionContent) -> Self {
		Section {
			start: bit.start,
			end: bit.end,
			content
		}
	}
}

pub struct SectionBuffer {
	pub sections: [Section; SECBUF_SIZE],
	pub len: usize
}

impl SectionBuffer {
	pub fn push(&mut self, section: Section) -> Result<(), ()> {
		if self.len >= self.sections.len() {
			return Err(());
		}

		self.sections[self.len] = section;
		self.len += 1;
		Ok(())
	}

	pub fn iter(&self) -> SectionBufferIter {
		SectionBufferIter {
			buffer: self,
			index: 0,
		}
	}
}

pub struct SectionBufferIter<'a> {
	buffer: &'a SectionBuffer,
	index: usize,
}

impl<'a> Iterator for SectionBufferIter<'a> {
	type Item = &'a Section;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.buffer.len {
			return None;
		}

		let item: &Section = &self.buffer.sections[self.index];
		self.index += 1;

		Some(item)
	}
}

// Decoder Interface
pub trait Decoder {
	// Decode a SampleBuffer
	// output is a SectionBuffer
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()>;

	// Gives back the number of pins that the decoder uses
	fn num_pins(&self) -> usize;

	// Gives back the DecoderPin at index, None if no such index exists
	fn get_pin(&self, idx: usize) -> Option<DecoderPin>;

	// Gives back the Name of a Pin at the given index
	fn get_pin_name(&self, idx: usize) -> Option<&'static str>;
}