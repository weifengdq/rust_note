use std::net::UdpSocket;
fn main() {
    let addr = "127.0.0.1:8888";
    let socket = UdpSocket::bind(addr).unwrap();
    let mut buf = [0; 1024];
    let mut count = 0;
    // 向7878端口发送数据, 引信
    socket.send_to(b"hello", "127.0.0.1:7878").unwrap();
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!(
            "Received {} bytes from {}: {}",
            amt,
            src,
            String::from_utf8_lossy(&buf[..amt])
        );
        socket.send_to(&buf[..amt], src).unwrap();
        count += 1;
        if count == 10 {
            break;
        }
    }
}
