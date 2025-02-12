use crate::decoder::*;

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

impl DecoderUart {
}

fn decode_uart() {

}

static SETTINGS_UART: [DecoderSetting; 6] = [
	DecoderSetting { name: "RX Pin" },
	DecoderSetting { name: "TX Pin" },
	DecoderSetting { name: "Baudrate" },
	DecoderSetting { name: "Data Bits" },
	DecoderSetting { name: "Parity" },
	DecoderSetting { name: "Stop Bits" }
];

pub static DECODER_UART: ProtocolDecoder = ProtocolDecoder {
	name: "UART",
	settings: &SETTINGS_UART,
	decode: decode_uart
};
