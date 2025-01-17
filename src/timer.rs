use stm32f4::stm32f429::TIM2;
use stm32f4::stm32f429::RCC;

pub const TICKS_PER_US: u32 = 90;

pub fn timer_init() {
	unsafe {
		/* Enable Timer Clock */
		(*RCC::ptr()).apb1enr.modify(|_, w| w.tim2en().enabled());

		/* Disable Timer */
		(*TIM2::ptr()).cr1.write(|w| w.bits(0));
		(*TIM2::ptr()).cr2.write(|w| w.bits(0));

		/* No Prescaler (90 MHz) */
		(*TIM2::ptr()).psc.write(|w| w.bits(0));

		/* Auto Reload Register */
		(*TIM2::ptr()).arr.write(|w| w.bits(0xffffffff));

		/* Disable Interrupt */
		(*TIM2::ptr()).dier.write(|w| w.bits(0));

		/* Enable Timer */
		(*TIM2::ptr()).cr1.modify(|_, w| w.cen().enabled());
	}
}

pub fn timer_get() -> u32 {
	unsafe { (*TIM2::ptr()).cnt.read().bits() }
}
