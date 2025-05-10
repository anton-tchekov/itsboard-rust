use crate::decoder::*;
use crate::sample::*;

pub enum ParitySetting {
	None,
	Even,
	Odd
}

pub enum DataBits {
	Five = 5,
	Six,
	Seven,
	Eight,
	Nine
}

pub enum StopBits {
	One = 1,
	OneAndHalf,
	Two
}
pub struct DecoderUart {
	pub rx_pin: DecoderPin,
	pub tx_pin: DecoderPin,
	pub databits: DataBits,
	pub parity: ParitySetting,
	pub stopbits: StopBits,
	pub baudrate: u32
}

impl Decoder for DecoderUart {
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		Ok(())
		// TODO
	}
}
