use crate::decoder::*;

struct DecoderI2C {
	sda_pin: DecoderPin,
	scl_pin: DecoderPin
}

impl DecoderI2C {
}

fn decode_i2c() {

}

static SETTINGS_I2C: [DecoderSetting; 2] = [
	DecoderSetting { name: "SDA Pin" },
	DecoderSetting { name: "SCL Pin" },
];

pub static DECODER_I2C: ProtocolDecoder = ProtocolDecoder {
	name: "I2C",
	settings: &SETTINGS_I2C,
	decode: decode_i2c
};
