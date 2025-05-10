use crate::decoder::*;
use crate::sample::*;

pub struct DecoderI2C {
	pub sda_pin: DecoderPin,
	pub scl_pin: DecoderPin
}

impl Decoder for DecoderI2C {
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		Ok(())
		// TODO
	}
	
	fn num_pins(&self) -> usize {
		2
	}
	
	fn get_pin(&self, idx: usize) -> Option<DecoderPin> {
		match idx
		{
			0 => Some(self.sda_pin),
			1 => Some(self.scl_pin),
			_ => None,
		}
	}
	
	fn get_pin_name(&self, idx: usize) -> Option<&'static str> {
		match idx
		{
			0 => Some("SDA"),
			1 => Some("SCL"),
			_ => None,
		}
	}
}
