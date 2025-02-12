use crate::decoder::*;
use crate::SampleBuffer;

pub struct DecoderI2C {
	sda_pin: DecoderPin,
	scl_pin: DecoderPin
}

impl DecoderI2C {
	fn decode(&self, samples: &SampleBuffer) {
		// TODO
	}
}
