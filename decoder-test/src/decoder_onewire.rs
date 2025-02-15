use crate::decoder::*;
use crate::sample::*;

pub struct DecoderOneWire {
	onewire_pin: DecoderPin
}

impl Decoder for DecoderOneWire {
	fn decode(&self, samples: &SampleBuffer, range: Range, output: &mut [Section]) -> usize {
		// TODO
		0
	}
}
