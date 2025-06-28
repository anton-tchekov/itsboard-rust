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
	pub fn load(flash: &UserFlash) -> (DecoderUnion, [u8; 8])
	{
		/* Get the Length of the Decoder */
		const MAX_LEN: usize = size_of::<DecoderUnion>();
		let mut bytes: [u8; MAX_LEN] = [0; MAX_LEN];
		let mut sels: [u8; 8] = [0; 8];

		/* Read Data from flash and serialize */
		bytes.copy_from_slice(&flash.as_slice()[0..MAX_LEN]);
		sels.copy_from_slice(&flash.as_slice()[MAX_LEN..(MAX_LEN+8)]);

		(match postcard::from_bytes(&bytes)
		{
			Ok(x) => x,
			Err(_) => DecoderUnion::None,
		}, sels)
	}

	/* POTENTIAL TODO: Save the rest of the sector before erasing and rewrite */
	pub fn save(flash: &mut UserFlash, decoder: &DecoderUnion, sels: &[u8])
	{
		/* Get the Length of the Decoder */
		const MAX_LEN: usize = size_of::<DecoderUnion>();
		let mut bytes: [u8; MAX_LEN+8] = [0; MAX_LEN+8];

		bytes[MAX_LEN..MAX_LEN+8].copy_from_slice(sels);

		/* Convert Decoder to Bytes and write them into the Flash */
		postcard::to_slice(&decoder, &mut bytes).unwrap();
		flash.erase();
		flash.write(bytes.iter());
	}
}
