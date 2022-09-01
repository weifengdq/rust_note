#![no_std]
#![no_main]

use panic_halt as _;
use core::{fmt::Write as _};
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    prelude::*,
    pac,
    rcc::{Config, RccExt},
    serial,
};
use embedded_time::rate::{Hertz, Baud};

#[entry]
fn main() -> !{
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hse(Hertz(12_000_000)));

    let gpioa = dp.GPIOA.split(&mut rcc);
    let tx_pin = gpioa.pa9;
    let rx_pin = gpioa.pa10;
    let serial = dp
        .USART1
        .usart(
            tx_pin, 
            rx_pin, 
            serial::Config::default().baudrate(Baud(115_200)), 
            &mut rcc
        )
        .unwrap();
    let (mut tx, _) = serial.split();

    let mut delay = cp.SYST.delay(rcc.clocks);

    let mut cnt: u8 = 0;

    loop {
        write!(tx, "Hello, world! {}\r\n", cnt).ok();
        cnt += 1;
        delay.delay_ms(1000_u16);
    }
}
