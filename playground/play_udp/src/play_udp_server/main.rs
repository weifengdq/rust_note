use std::net::UdpSocket;
fn main() {
    let addr = "127.0.0.1:7878";
    let socket = UdpSocket::bind(addr).unwrap();
    let mut buf = [0; 1024];
    let mut count = 0;
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!(
            "Received {} bytes from {}: {:?}, {}",
            amt,
            src,
            &buf[..amt],
            String::from_utf8_lossy(&buf[..amt])
        );
        // buf扩展count
        buf[amt] = count + '0' as u8;
        socket.send_to(&buf[..amt + 1], src).unwrap();
        count += 1;
        if count == 10 {
            break;
        }
    }
}
