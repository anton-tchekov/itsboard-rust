// Busy-loop delay functions

use crate::hw::*;

pub fn delay_us(us: u32)
{
	let interval = us * TICKS_PER_US;
	let start = timer_get();
	while timer_get() - start < interval {}
}

pub fn delay_ms(ms: u32)
{
	delay_us(1000 * ms);
}
