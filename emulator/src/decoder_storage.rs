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
	pub decoder: DecoderUnion
}

const FLASH_OFFSET: usize = 0;

impl DecoderStorage
{
	pub fn load(flash: &UserFlash) -> DecoderUnion
	{
		DecoderUnion::None
	}

	pub fn save(flash: &mut UserFlash, decoder: &DecoderUnion)
	{
	}
}
