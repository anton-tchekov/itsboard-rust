#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4::stm32f429::{self, interrupt};

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

    /*rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
    tim2.dier.write(|w| w.uie().enabled());
    tim2.psc.write(|w| w.psc().bits(1000));
    tim2.arr.write(|w| w.arr().bits(2000));
    tim2.cr1.write(|w| w.cen().enabled());
    unsafe { cortex_m::peripheral::NVIC::unmask(stm32f429::Interrupt::TIM2) };*/
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

        // cortex_m::asm::wfi();
    }
}

/*

#[interrupt]
fn TIM2() {
    unsafe { (*stm32f429::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit()) };
    let ptr = stm32f429::GPIOD::ptr();
    unsafe {
        if (*ptr).odr.read().odr0().is_high() {
            (*ptr).bsrr.write(|w| w.br0().set_bit());
        } else {
            (*ptr).bsrr.write(|w| w.bs1().set_bit());
        }
    }
}
*/