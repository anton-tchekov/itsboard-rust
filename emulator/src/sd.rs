pub const SD_HC              : u8 = 1 << 2;

pub struct Sd {
	pub serial: u32,
	pub capacity: u32,
	pub oem: [u8; 2],
	pub product_name: [u8; 5],
	pub manufacturer: u8,
	pub revision: u8,
	pub manufacturing_year: u8,
	pub manufacturing_month: u8,
	pub card_type: u8
}

impl Sd {
	pub fn init() -> Result<Sd, ()> {
		Err(())
	}

	pub fn read(&self, block: u32, buf: &mut [u8]) -> Result<(), ()> {
		Err(())
	}

	pub fn write(&self, block: u32, buf: &[u8]) -> Result<(), ()> {
		Err(())
	}
}
