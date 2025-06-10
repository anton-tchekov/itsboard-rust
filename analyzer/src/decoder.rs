use crate::sample::*;

pub type DecoderPin = u32;

pub const SECBUF_SIZE: usize = 1000;
pub const TIMER_CLOCK_RATE: u32 = 90_000_000;

// GUI is responsible for choosing representation, colors, etc.
#[derive(Copy, Clone, Default)]
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
	I2cAddress(u8),
	Err(&'static str),
	ParityBit(bool),
	Word(u32),
}

#[derive(Copy, Clone, Default)]
pub struct Section
{
	// Which time the section starts on
	pub start: u32,

	// Which time the section ends on
	pub end: u32,

	// Arbitrary Content
	pub content: SectionContent
}

impl Section
{
	pub fn from_bit(bit: &BitData, content: SectionContent) -> Self
	{
		Section
		{
			start: bit.start_time,
			end: bit.end_time,
			content
		}
	}
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
		if self.len >= self.sections.len()
		{
			return;
		}

		self.sections[self.len] = section;
		self.len += 1;
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
