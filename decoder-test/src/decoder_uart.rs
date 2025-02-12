use crate::decoder::*;
use crate::SampleBuffer;

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
	Two
}

pub struct DecoderUart {
	rx_pin: DecoderPin,
	tx_pin: DecoderPin,
	databits: DataBits,
	parity: ParitySetting,
	stopbits: StopBits,
	baudrate: u32
}

impl Decoder for DecoderUart {
	fn decode(&self, samples: &SampleBuffer) {
		// TODO
	}
}
