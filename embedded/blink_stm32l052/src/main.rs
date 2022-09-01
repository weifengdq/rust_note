#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    prelude::*,
    pac,
    rcc::{Config, RccExt},
};
use embedded_time::rate::units::Hertz;

#[entry]
fn main() -> !{
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    // rcc 使用内部 16M RC振荡器
    // let mut rcc = dp.RCC.freeze(Config::hsi16());
    // rcc 使用外部 12M 晶振
    let mut rcc = dp.RCC.freeze(Config::hse(Hertz(12_000_000)));
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb5.into_push_pull_output();
    let mut delay = cp.SYST.delay(rcc.clocks);
    loop {
        led.set_high().unwrap();
        delay.delay_ms(2000_u16);
        led.set_low().unwrap();
        delay.delay_ms(2000_u16);
    }
}
