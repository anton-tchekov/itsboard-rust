pub const TICKS_PER_US: u32 = 90;

pub struct HW
{
	pub tx: String
}

impl HW
{
	pub fn new() -> HW
	{
		Self
		{
			tx: "".to_string()
		}
	}
}
