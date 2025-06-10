use crate::decoder::*;
use crate::sample::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DecoderI2C
{
	pub sda_pin: DecoderPin,
	pub scl_pin: DecoderPin
}

impl Decoder for DecoderI2C
{
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()>
	{
		Ok(())
		// TODO
	}

	fn is_valid(&self) -> bool
	{
		self.sda_pin != self.scl_pin
	}

	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>
	{
		match idx
		{
			0 => Some(("SDA", self.sda_pin)),
			1 => Some(("SCL", self.scl_pin)),
			_ => None,
		}
	}
}
