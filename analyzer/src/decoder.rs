use crate::SampleBuffer;

pub type DecoderPin = i32;

// Decoder Interface
pub trait Decoder {
	// TODO: Define/Specify output type
	fn decode(&self, samples: &SampleBuffer);
}
