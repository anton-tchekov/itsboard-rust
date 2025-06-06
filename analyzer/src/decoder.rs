use stm32f4xx_hal::{flash::{FlashExt, LockedFlash}, pac::FLASH};

use crate::{sample::*, userflash::UserFlash};

pub type DecoderPin = i32;

pub const SECBUF_SIZE: usize = 100;
pub const TIMER_CLOCK_RATE: u32 = 90_000_000;


// GUI is responsible for choosing representation, colors, etc.
#[derive(Copy, Clone, Default)]
pub enum SectionContent
{
	// Add more when needed
	#[default]
	Empty,
	Byte(u8),
	TxByte(u8),
	RxByte(u8),
	Bit(bool),
	StartBit,
	StopBit,
	I2cAddress(u8),
	Err(&'static str),
	ParityBit(bool),
	Word(u32),
}

#[derive(Copy, Clone, Default)]
pub struct Section
{
	// Which time the section starts on
	pub start: u32,

	// Which time the section ends on
	pub end: u32,

	// Arbitrary Content
	pub content: SectionContent
}

impl Section {
	pub fn from_bit(bit: &BitData, content: SectionContent) -> Self {
		Section {
			start: bit.start_time,
			end: bit.end_time,
			content
		}
	}
}

pub struct SectionBuffer
{
	pub sections: [Section; SECBUF_SIZE],
	pub len: usize
}

impl SectionBuffer
{
	pub fn clear(&mut self)
	{
		self.len = 0;
	}

	pub fn push(&mut self, section: Section) -> Result<(), ()>
	{
		if self.len >= self.sections.len()
		{
			return Err(());
		}

		self.sections[self.len] = section;
		self.len += 1;
		Ok(())
	}

	// start: Timstamp of window start
	// Returns sample index
	pub fn find_view(&self, start: u32, end: u32) -> (usize, usize)
	{
		let mut first = None;
		let mut last = 0;
		for (i, cur_sec) in self.sections.iter().take(self.len).enumerate()
		{
			if cur_sec.start >= start || cur_sec.end <= end ||
				(cur_sec.start <= start && cur_sec.end >= end)
			{
				if first.is_none()
				{
					first = Some(i);
				}

				last = i + 1;
			}
		}

		(first.unwrap_or(0), last)
	}
}

// Decoder Interface
pub trait Decoder
{
	// Decode a SampleBuffer
	// output is a SectionBuffer
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()>;

	// Gives back the DecoderPin at index, None if no such index exists
	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>;
}

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
	pub decoder: DecoderUnion,
	user_flash: UserFlash,
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