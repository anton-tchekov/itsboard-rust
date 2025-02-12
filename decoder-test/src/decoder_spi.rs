use crate::decoder::*;
use crate::SampleBuffer;

struct DecoderSPI {
	mosi_pin: DecoderPin,
	miso_pin: DecoderPin,
	sck_pin: DecoderPin,
	cs_pin: DecoderPin
}

impl Decoder for DecoderSPI {
	fn decode(&self, samples: &SampleBuffer) {
		// TODO
	}
}
