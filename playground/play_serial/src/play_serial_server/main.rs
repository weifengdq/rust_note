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
    let mut serial = serial::open("/dev/pts/6").unwrap();
    serial.configure(&SETTINGS).unwrap();
    serial
        .set_timeout(Duration::from_secs(1_000_000_000))
        .unwrap();
    let mut buf = [0; 1024];
    let mut count = 0;
    loop {
        let amt = serial.read(&mut buf[..]).unwrap();
        println!(
            "Received {} bytes: {:?}, {}",
            amt,
            &buf[..amt],
            String::from_utf8_lossy(&buf[..amt])
        );
        // buf扩展count
        buf[amt] = count + '0' as u8;
        serial.write(&buf[..amt + 1]).unwrap();
        count += 1;
        if count == 10 {
            break;
        }
    }
}
