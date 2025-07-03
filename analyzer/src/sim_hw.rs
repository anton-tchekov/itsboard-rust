use crate::userflash::UserFlash;

pub const TICKS_PER_US: u32 = 90;

pub struct HW
{
	pub tx: String,
	pub user_flash: UserFlash
}

impl HW
{
	pub fn new() -> HW
	{
		Self
		{
			tx: "".to_string(),
			user_flash: UserFlash {}
		}
	}
}
