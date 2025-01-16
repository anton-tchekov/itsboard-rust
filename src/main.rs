#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4::stm32f429::{self, interrupt};

#[cortex_m_rt::entry]
fn start() -> ! {
    let device_peripherals = stm32f429::Peripherals::take().unwrap();

    let gpiod = &device_peripherals.GPIOD;
    let rcc = &device_peripherals.RCC;
    let tim2 = &device_peripherals.TIM2;

    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    gpiod.moder.modify(|_, w| w.moder8().output());

    rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
    tim2.dier.write(|w| w.uie().enabled());
    tim2.psc.write(|w| w.psc().bits(1000));
    tim2.arr.write(|w| w.arr().bits(2000));
    tim2.cr1.write(|w| w.cen().enabled());

    unsafe { cortex_m::peripheral::NVIC::unmask(stm32f429::Interrupt::TIM2) };
    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn TIM2() {
    unsafe { (*stm32f429::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit()) };
    let ptr = stm32f429::GPIOD::ptr();
    unsafe {
        if (*ptr).odr.read().odr8().is_high() {
            (*ptr).bsrr.write(|w| w.br8().set_bit());
        } else {
            (*ptr).bsrr.write(|w| w.bs8().set_bit());
        }
    }
}
