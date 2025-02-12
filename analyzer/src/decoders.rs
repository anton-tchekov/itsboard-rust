// Decoder GUI Settings

pub struct DecoderSetting {
	pub name: &'static str,
}

pub struct ProtocolDecoder {
	pub name: &'static str,
	pub settings: &'static [DecoderSetting],
}

static SETTINGS_I2C: [DecoderSetting; 2] = [
	DecoderSetting { name: "SDA Pin" },
	DecoderSetting { name: "SCL Pin" },
];

pub static DECODER_I2C: ProtocolDecoder = ProtocolDecoder {
	name: "I2C",
	settings: &SETTINGS_I2C,
};

static SETTINGS_ONEWIRE: [DecoderSetting; 1] = [
	DecoderSetting { name: "OneWire Pin" },
];

pub static DECODER_ONEWIRE: ProtocolDecoder = ProtocolDecoder {
	name: "OneWire",
	settings: &SETTINGS_ONEWIRE,
};

static SETTINGS_SPI: [DecoderSetting; 4] = [
	DecoderSetting { name: "MISO Pin" },
	DecoderSetting { name: "MOSI Pin" },
	DecoderSetting { name: "SCK Pin" },
	DecoderSetting { name: "CS Pin" },
];

pub static DECODER_SPI: ProtocolDecoder = ProtocolDecoder {
	name: "SPI",
	settings: &SETTINGS_SPI,
};

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
};

static DECODERS: [&ProtocolDecoder; 4] = [
	&DECODER_SPI,
	&DECODER_UART,
	&DECODER_I2C,
	&DECODER_ONEWIRE
];
