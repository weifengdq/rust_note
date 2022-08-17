extern crate serial2;

use serial2::SerialPort;
use std::time::Duration;

fn main() {
    let mut port = SerialPort::open("/dev/pts/7", 115200).unwrap();
    SerialPort::set_read_timeout(&mut port, Duration::from_millis(10000)).unwrap();
    // SerialPort::discard_buffers(&mut port).unwrap();
    SerialPort::discard_input_buffer(&mut port).unwrap();
    SerialPort::discard_output_buffer(&mut port).unwrap();
    let mut buf = [0; 1024];
    let mut count = 0;
    // 发送数据, 引信
    port.write(b"hello").unwrap();
    loop {
        let amt = port.read(&mut buf[..]).unwrap();
        println!(
            "Received {} bytes: {:?}, {}",
            amt,
            &buf[..amt],
            String::from_utf8_lossy(&buf[..amt])
        );
        port.write(&buf[..amt]).unwrap();
        count += 1;
        if count == 10 {
            break;
        }
    }
}