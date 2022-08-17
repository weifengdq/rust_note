use std::{thread, time};

fn main() {
    // 线程1
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(time::Duration::from_millis(100));
        }
    });
    // 主线程
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(time::Duration::from_millis(100));
    }
    // 等待线程1结束
    handle.join().unwrap();
}
