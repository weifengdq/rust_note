extern crate serial;

use serial::prelude::*;
use std::io::prelude::*;
use std::time::Duration;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud115200,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() {
    let mut serial = serial::open("/dev/pts/7").unwrap();
    serial.configure(&SETTINGS).unwrap();
    serial
        .set_timeout(Duration::from_secs(1_000_000_000))
        .unwrap();
    let mut buf = [0; 1024];
    let mut count = 0;
    // 发送数据, 引信
    serial.write(b"hello").unwrap();
    loop {
        let amt = serial.read(&mut buf[..]).unwrap();
        println!(
            "Received {} bytes: {:?}, {}",
            amt,
            &buf[..amt],
            String::from_utf8_lossy(&buf[..amt])
        );
        serial.write(&buf[..amt]).unwrap();
        count += 1;
        if count == 10 {
            break;
        }
    }
}
