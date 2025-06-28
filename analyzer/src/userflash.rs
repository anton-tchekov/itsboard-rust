use stm32f4xx_hal::{flash::{FlashExt, FlashSector, LockedFlash}};

/* We use the last Sector as our User space for flash */
/* Since theres 2 Banks, we use the Address of Bank 2s last Sector */
const FLASH_START             : usize = 0x08000000;
const FLASH_USER_SPACE_START  : usize = 0x081E0000;

pub struct UserFlash
{
	pub locked_flash: LockedFlash,
	pub sector: FlashSector,
}

impl UserFlash
{
	pub fn new(locked_flash: LockedFlash) -> Self
	{
		/* unwrap cant fail aslong as the constants are correct */
		let sector = locked_flash.sector(FLASH_USER_SPACE_START - FLASH_START).unwrap();

		UserFlash { locked_flash, sector }
	}

	pub fn erase(&mut self)
	{
		let mut unlocked_flash = self.locked_flash.unlocked();
		let _ = unlocked_flash.erase(self.sector.number);
	}

	pub fn write<'a, I>(&mut self, bytes: I)
	where
	I: Iterator<Item = &'a u8>,
	{
		let mut unlocked_flash = self.locked_flash.unlocked();
		let _ = unlocked_flash.program(self.sector.offset, bytes);
	}

	pub fn as_slice(&self) -> &[u8]
	{
		&self.locked_flash.read()[self.sector.offset..]
	}
}
