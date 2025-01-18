use stm32f4xx_hal::{prelude::*, pac::{Peripherals,SPI1}};
use stm32f4xx_hal::spi::{Polarity, Mode, Phase, Spi};
use stm32f4xx_hal::pac::RCC;
use stm32f4xx_hal::uart::{Config, Serial};
use core::fmt::Write;
use stm32f4xx_hal::pac::*;
use stm32f4xx_hal::serial::Tx;
use core::ptr::{read_volatile, write_volatile};

use crate::delay_us;

const LCD_RST: u32 = 12;
const LCD_DC: u32 = 13;
const LCD_CS: u32 = 14;

pub const TICKS_PER_US: u32 = 90;

pub struct HW {
	pub spi: Spi<SPI1>,
	pub tx: Tx<USART3>
}

const PERIPH_BASE     : u32 = 0x40000000;
const APB2PERIPH_BASE : u32 = PERIPH_BASE + 0x10000;
const SPI1_BASE       : u32 = APB2PERIPH_BASE + 0x3000;
const SPI1_SR         : u32 = SPI1_BASE + 0x08;
const SPI1_DR         : u32 = SPI1_BASE + 0x0C;

const SPI_SR_RXNE     : u32 = 1 << 0;
const SPI_SR_TXE      : u32 = 1 << 1;
const SPI_SR_BSY      : u32 = 1 << 7;

pub fn hw_init() -> HW {
	let dp = Peripherals::take().unwrap();
	let clocks = dp.RCC.constrain().cfgr
		.use_hse(8.MHz())
		.hclk(180.MHz())
		.sysclk(180.MHz())
		.pclk1(45.MHz())
		.pclk2(90.MHz())
		.freeze();

	unsafe {
		(*RCC::ptr()).ahb1enr().modify(|_, w|
			w.gpioaen().enabled()
			.gpioden().enabled()
			.gpioeen().enabled()
			.gpiofen().enabled());

		(*RCC::ptr()).apb2enr().modify(|_, w| w.spi1en().enabled());
	}

	let gpiod = dp.GPIOD.split();

	unsafe {
		/* DC: F13, RST: F12, SDCS: E11 */
		(*GPIOD::ptr()).moder().write(|w| w.bits(0x50005555));
		(*GPIOE::ptr()).moder().write(|w| w.bits(0x00405555));
		(*GPIOF::ptr()).moder().write(|w| w.bits(0x05000000));

		/* BL: D15, CS: D14 */
		(*GPIOD::ptr()).bsrr().write(|w| w.bits(0xFF | (1 << 15)));
		(*GPIOE::ptr()).bsrr().write(|w| w.bits(0xFF | (1 << 11)));

		/* All pullup */
		(*GPIOD::ptr()).pupdr().write(|w| w.bits(0x5555));
		(*GPIOE::ptr()).pupdr().write(|w| w.bits(0x5555));
		(*GPIOF::ptr()).pupdr().write(|w| w.bits(0x5555));
		(*GPIOG::ptr()).pupdr().write(|w| w.bits(0x5555));

		/* All fastest */
		(*GPIOD::ptr()).ospeedr().write(|w| w.bits(0xFFFF));
		(*GPIOE::ptr()).ospeedr().write(|w| w.bits(0xFFFF));
		(*GPIOF::ptr()).ospeedr().write(|w| w.bits(0xFFFF));
		(*GPIOG::ptr()).ospeedr().write(|w| w.bits(0xFFFF));
	}

	unsafe {
		/* Enable Timer Clock */
		(*RCC::ptr()).apb1enr().modify(|_, w| w.tim2en().enabled());

		/* Disable Timer */
		(*TIM2::ptr()).cr1().write(|w| w.bits(0));
		(*TIM2::ptr()).cr2().write(|w| w.bits(0));

		/* No Prescaler (90 MHz) */
		(*TIM2::ptr()).psc().write(|w| w.bits(0));

		/* Auto Reload Register */
		(*TIM2::ptr()).arr().write(|w| w.bits(0xffffffff));

		/* Disable Interrupt */
		(*TIM2::ptr()).dier().write(|w| w.bits(0));

		/* Enable Timer */
		(*TIM2::ptr()).cr1().modify(|_, w| w.cen().enabled());
	}

	let gpioa = dp.GPIOA.split();
	unsafe {
		(*GPIOA::ptr()).ospeedr().write(|w| w.bits(0xFFFF));
	}

	let sclk = gpioa.pa5.into_alternate();
	let miso = gpioa.pa6.into_alternate();
	let mosi = gpioa.pa7.into_alternate();

	let spi = dp.SPI1.spi(
		(sclk, miso, mosi),
		Mode {
			polarity: Polarity::IdleLow,
			phase: Phase::CaptureOnFirstTransition,
		},
		2.MHz(),
		&clocks,
	);

	let tx_pin = gpiod.pd8.into_alternate();
	let mut tx = Serial::tx(
		dp.USART3,
		tx_pin,
		Config::default()
			.baudrate(115200.bps())
			.wordlength_8()
			.parity_none(),
		&clocks,
	).unwrap();

	writeln!(tx, "ITS-Board initialized\n").unwrap();

	HW { spi, tx }
}

pub fn timer_get() -> u32 {
	unsafe { (*TIM2::ptr()).cnt().read().bits() }
}

pub fn spi_ll_xchg(val: u8) -> u8 {
	unsafe {
		delay_us(1);
		while (read_volatile(SPI1_SR as *mut u32) & SPI_SR_TXE) == 0 {}
		write_volatile(SPI1_DR as *mut u32, val.into());
		while (read_volatile(SPI1_SR as *mut u32) & SPI_SR_RXNE) == 0 {}
		while (read_volatile(SPI1_SR as *mut u32) & SPI_SR_BSY) != 0 {}
		let v = read_volatile(SPI1_DR as *mut u32) as u8;
		delay_us(1);
		v
	}
}

pub fn buttons_read() -> u8 {
	(unsafe { (*GPIOF::ptr()).idr().read().bits() } & 0xFF) as u8
}

pub fn blueset(val: u8) {
	unsafe { (*GPIOD::ptr()).bsrr().write(|w| w.bits(val as u32)); }
}

pub fn blueclear(val: u8) {
	unsafe { (*GPIOD::ptr()).bsrr().write(|w| w.bits((val as u32) << 16)); }
}

pub fn yellowset(val: u8) {
	unsafe { (*GPIOE::ptr()).bsrr().write(|w| w.bits(val as u32)); }
}

pub fn yellowclear(val: u8) {
	unsafe { (*GPIOE::ptr()).bsrr().write(|w| w.bits((val as u32) << 16)); }
}

pub fn lcd_rst_0() {
	unsafe { (*GPIOF::ptr()).bsrr().write(|w| w.bits(1 << (LCD_RST + 16))); }
}

pub fn lcd_rst_1() {
	unsafe { (*GPIOF::ptr()).bsrr().write(|w| w.bits(1 << LCD_RST)); }
}

pub fn lcd_dc_0() {
	unsafe { (*GPIOF::ptr()).bsrr().write(|w| w.bits(1 << (LCD_DC + 16))); }
}

pub fn lcd_dc_1() {
	unsafe { (*GPIOF::ptr()).bsrr().write(|w| w.bits(1 << LCD_DC)); }
}

pub fn lcd_cs_0() {
	unsafe { (*GPIOD::ptr()).bsrr().write(|w| w.bits(1 << (LCD_CS + 16))); }
}

pub fn lcd_cs_1() {
	unsafe { (*GPIOD::ptr()).bsrr().write(|w| w.bits(1 << LCD_CS)); }
}
