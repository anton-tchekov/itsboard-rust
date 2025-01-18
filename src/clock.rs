use stm32f4xx_hal as hal;
use hal::{prelude::*, pac::{Peripherals}};
pub fn clock_init() {

	let device_periphs = Peripherals::take().unwrap();

    let clocks = device_periphs.RCC.constrain().cfgr
        .use_hse(8.MHz())
        .hclk(180.MHz())
        .sysclk(180.MHz())
        .pclk1(45.MHz())
        .pclk2(90.MHz())
        .freeze();
}
