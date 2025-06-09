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
		/* Get the Length of the Decoder */
		const MAX_LEN: usize = size_of::<DecoderUnion>();
		let mut bytes: [u8; MAX_LEN] = [0; MAX_LEN];

		/* Read Data from flash and serialize */
		bytes.copy_from_slice(&flash.as_slice()[FLASH_OFFSET..FLASH_OFFSET + MAX_LEN]);

		match postcard::from_bytes(&bytes)
		{
			Ok(x) => x,
			Err(_) => DecoderUnion::None,
		}
	}

	/* POTENTIAL TODO: Save the rest of the sector before erasing and rewrite */
	pub fn save(flash: &mut UserFlash, decoder: &DecoderUnion)
	{
		/* Get the Length of the Decoder */
		const MAX_LEN: usize = size_of::<DecoderUnion>();
		let mut bytes: [u8; MAX_LEN] = [0; MAX_LEN];

		/* Convert Decoder to Bytes and write them into the Flash */
		postcard::to_slice(&decoder, &mut bytes).unwrap();
		flash.erase();
		flash.write(FLASH_OFFSET, bytes.iter());
	}
}
