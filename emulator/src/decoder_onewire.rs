use crate::decoder::*;
use crate::sample::*;

pub struct DecoderOneWire
{
	pub onewire_pin: DecoderPin
}

impl Decoder for DecoderOneWire
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
			0 => Some(("OW", self.onewire_pin)),
			_ => None,
		}
	}
}
