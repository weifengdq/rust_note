#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    prelude::*,
    pac,
    rcc::{Config, RccExt},
    serial,
};
use embedded_time::rate::{Hertz, Baud};
use nb::block;

#[entry]
fn main() -> !{
    let dp = pac::Peripherals::take().unwrap();
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
    let (mut tx, mut rx) = serial.split();

    loop {
        // 读一个字节然后立即发回去
        let received = block!(rx.read()).unwrap();
        block!(tx.write(received)).ok();
    }
}
