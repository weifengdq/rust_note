use std::{sync::mpsc, thread, time};

fn main() {
    // 多生产者-单消费者
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            tx.send(i).unwrap();
            thread::sleep(time::Duration::from_millis(1000-i*100));
            tx.send(i+100).unwrap();
        });
    }
    for _ in 0..20 {
        let j = rx.recv().unwrap();
        println!("Got: {}", j);
    }
}
