extern crate serial2;

use serial2::SerialPort;
use std::time::Duration;

fn main() {
    let mut port = SerialPort::open("/dev/pts/6", 115200).unwrap();
    SerialPort::set_read_timeout(&mut port, Duration::from_millis(10000)).unwrap();
    // SerialPort::discard_buffers(&mut port).unwrap();
    SerialPort::discard_input_buffer(&mut port).unwrap();
    SerialPort::discard_output_buffer(&mut port).unwrap();
    let mut buf = [0; 1024];
    let mut count = 0;
    loop {
        let amt = port.read(&mut buf[..]).unwrap();
        println!(
            "Received {} bytes: {:?}, {}",
            amt,
            &buf[..amt],
            String::from_utf8_lossy(&buf[..amt])
        );
        // buf扩展count
        buf[amt] = count + '0' as u8;
        port.write(&buf[..amt + 1]).unwrap();
        count += 1;
        if count == 10 {
            break;
        }
    }
}
