use crate::decoder::*;

struct DecoderOneWire {
	onewire_pin: DecoderPin
}

impl DecoderOneWire {
}

fn decode_onewire() {

}

static SETTINGS_ONEWIRE: [DecoderSetting; 1] = [
	DecoderSetting { name: "OneWire Pin" },
];

pub static DECODER_ONEWIRE: ProtocolDecoder = ProtocolDecoder {
	name: "OneWire",
	settings: &SETTINGS_ONEWIRE,
	decode: decode_onewire
};
