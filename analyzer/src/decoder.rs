use crate::{bytewriter::ByteMutWriter, sample::*};

pub type DecoderPin = i32;

pub const SECBUF_SIZE: usize = 100;

// GUI is responsible for choosing representation, colors, etc.
#[derive(Copy, Clone, Default)]
pub enum SectionContent
{
	// Add more when needed
	#[default]
	Empty,
	Byte(u8),
	Bit(bool),
	StartBit,
	StopBit,
	I2cAddress(u8)
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

pub struct SectionBuffer
{
	pub sections: [Section; SECBUF_SIZE],
	pub len: usize
}

impl SectionBuffer
{
	pub fn push(&mut self, section: Section) -> Result<(), ()>
	{
		if self.len >= self.sections.len()
		{
			return Err(());
		}

		self.sections[self.len] = section;
		self.len += 1;
		Ok(())
	}
}

// Decoder Interface
pub trait Decoder
{
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
