use crate::decoder::*;
use crate::sample::*;

static BAUDRATES: &'static [i32] = &[
	300,
	600,
	1200,
	1800,
	2400,
	4800,
	9600,
	19200,
	38400,
	57600,
	115200
];

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
	fn decode(&self, samples: &SampleBuffer) {
		// TODO
	}
}
