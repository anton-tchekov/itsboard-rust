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

	fn num_pins(&self) -> usize {
		4
	}

	fn get_pin(&self, idx: usize) -> Option<DecoderPin>
	{
		match idx
		{
			0 => Some(self.mosi_pin),
			1 => Some(self.miso_pin),
			2 => Some(self.sck_pin),
			3 => Some(self.cs_pin),
			_ => None,
		}
	}

	fn get_pin_name(&self, idx: usize) -> Option<&'static str>
	{
		match idx
		{
			0 => Some("MOSI"),
			1 => Some("MISO"),
			2 => Some("SCK"),
			3 => Some("CS"),
			_ => None,
		}
	}
}
