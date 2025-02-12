pub type DecoderPin = i32;

pub struct DecoderSetting {
	pub name: &'static str,
}

pub struct ProtocolDecoder {
	pub name: &'static str,
	pub settings: &'static [DecoderSetting],
	pub decode: fn()
}
