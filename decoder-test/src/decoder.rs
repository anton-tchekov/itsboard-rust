use crate::sample::*;

pub type DecoderPin = i32;

pub const SECBUF_SIZE: usize = 100;
pub const TIMER_CLOCK_RATE: u32 = 90_000_000;

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
			start: bit.start_time,
			end: bit.end_time,
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

		let item = &self.buffer.sections[self.index];
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