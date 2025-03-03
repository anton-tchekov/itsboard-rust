use crate::decoder::*;
use crate::sample::*;

struct DecoderSPI {
	pub mosi_pin: DecoderPin,
	pub miso_pin: DecoderPin,
	pub sck_pin: DecoderPin,
	pub cs_pin: DecoderPin
}

impl Decoder for DecoderSPI {
	fn decode(&self, samples: &SampleBuffer) {
		// TODO
	}
}
