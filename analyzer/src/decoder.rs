use crate::sample::SampleBuffer;
use crate::decoder_onewire::rom_cmd::ROMCmd;

pub type DecoderPin = u32;

pub const SECBUF_SIZE: usize = 1000;
pub const TIMER_CLOCK_RATE: u32 = 90_000_000;
pub const TIMER_TICKS_PER_US: u32 = TIMER_CLOCK_RATE / 1_000_000;

// GUI is responsible for choosing representation, colors, etc.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum SectionContent
{
	// Add more when needed
	#[default]
	Empty,
	Byte(u8),
	TxByte(u8),
	RxByte(u8),
	Bit(bool),
	StartBit,
	StopBit,
	RepeatedStart,
	Ack,
	Nak,
	I2cWrite,
	I2cRead,
	I2cAddress(u8),
	ParityBit(bool),
	Err(&'static str),
	Reset,
	CRC(u8),
	ResetResponse(bool),
	Data(u64),
	ResetRecovery,
	FunctionCmd(u8),
	FamilyCode(u8),
	SensorID(u64),
	ROMCmd(ROMCmd),
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Section
{
	// Which time the section starts on
	pub start: u32,

	// Which time the section ends on
	pub end: u32,

	// Arbitrary Content
	pub content: SectionContent
}

pub struct SectionBuffer
{
	pub sections: [Section; SECBUF_SIZE],
	pub len: usize
}

impl SectionBuffer
{
	pub fn clear(&mut self)
	{
		self.len = 0;
	}

	pub fn push(&mut self, section: Section)
	{
		if self.is_full()
		{
			return;
		}

		self.sections[self.len] = section;
		self.len += 1;
	}

	pub fn is_full(&self) -> bool
	{
		self.len >= self.sections.len()
	}

	pub fn iter(&self) -> SectionBufferIter
	{
		SectionBufferIter {
			buffer: self,
			index: 0,
		}
	}

	// start: Timstamp of window start
	// Returns sample index
	pub fn find_view(&self, start: u32, end: u32) -> (usize, usize)
	{
		let mut first = None;
		let mut last = 0;
		for (i, cur_sec) in self.sections.iter().take(self.len).enumerate()
		{
			if cur_sec.start >= start || cur_sec.end <= end ||
				(cur_sec.start <= start && cur_sec.end >= end)
			{
				if first.is_none()
				{
					first = Some(i);
				}

				last = i + 1;
			}
		}

		(first.unwrap_or(0), last)
	}
}

pub struct SectionBufferIter<'a>
{
	buffer: &'a SectionBuffer,
	index: usize,
}

impl<'a> Iterator for SectionBufferIter<'a>
{
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
pub trait Decoder
{
	// Decode a SampleBuffer
	// output is a SectionBuffer
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()>;

	// Is the current configuration valid?
	fn is_valid(&self) -> bool;

	// Gives back the DecoderPin at index, None if no such index exists
	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>;
}

pub fn pin_duplicates(arr: &[DecoderPin]) -> bool
{
	let len = arr.len();
	if len < 2
	{
		return false;
	}

	let mut seen = [false; 16];
	for &item in arr.iter()
	{
		let index = item as usize;
		if seen[index]
		{
			return true;
		}

		seen[index] = true;
	}

	false
}
