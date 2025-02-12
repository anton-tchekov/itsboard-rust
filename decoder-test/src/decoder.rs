use crate::sample::*;

pub type DecoderPin = i32;

pub struct Section {
	start: usize,
	end: usize,
	color: u16,

}

// Decoder Interface
pub trait Decoder {
	// TODO: Define/Specify output type
	fn decode(&self, samples: &SampleBuffer);
}
