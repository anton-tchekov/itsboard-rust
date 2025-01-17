#![no_std]
#![no_main]

mod uart;
mod font;
mod spi;
mod timer;
mod delay;
mod lcd_ll;
mod lcd;
mod terminus16;
mod terminus16_bold;

use crate::terminus16::*;
use crate::terminus16_bold::*;
use crate::lcd::*;
use crate::font::*;
use crate::uart::*;
use crate::timer::*;
use crate::spi::*;

use panic_halt as _;
use stm32f4::stm32f429::{self};

#[cortex_m_rt::entry]
fn start() -> ! {
	let device_peripherals = stm32f429::Peripherals::take().unwrap();

	let gpioe = &device_peripherals.GPIOE;
	let gpiod = &device_peripherals.GPIOD;
	let rcc = &device_peripherals.RCC;
	let tim2 = &device_peripherals.TIM2;

	rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
	rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());

	gpioe.moder.modify(|_, w| w.moder0().output());
	gpioe.moder.modify(|_, w| w.moder1().output());
	gpioe.moder.modify(|_, w| w.moder2().output());
	gpioe.moder.modify(|_, w| w.moder3().output());
	gpioe.moder.modify(|_, w| w.moder4().output());
	gpioe.moder.modify(|_, w| w.moder5().output());
	gpioe.moder.modify(|_, w| w.moder6().output());
	gpioe.moder.modify(|_, w| w.moder7().output());

	gpiod.moder.modify(|_, w| w.moder0().output());
	gpiod.moder.modify(|_, w| w.moder1().output());
	gpiod.moder.modify(|_, w| w.moder2().output());
	gpiod.moder.modify(|_, w| w.moder3().output());
	gpiod.moder.modify(|_, w| w.moder4().output());
	gpiod.moder.modify(|_, w| w.moder5().output());
	gpiod.moder.modify(|_, w| w.moder6().output());
	gpiod.moder.modify(|_, w| w.moder7().output());

	gpiod.bsrr.write(|w| w.br0().set_bit());
	gpiod.bsrr.write(|w| w.br1().set_bit());
	gpiod.bsrr.write(|w| w.br2().set_bit());
	gpiod.bsrr.write(|w| w.br3().set_bit());
	gpiod.bsrr.write(|w| w.br4().set_bit());
	gpiod.bsrr.write(|w| w.br5().set_bit());
	gpiod.bsrr.write(|w| w.br6().set_bit());
	gpiod.bsrr.write(|w| w.br7().set_bit());

	timer_init();
	spi_ll_init();
	uart_init(115200);
	uart_tx_str("Hello World");

	lcd_init(lcd_color(0, 0, 0));
	lcd_rect(10, 10, 100, 100, lcd_color(255, 0, 0));
	font_str(200, 200, "Hello World",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16_BOLD);

	font_str(200, 216, "This is a test",
		lcd_color(255, 255, 255), lcd_color(0, 0, 0), &TERMINUS16);

	loop {
		gpiod.bsrr.write(|w| w.bs0().set_bit());
		let mut i = 0;
		while i < 1000000 {
			cortex_m::asm::nop();
			i += 1;
		}

		gpiod.bsrr.write(|w| w.br0().set_bit());

		i = 0;
		while i < 1000000 {
			cortex_m::asm::nop();
			i += 1;
		}
	}
}
