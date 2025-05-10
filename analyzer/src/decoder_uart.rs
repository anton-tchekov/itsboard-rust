use crate::decoder::*;
use crate::sample::*;

pub enum Parity
{
	None,
	Even,
	Odd
}

pub enum DataBits
{
	Five = 5,
	Six,
	Seven,
	Eight,
	Nine
}

pub enum StopBits
{
	One = 1,
	OneAndHalf,
	Two
}

pub struct DecoderUart
{
	pub rx_pin: DecoderPin,
	pub tx_pin: DecoderPin,
	pub databits: DataBits,
	pub parity: Parity,
	pub stopbits: StopBits,
	pub baudrate: u32
}

impl Decoder for DecoderUart
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
			0 => Some(("RX", self.rx_pin)),
			1 => Some(("TX", self.tx_pin)),
			_ => None,
		}
	}
}
