use crate::userflash::UserFlash;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DecoderUnion
{
	None,
	Uart(crate::decoder_uart::DecoderUart),
	SPI(crate::decoder_spi::DecoderSPI),
	I2C(crate::decoder_i2c::DecoderI2C),
	OneWire(crate::decoder_onewire::DecoderOneWire),
}

pub struct DecoderStorage
{
}

impl DecoderStorage
{
	pub fn load(_flash: &UserFlash) -> (DecoderUnion, [u8; 8])
	{
		(DecoderUnion::None, [0; 8])
	}

	pub fn save(_flash: &mut UserFlash, _decoder: &DecoderUnion, _a: &[u8])
	{
	}
}
