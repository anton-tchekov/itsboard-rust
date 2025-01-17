pub const TICKS_PER_US: u32 = 90;

pub fn timer_init() {
	// TODO
	/*rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
	tim2.dier.write(|w| w.uie().enabled());
	tim2.psc.write(|w| w.psc().bits(1000));
	tim2.arr.write(|w| w.arr().bits(2000));
	tim2.cr1.write(|w| w.cen().enabled());
	unsafe { cortex_m::peripheral::NVIC::unmask(stm32f429::Interrupt::TIM2) };*/
}

pub fn timer_get() -> u32 {
	// return TIM2->CNT;
	0
}
