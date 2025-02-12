use crate::decoder::*;

struct DecoderSPI {
	mosi_pin: DecoderPin,
	miso_pin: DecoderPin,
	sck_pin: DecoderPin,
	cs_pin: DecoderPin
}

impl DecoderSPI {
}

fn decode_spi() {
}

static SETTINGS_SPI: [DecoderSetting; 4] = [
	DecoderSetting { name: "MISO Pin" },
	DecoderSetting { name: "MOSI Pin" },
	DecoderSetting { name: "SCK Pin" },
	DecoderSetting { name: "CS Pin" },
];

pub static DECODER_SPI: ProtocolDecoder = ProtocolDecoder {
	name: "SPI",
	settings: &SETTINGS_SPI,
	decode: decode_spi
};
