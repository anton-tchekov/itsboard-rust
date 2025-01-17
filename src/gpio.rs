use stm32f4::stm32f429::*;
use stm32f4::stm32f429::RCC;

pub fn gpio_init() {
	unsafe {
		(*RCC::ptr()).ahb1enr.modify(|_, w| w.gpioden().enabled());
		(*RCC::ptr()).ahb1enr.modify(|_, w| w.gpioeen().enabled());
		(*RCC::ptr()).ahb1enr.modify(|_, w| w.gpiofen().enabled());

		/* DC: F13, RST: F12, SDCS: E11 */
		(*GPIOD::ptr()).moder.write(|w| w.bits(0x50005555));
		(*GPIOE::ptr()).moder.write(|w| w.bits(0x00405555));
		(*GPIOF::ptr()).moder.write(|w| w.bits(0x05000000));

		/* BL: D15, CS: D14 */
		(*GPIOD::ptr()).bsrr.write(|w| w.bits(0xFF | (1 << 15)));
		(*GPIOE::ptr()).bsrr.write(|w| w.bits(0xFF));

		/* All pullup */
		(*GPIOD::ptr()).pupdr.write(|w| w.bits(0x5555));
		(*GPIOE::ptr()).pupdr.write(|w| w.bits(0x5555));
		(*GPIOF::ptr()).pupdr.write(|w| w.bits(0x5555));
		(*GPIOG::ptr()).pupdr.write(|w| w.bits(0x5555));

		/* All fastest */
		(*GPIOD::ptr()).ospeedr.write(|w| w.bits(0xFFFF));
		(*GPIOE::ptr()).ospeedr.write(|w| w.bits(0xFFFF));
		(*GPIOF::ptr()).ospeedr.write(|w| w.bits(0xFFFF));
		(*GPIOG::ptr()).ospeedr.write(|w| w.bits(0xFFFF));
	}
}

pub fn buttons_read() -> u8 {
	(unsafe { (*GPIOF::ptr()).idr.read().bits() } & 0xFF) as u8
}

pub fn blueset(val: u8) {
	unsafe { (*GPIOD::ptr()).bsrr.write(|w| w.bits(val as u32)); }
}

pub fn blueclear(val: u8) {
	unsafe { (*GPIOD::ptr()).bsrr.write(|w| w.bits((val as u32) << 16)); }
}

pub fn yellowset(val: u8) {
	unsafe { (*GPIOE::ptr()).bsrr.write(|w| w.bits(val as u32)); }
}

pub fn yellowclear(val: u8) {
	unsafe { (*GPIOE::ptr()).bsrr.write(|w| w.bits((val as u32) << 16)); }
}
