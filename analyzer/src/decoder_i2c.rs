use crate::decoder::*;
use crate::sample::*;

pub struct DecoderI2C {
	pub sda_pin: DecoderPin,
	pub scl_pin: DecoderPin
}

impl Decoder for DecoderI2C {
	fn decode(&self, samples: &SampleBuffer) {
		// TODO
	}
}
