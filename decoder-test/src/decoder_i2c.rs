use crate::decoder::*;
use crate::sample::*;

pub struct DecoderI2C {
	sda_pin: DecoderPin,
	scl_pin: DecoderPin
}

impl Decoder for DecoderI2C {
	fn decode(&self, samples: &SampleBuffer, range: Range, output: &mut [Section]) -> usize {
		// TODO
		0
	}
}
