use crate::sample::*;

pub type DecoderPin = i32;

// GUI is responsible for choosing representation, colors, etc.
#[derive(Copy, Clone, Default)]
pub enum SectionContent {
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
pub struct Section {
	// Which sample the section starts on
	pub start: usize,

	// Length of the section in samples
	pub len: usize,

	// Arbitrary Content
	pub content: SectionContent
}

// Range of samples
pub struct Range {
	// Sample the range starts on
	pub start: usize,

	// Size of the range
	pub len: usize
}

// Decoder Interface
pub trait Decoder {
	// Decode a part of a SampleBuffer
	// range is which part of the SampleBuffer is currently visible/to be decoded
	// output is a fixed array (to avoid heap allocation) in which the Sections are to be placed
	// Returns the number of Sections that were written to output
	fn decode(&self, samples: &SampleBuffer, range: Range, output: &mut [Section]) -> usize;
}
