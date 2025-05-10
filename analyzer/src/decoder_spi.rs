use crate::decoder::*;
use crate::sample::*;

pub struct DecoderSPI
{
	pub mosi_pin: DecoderPin,
	pub miso_pin: DecoderPin,
	pub sck_pin: DecoderPin,
	pub cs_pin: DecoderPin
}

impl Decoder for DecoderSPI
{
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()>
	{
		Ok(())
		// TODO
	}

	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>
	{
		match idx
		{
			0 => Some(("MOSI", self.mosi_pin)),
			1 => Some(("MISO", self.miso_pin)),
			2 => Some(("SCK", self.sck_pin)),
			3 => Some(("CS", self.cs_pin)),
			_ => None,
		}
	}
}
