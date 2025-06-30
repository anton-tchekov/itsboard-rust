use std::{thread, time};

pub fn delay_ms(ms: u32)
{
	thread::sleep(time::Duration::from_millis(ms.into()));
}
