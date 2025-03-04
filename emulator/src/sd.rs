struct Sd {
	card_type: u8
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
