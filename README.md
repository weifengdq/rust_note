# rust_note

# RUST 笔记

[TOC]

## 测试环境

```bash
WSL2
Ubuntu 22.04
Kernel 5.15.57.1
cargo  1.63.0 (不定期更新)
```

## RUST 安装

```bash
# 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 版本检查
# 第一句可以放到 ~/.bashrc里面
source "$HOME/.cargo/env"
cargo --version
rustc --version

# 更新
rustup update
```

## VSCode 配置

插件:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

## 新工程

```bash
cargo new playground
cd playground
cargo run
```

可执行文件的路径为

```bash
playground/target/debug/playground

# 查看文件大小发现有3.xM, 应该是打包进了rust的运行时之类的
$ ls -lh target/debug/playground

# 编译release版本, 会小一点
$ cargo build --release
$ ls -lh target/release/playground

# 还有其它减少体积的方式, 可自行搜索

# 清理
$ cargo clean
```

## args 命令行参数传入

C语言编程时, 经常可以看到main函数

```c
int main(int argc, char *argv[])
```

这里argc是参数个数, argv[]是参数的字符串数组, argv[0], argv[1]... 这些就是通过命令行传入的参数.  

rust 中也有类似的, 如 `std::env::args`, 用法举例

```rust
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

测试

```bash
$ cargo run 1 a 127.0.0.1
   Compiling playground v0.1.0 (/home/karoto/git/rust_note/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/playground 1 a 127.0.0.1`
["target/debug/playground", "1", "a", "127.0.0.1"]
```

或者是for循环的方式, 一个一个打出来

```bash
    for i in env::args() {
        println!("{}", i);
    }
```

参考 [args in std::env - Rust (rust-lang.org)](https://doc.rust-lang.org/std/env/fn.args.html)

## 多文件 mod 和 include

现在有两个文件

```bash
$ tree src
src
├── main.rs
└── test_args.rs

# test_args 中要被调用的函数前面加上pub
# 这里main改成其它名字比较好
$ cat src/test_args.rs 
use std::env;
pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

# 用mod把文件引入, mod是modules缩写, 可以把一个文件看成一个module
$ cat src/main.rs 
mod test_args;
fn main() {
    test_args::main();
}

# 运行
$ cargo run 1 a 
   Compiling playground v0.1.0 (/home/karoto/git/rust_note/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/playground 1 a`
["target/debug/playground", "1", "a"]
```

还有一种 include 的方式

```bash
# test_args 中的main要改成其它名字
$ cat src/test_args.rs 
use std::env;
pub fn print_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

# include 引入文件/文件夹, 直接用里面pub过的fn
$ cat src/main.rs
include!("../src/test_args.rs");
fn main() {
    print_args();
}
```

参考 [rust多文件/文件夹及模块管理 - 简书 (jianshu.com)](https://www.jianshu.com/p/b6ae79bc478a)

## 多bin 与 workspace

比如server/client, pub/sub... 参考 [关于rust：如何使用Cargo构建多个二进制文件](https://www.codenong.com/36604010/):

- `Cargo.toml`文件中指定多个 `[[bin]]`
- 或者` [workspace]` 搭配多个工程, 每个工程都有一个 `Cargo.toml`
- ...

个人开发怎么都行, 多人开发第二种应该会好一点, 或者两种组合使用

第一种方式举例

```bash
$ tree
.
├── Cargo.toml
└── src
    ├── play_args
    │   ├── main.rs
    │   └── test_args.rs
    └── play_hello
        └── main.rs

# 在toml里面写两个[[bin]], 填入可执行文件名字和路径
$ cat Cargo.toml 
[package]
name = "playground"
version = "0.1.0"
edition = "2021"

[dependencies]

[[bin]]
name = "play_hello"
path = "src/play_hello/main.rs"

[[bin]]
name = "play_args"
path = "src/play_args/main.rs"

$ cargo build
$ ./target/debug/play_args  1 2 3
["./target/debug/play_args", "1", "2", "3"]
$ ./target/debug/play_hello 
Hello, world!
```

第二种方式举例

```bash
# play_args 和 play_hello 是直接 cargo new 出来的
$ tree
.
├── Cargo.toml
├── play_args
│   ├── Cargo.toml
│   └── src
│       ├── main.rs
│       └── test_args.rs
└── play_hello
    ├── Cargo.toml
    └── src
        └── main.rs
        
# 顶层手写一个Cargo.toml, 内容如下
$ cat Cargo.toml 
[workspace]
members = ["play_args", "play_hello"]

$ cargo build
   Compiling play_args v0.1.0 (/home/karoto/git/rust_note/playground/play_args)
   Compiling play_hello v0.1.0 (/home/karoto/git/rust_note/playground/play_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
$ ./target/debug/play_args 1 a
["./target/debug/play_args", "1", "a"]
$ ./target/debug/play_hello 
Hello, world!
```

## println

[println in std - Rust (rust-lang.org)](https://doc.rust-lang.org/std/macro.println.html)

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // 整数
    let a = 1;
    println!("{}", a);
    // 字符
    let b = 'a';
    println!("{}", b);
    // 字符串
    let s = "hello";
    println!("{}", s);
    // 布尔值
    let b = true;
    println!("{}", b);
    // 浮点数
    let f = 3.1415926;
    println!("{}", f);
    // 数组
    let arr = [10, 11, 13, 24, 50];
    println!("{:?}", arr);
    // 元组
    let tup = (1, "hello", true);
    println!("{:?}", tup);
    // 函数
    let f = add;
    println!("11 + 22 = {}", f(11, 22));
    // =================
    let c = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("{}", c);
    // lambda表达式
    let lambda = |x: i32, y: i32| x + y;
    println!("{}", lambda(3, 4));
    // 打印匿名函数
    let anon = |x: i32, y: i32| x + y;
    println!("{:?}", anon(5, 6));
    // 匿名函数作为参数传递
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(9));
    // 匿名函数作为返回值
    let f = || {
        println!("hi");
    };
    f();
    // 打印十六进制, 不够2位补0
    let hex = |x: i32| -> String {
        format!("{:02x}", x)
    };
    println!("{}", hex(255));
    // 遍历数组, 打印十六进制
    for i in arr.iter() {
        println!("{}", hex(*i));
    }
    // 排序
    let mut v = vec![10, 30, 11, 20, 4, 330, 21, 110, 5, 10, 1];
    v.sort();
    println!("{:?}", v);
}
```

运行结果

```bash
1
a
hello
true
3.1415926
[10, 11, 13, 24, 50]
(1, "hello", true)
11 + 22 = 33
3
7
11
10
hi
ff
0a
0b
0d
18
32
[1, 4, 5, 10, 10, 11, 20, 21, 30, 110, 330]
```

## thread

[std::thread - Rust (rust-lang.org)](https://doc.rust-lang.org/std/thread/)

```rust
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
```

运行结果

```bash
$ ./target/debug/play_thread 
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 4 from the main thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

## 多生产-单消费

```rust
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

```

运行

```bash
$ ./target/debug/play_mpsc 
Got: 0
Got: 2
Got: 3
Got: 1
Got: 4
Got: 5
Got: 6
Got: 7
Got: 9
Got: 8
Got: 109
Got: 108
Got: 107
Got: 106
Got: 105
Got: 104
Got: 103
Got: 102
Got: 101
Got: 100
```

关于 多生产者、单消费者FIFO队列通信, 参考:

- [std::sync::mpsc - Rust (rust-lang.org)](https://doc.rust-lang.org/std/sync/mpsc/)

## UDP

[UdpSocket in std::net - Rust (rust-lang.org)](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)

下面的例子就不区分server和client了

```bash
$ cargo new play_udp

# play_udp/Cargo.toml 最下面添加
[[bin]]
name = "play_udp_server"
path = "src/play_udp_server/main.rs"

[[bin]]
name = "play_udp_client"
path = "src/play_udp_client/main.rs"

$ tree play_udp/
play_udp/
├── Cargo.toml
└── src
    ├── play_udp_client
    │   └── main.rs
    └── play_udp_server
        └── main.rs
```

7878端口

```rust
// 导入UdpSocket
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
```

8888端口

```rust
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
```

运行

```bash
$ cargo build

# 先运行server, 再运行client
$ ./target/debug/play_udp_server 
$ ./target/debug/play_udp_client
```

## 库引入

三种库:

- [crates.io: Rust Package Registry](https://crates.io/), RUST的官方仓库, 如 `rand = "0.3"`
- Git仓库, 如 `color = { git = "https://github.com/bjz/color-rs" }`
- 绝对/相对路径, 如 `geometry = { path = "crates/geometry" }`

如果一直卡顿或报错

```bash
# 引入库报
# Blocking waiting for file lock on package cache
rm -rf ~/.cargo/.package-cache 
```

## UART serial

[serial - Rust (docs.rs)](https://docs.rs/serial/0.4.0/serial/)

用python生成一对虚拟串口(尽量不要同时读写)

```python
#!/usr/bin/python3

import pty
import os
import select

def mkpty():
    #  Open the pseudo terminal
    master1, slave = pty.openpty()
    slaveName1 = os.ttyname(slave)
    master2, slave = pty.openpty()
    slaveName2 = os.ttyname(slave)
    print('slave device names: ', slaveName1, slaveName2)
    return master1, master2

if __name__ == "__main__":
    master1, master2 = mkpty()
    while True:
        rl, wl, el = select.select([master1,master2], [], [], 1)
        for master in rl:
            data = os.read(master, 128)
            print("read %d data." % len(data) )
            if master==master1:
                os.write(master2, data)
            else:
                os.write(master1, data)
```

运行, 打印出的  `/dev/pts/31`, `/dev/pts/32` 就是一对虚拟串口

```bash
$ python3 vcom.py 
slave device names:  /dev/pts/6 /dev/pts/7
```

rust工程引入serial, 在`Cargo.toml`

```bash
[dependencies]
serial = "0.4.0"
```

类似上面的udp

```bash
$ cargo new play_serial

# play_serial/Cargo.toml 引入serial, 添加bin
[dependencies]
serial = "0.4.0"

[[bin]]
name = "play_serial_server"
path = "src/play_serial_server/main.rs"

[[bin]]
name = "play_serial_client"
path = "src/play_serial_client/main.rs"

$ tree play_serial/
play_serial/
├── Cargo.toml
└── src
    ├── play_serial_client
    │   └── main.rs
    ├── play_serial_server
    │   └── main.rs
    └── vcom.py
```

server

```rust
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
```

client

```rust
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
```

运行

```bash
$ cargo build
# Blocking waiting for file lock on package cache
# 如果一直卡在上面这句, 就运行下面的命令
# rm -rf ~/.cargo/.package-cache 

# 串口操作可能需要加 sudo
$ ./target/debug/play_serial_server
$ ./target/debug/play_serial_client 
```

## UART serial2

上面的 [dcuddeback/serial-rs: Rust library for interacting with serial ports. (github.com)](https://github.com/dcuddeback/serial-rs) 可以看到已经停止维护5年了, 最近有个 [de-vri-es/serial2-rs: Cross platform serial ports for Rust (github.com)](https://github.com/de-vri-es/serial2-rs), 先不谈性能如何, 先用用看(当然, 如果对所有的都不满意, 可以自己写一个, 或者FFI套C)

[serial2 - Rust (docs.rs)](https://docs.rs/serial2/latest/serial2/), examples中可以列出可用串口, 多线程, 多串口操作等

```bash
$ cargo new play_serial2

# play_serial/Cargo.toml 引入serial, 添加bin
[dependencies]
serial2 = "0.1.6"

[[bin]]
name = "play_serial2_server"
path = "src/play_serial2_server/main.rs"

[[bin]]
name = "play_serial2_client"
path = "src/play_serial2_client/main.rs"

$ tree play_serial2/
play_serial2
├── Cargo.toml
└── src
    ├── play_serial2_client
    │   └── main.rs
    └── play_serial2_server
        └── main.rs
```

Server

```rust
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
```

Client

```rust
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
```

测试结果类似上小节, 过程略

## SocketCAN

在Ubuntu虚拟出一对CAN, `vxcan.sh`

```bash
#!/bin/sh
sudo modprobe can_raw
sudo modprobe vxcan

# 如果 ip link 中存在 vcan0, 就删除 vcan0
if ip link show can0 > /dev/null 2>&1; then
    sudo ip link set dev can0 down
    sudo ip link set dev vxcan0 down
    sudo ip link delete dev can0 type vxcan
fi

sudo ip link add dev can0 type vxcan
sudo ip link set up can0
sudo ip link set dev vxcan0  up
```

[Search Results for 'can' - crates.io: Rust Package Registry](https://crates.io/search?page=1&per_page=10&q=can) 可以找找更多的封装can的, 还有很多新特性支持, 如异步, 多线程等, 但似乎没有找到太好用的

Cargo.toml 中引入socketcan

```bash
[dependencies]
socketcan = "1.7.0"
```

测试代码

```rust
extern crate socketcan;

fn main() {
    println!("Hello, world!");
    // 创建一个socketcan的客户端
    let socket = socketcan::CANSocket::open("can0").unwrap();
    // 创建一个帧, id, data, rtr, err
    let mut frame = socketcan::CANFrame::new(0x123, &[1, 3, 5, 7, 9, 11, 13, 15], false, false).unwrap();
    // 发送帧
    socket.write_frame(&frame).unwrap();
    // 修改id
    frame = socketcan::CANFrame::new(0x1FFFFFFF, &[1, 3, 5, 7, 9, 11, 13, 15], false, false).unwrap();
    // 发送帧
    socket.write_frame(&frame).unwrap();
    // 接收帧
    frame = socket.read_frame().unwrap();
    // 打印帧
    println!("{:?}", frame);
}
```

这个毕竟年代久远了, frame中的id, data之类的不是pub, 性能一般, 也不支持CANFD...

其实不如直接完全照搬SocketCAN的C接口, 这样可以无缝理解, 下面用 libc 重写一下.  

## CAN

[libc - Rust (docs.rs)](https://docs.rs/libc/latest/libc/)

```bash
$ cargo new play_can

# play_can/Cargo.toml
[dependencies]
libc = "0.2.132"
ifstructs = "0.1.1"
iptool = "0.1.0"
```

下面就把C代码逐行翻译成rust, 参考 `main.rs`

```rust
extern crate ifstructs;
extern crate iptool;
extern crate libc;

use std::ffi::CString;

// can_id: 32 bit, CAN_ID + EFF/RTR/ERR flags
struct CanFrame {
    can_id: u32,
    can_dlc: u8,
    __pad: u8,
    __res0: u8,
    __res1: u8,
    data: [u8; 8],
}

fn canopen(ifname: &str) -> i32 {
    let ifname = CString::new(ifname).unwrap();
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_CAN, libc::SOCK_RAW, libc::CAN_RAW);
        if fd < 0 {
            println!("socket error");
            std::process::exit(1);
        }
        let mut ifr: ifstructs::ifreq = std::mem::zeroed();
        for i in 0..ifname.as_bytes().len() {
            ifr.ifr_name[i] = ifname.as_bytes()[i];
        }
        libc::ioctl(fd, iptool::SIOCGIFINDEX as libc::c_ulong, &mut ifr);
        let mut addr: libc::sockaddr_can = std::mem::zeroed();
        addr.can_family = libc::AF_CAN as libc::sa_family_t;
        addr.can_ifindex = ifr.ifr_ifru.ifr_ifindex;
        let ret = libc::bind(
            fd,
            &addr as *const libc::sockaddr_can as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_can>() as libc::socklen_t,
        );
        if ret < 0 {
            println!("bind error");
            std::process::exit(1);
        }
        return fd as i32;
    }
}

fn canclose(fd: i32) -> i32 {
    unsafe {
        return libc::close(fd as libc::c_int);
    }
}

fn canwrite(fd: i32, frame: &mut CanFrame) -> i32 {
    unsafe {
        let ret = libc::write(
            fd as libc::c_int,
            frame as *mut CanFrame as *const libc::c_void,
            std::mem::size_of::<CanFrame>() as libc::size_t,
        );
        return ret as i32;
    }
}

fn canread(fd: i32, frame: &mut CanFrame) -> i32 {
    unsafe {
        let ret = libc::read(
            fd as libc::c_int,
            frame as *mut CanFrame as *mut libc::c_void,
            std::mem::size_of::<CanFrame>() as libc::size_t,
        );
        return ret as i32;
    }
}

fn main() {
    let fd = canopen("can0");
    let mut frame = CanFrame {
        can_id: 0x123,
        can_dlc: 8,
        __pad: 0,
        __res0: 0,
        __res1: 0,
        data: [0; 8],
    };
    canwrite(fd, &mut frame);
    canread(fd, &mut frame);
    println!(
        "can_id: {:08x}, can_dlc: {}, __pad: {}, __res0: {}, __res1: {}, data: {:02x?}",
        frame.can_id, frame.can_dlc, frame.__pad, frame.__res0, frame.__res1, frame.data
    );
    canclose(fd);
}

// $ cansend vxcan0 12345678#11.22
// can_id: 92345678, can_dlc: 2, __pad: 0, __res0: 0, __res1: 0, data: [11, 22, 00, 00, 00, 00, 00, 00]

```

测试

```bash
$ candump -td -x any

$ cargo build
$ ./target/debug/play_can

$ cansend vxcan0 12345678#11.22
# 因为是扩展帧, can_id 最高位置1, 所以can_id是92345678
```

## CANFD

```bash
$ cargo new play_canfd

# play_canfd/Cargo.toml
[dependencies]
libc = "0.2.132"
ifstructs = "0.1.1"
iptool = "0.1.0"
```

把C代码逐行翻译成rust, 参考 `canfd.rs`

```rust
extern crate ifstructs;
extern crate iptool;
extern crate libc;

use std::ffi::CString;

pub struct CanfdFrame {
    can_id: u32,
    len: u8,
    flags: u8,
    __res0: u8,
    __res1: u8,
    data: [u8; 64],
}

pub fn open(ifname: &str) -> i32 {
    let ifname = CString::new(ifname).unwrap();
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_CAN, libc::SOCK_RAW, libc::CAN_RAW);
        if fd < 0 {
            println!("socket error");
            std::process::exit(1);
        }
        let mut ifr: ifstructs::ifreq = std::mem::zeroed();
        for i in 0..ifname.as_bytes().len() {
            ifr.ifr_name[i] = ifname.as_bytes()[i];
        }
        libc::ioctl(fd, iptool::SIOCGIFINDEX as libc::c_ulong, &mut ifr);
        let mut addr: libc::sockaddr_can = std::mem::zeroed();
        addr.can_family = libc::AF_CAN as libc::sa_family_t;
        addr.can_ifindex = ifr.ifr_ifru.ifr_ifindex;
        // canfd support
        let mut canfd_on: libc::c_int = 1;
        libc::setsockopt(
            fd,
            libc::SOL_CAN_RAW,
            libc::CAN_RAW_FD_FRAMES,
            &mut canfd_on as *mut libc::c_int as *mut libc::c_void,
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        );
        let ret = libc::bind(
            fd,
            &addr as *const libc::sockaddr_can as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_can>() as libc::socklen_t,
        );
        if ret < 0 {
            println!("bind error");
            std::process::exit(1);
        }
        return fd as i32;
    }
}

pub fn close(fd: i32) -> i32 {
    unsafe {
        return libc::close(fd as libc::c_int);
    }
}

pub fn write(fd: i32, frame: &mut CanfdFrame) -> i32 {
    unsafe {
        let ret = libc::write(
            fd as libc::c_int,
            frame as *mut CanfdFrame as *const libc::c_void,
            std::mem::size_of::<CanfdFrame>() as libc::size_t,
        );
        return ret as i32;
    }
}

pub fn read(fd: i32, frame: &mut CanfdFrame) -> i32 {
    unsafe {
        let ret = libc::read(
            fd as libc::c_int,
            frame as *mut CanfdFrame as *mut libc::c_void,
            std::mem::size_of::<CanfdFrame>() as libc::size_t,
        );
        return ret as i32;
    }
}

pub fn test() {
    let fd = open("can0");
    let mut frame = CanfdFrame {
        can_id: 0x123,
        len: 64,
        flags: 0,
        __res0: 0,
        __res1: 0,
        data: [0; 64],
    };
    write(fd, &mut frame);
    read(fd, &mut frame);
    println!(
        "can_id: {:08x}, can_dlc: {}, __pad: {}, __res0: {}, __res1: {}, data: {:02x?}",
        frame.can_id, frame.len, frame.flags, frame.__res0, frame.__res1, frame.data
    );
    close(fd);
}

```

然后是 `main.rs`

```rust
mod canfd;

fn main() {
    canfd::test();
}
```

测试

```bash
$ candump -td -x any

$ cargo build
$ ./target/debug/play_canfd

$ cansend vxcan0 12345678##3.11.22.33.44.55.66.77.88.99.AA.BB.CC.DD.EE.FF
# 因为是扩展帧, can_id 最高位置1, 所以can_id是92345678
# B, BRS
# E, ESI
# 15字节自动扩充到canfd的16字节
```

除了直接翻译C以外, 另一种流行的思路是既然C底层受众广, 很多也比较稳, 那就直接FFI的方式, 让rust用C的头文件和编译好的静态库好了, 如用 `bindgen`, 完成了很多自动化的工作, 简单粗暴又高效.  

## play_ffi_cantools

本篇给出一个 Rust 通过FFI(Foreign Function Interface)调用 DBC生成的C代码 解析传感器数据的例子

### 传感器 DBC

以一个简单的开放DBC的倾角传感器MTLT305D为例, DBC可以到官网下载

- [MTLT305D - 新纳传感: Aceinna: Leader in MEMS Sensor Technology](https://www.aceinna.cn/inertial-systems/MTLT305D), 这个是中文官网, 下载的版本是 `Aceinna MTLT305D_dbc_19.1.51_20190621.dbc`
- [MTLT305D - Aceinna: Leader in MEMS Sensor Technology](https://www.aceinna.com/inertial-systems/MTLT305D), 这个是英文官网, 下载的版本是 `Aceinna MTLT305D_dbc_19.1.51_20190620.dbc`

下载后重命名为 `MTLT305D_dbc_19.1.51_20190621.dbc`, 注意:

- 可以不需要CAN分析仪, PC上直接用vcan或vxcan模拟即可
- 可以不需要真的去买一个传感器, 下面会写一个传感器的模拟器
- 这可能是一个非标准的DBC或者里面存在一些错误, 用 rust 开源的 `dbcc` 或 `dbc-codegen` 生成报错, python cantools 可能有一定的纠错能力,  是可以顺利生成C代码的, 这也是本篇的由来.  

### VXCAN

首先是PC没有can口, 用VXCAN来模拟一对 `can0-vxcan0`, 解析程序用`can0`, 模拟器用`vxcan0`, 脚本`vxcan.sh`如下

```bash
#!/bin/sh
sudo modprobe can_raw
sudo modprobe vxcan

if ip link show can0 > /dev/null 2>&1; then
    sudo ip link set dev can0 down
    sudo ip link set dev vxcan0 down
    sudo ip link delete dev can0 type vxcan
fi

sudo ip link add dev can0 type vxcan
sudo ip link set up can0
sudo ip link set dev vxcan0 up
```

### Python Cantools 写传感器模拟器

用Python配合cantools来写传感器模拟器极为简单, 先安装必要的库

```bash
python3 -m pip install python-can
python3 -m pip install cantools
```

倾角传感器主要用到 `3轴acc, 3轴gyro, pitch, roll`, 主要涉及3帧报文, 所以模拟器里也只实现这3帧报文, 100Hz, 造假数据循环播发. 加载DBC, 填充消息即可, `fake_mtlt305d.py`如下

```python
#!/usr/bin/python3

import can
import cantools
import time
from datetime import datetime
from threading import Timer

can_bus = can.interface.Bus(bustype='socketcan', channel='vxcan0', bitrate=500000)
sensor_dbc = cantools.database.load_file('MTLT305D_dbc_19.1.51_20190621.dbc')

def x8F02D80_send(acc_x, acc_y, acc_z):
    msg = sensor_dbc.get_message_by_name("Aceinna_Accel")
    data = msg.encode({
        "Aceinna_AccX": acc_x,
        "Aceinna_AccY": acc_y,
        "Aceinna_AccZ": acc_z,
        "Aceinna_LateralAcc_FigureOfMerit": 0,
        "Aceinna_LongiAcc_FigureOfMerit": 0,
        "Aceinna_VerticAcc_FigureOfMerit": 0,
        "Aceinna_Support_Rate_Acc": 0
    })
    message = can.Message(
        arbitration_id = msg.frame_id,
        data = data,
        is_extended_id = True
    )
    can_bus.send(message)

def xCF02A80_send(gyro_x, gyro_y, gyro_z):
    msg = sensor_dbc.get_message_by_name("Aceinna_AngleRate")
    data = msg.encode({
        "Aceinna_GyroX": gyro_x,
        "Aceinna_GyroY": gyro_y,
        "Aceinna_GyroZ": gyro_z,
        "Aceinna_PitchRate_Figure_OfMerit": 0,
        "Aceinna_RollRate_Figure_OfMerit": 0,
        "Aceinna_YawRate_Figure_OfMerit": 0,
        "Aceinna_AngleRate_Latency": 0
    })
    message = can.Message(
        arbitration_id = msg.frame_id,
        data = data,
        is_extended_id = True
    )
    can_bus.send(message)

def xCF02980_send(pitch, roll):
    msg = sensor_dbc.get_message_by_name("Aceinna_Angles")
    data = msg.encode({
        "Aceinna_Pitch": pitch,
        "Aceinna_Roll": roll,
        "Aceinna_Pitch_Compensation": 0,
        "Aceinna_Pitch_Figure_OfMerit": 0,
        "Aceinna_Roll_Compensation": 0,
        "Aceinna_Roll_Figure_OfMerit": 0,
        "Aceinna_PitchRoll_Latency": 0
    })
    message = can.Message(
        arbitration_id = msg.frame_id,
        data = data,
        is_extended_id = True
    )
    can_bus.send(message)

if __name__ == '__main__':
    cnt = 0
    while True:
        t = time.time()
        x8F02D80_send(1+cnt,2+cnt,3+cnt)
        xCF02A80_send(4+cnt,5+cnt,6+cnt)
        xCF02980_send(7+cnt,8+cnt)
        cnt = (cnt+1)%10
        dt = time.time() - t
        if dt < 0.01:
            time.sleep(0.01 - dt)
```

运行程序, 检验

```bash
$ python3 fake_mtlt305d.py

$ candump -td -x can0
 ...
 (000.010047)  can0  TX - -  08F02D80   [8]  C8 7D 2C 7E 90 7E 00 00
 (000.000061)  can0  TX - -  0CF02A80   [8]  80 7F 00 80 80 80 00 00
 (000.000026)  can0  TX - -  0CF02980   [8]  00 00 81 00 80 81 00 00
 (000.010046)  can0  TX - -  08F02D80   [8]  2C 7E 90 7E F4 7E 00 00
 (000.000065)  can0  TX - -  0CF02A80   [8]  00 80 80 80 00 81 00 00
 (000.000025)  can0  TX - -  0CF02980   [8]  00 80 81 00 00 82 00 00
 (000.010045)  can0  TX - -  08F02D80   [8]  90 7E F4 7E 58 7F 00 00
 (000.000060)  can0  TX - -  0CF02A80   [8]  80 80 00 81 80 81 00 00
 (000.000026)  can0  TX - -  0CF02980   [8]  00 00 82 00 80 82 00 00
 (000.010048)  can0  TX - -  08F02D80   [8]  F4 7E 58 7F BC 7F 00 00
```

### DBC 生成C代码

```bash
python3 -m cantools generate_c_source --database-name mtlt305d -e UTF-8  MTLT305D_dbc_19.1.51_20190621.dbc
```

把生成的 `mtlt305d.c` 和 `mtlt305d.h` 移到src文件夹.  

### Rust bindgen

先来看下最终的工程目录

```bash
$ tree play_ffi_cantools/
play_ffi_cantools/
├── build.rs
├── Cargo.toml
├── fake_mtlt305d.py
├── MTLT305D_dbc_19.1.51_20190621.dbc
├── src
│   ├── can.rs
│   ├── main.rs
│   ├── mtlt305d.c
│   └── mtlt305d.h
├── vxcan.sh
└── wrapper.h
```

[Library Usage with build.rs - The `bindgen` User Guide (rust-lang.github.io)](https://rust-lang.github.io/rust-bindgen/library-usage.html)

把`bindgen`添加到`Cargo.toml`, 事实上还需要 [rust-lang/cc-rs: Rust library for build scripts to compile C/C++ code into a Rust library (github.com)](https://github.com/rust-lang/cc-rs) 把 C代码编译成库, `dependencies`里的三个库可参考上篇, 是给`CAN`用的:

```bash
$ cat Cargo.toml
[package]
name = "play_ffi_cantools"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2.132"
ifstructs = "0.1.1"
iptool = "0.1.0"

[build-dependencies]
bindgen = "0.60.1"
cc = "1.0.73"
```

写一个`wrapper.h`, (不一定非要叫这个名字, 和`build.rs`对应上就行?)

```c
#include "src/mtlt305d.h"
```

编写`build.rs`: 首先是用cc把dbc生成的c代码编译成库, 然后在编译时生成 Rust FFI 绑定

```rust
extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/mtlt305d.c")
        .compile("mtlt305d");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

给库写rust接口或者直接填充`src/main.rs`解析:

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod can;

#[derive(Debug)]
struct mtlt305d_t {
    acc_x: f64,
    acc_y: f64,
    acc_z: f64,
    gyro_x: f64,
    gyro_y: f64,
    gyro_z: f64,
    pitch: f64,
    roll: f64,
}

impl mtlt305d_t {
    fn new() -> mtlt305d_t {
        mtlt305d_t {
            acc_x: 0.0,
            acc_y: 0.0,
            acc_z: 0.0,
            gyro_x: 0.0,
            gyro_y: 0.0,
            gyro_z: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}

fn mtlt305d_parser(frame: &can::CanFrame, mtlt305d: &mut mtlt305d_t) -> i32 {
    let id = frame.can_id & 0x1FFFFFFF;
    let ret = match id {
        MTLT305D_ACEINNA_ANGLES_FRAME_ID => unsafe {
            let mut msg = std::mem::zeroed();
            mtlt305d_aceinna_angles_unpack(
                &mut msg,
                &frame.data[0],
                MTLT305D_ACEINNA_ANGLES_LENGTH.into(),
            );
            mtlt305d.pitch = mtlt305d_aceinna_angles_aceinna_pitch_decode(msg.aceinna_pitch);
            mtlt305d.roll = mtlt305d_aceinna_angles_aceinna_roll_decode(msg.aceinna_roll);
            1
        },
        MTLT305D_ACEINNA_ACCEL_FRAME_ID => unsafe {
            let mut msg = std::mem::zeroed();
            mtlt305d_aceinna_accel_unpack(
                &mut msg,
                &frame.data[0],
                MTLT305D_ACEINNA_ACCEL_LENGTH.into(),
            );
            mtlt305d.acc_x = mtlt305d_aceinna_accel_aceinna_acc_x_decode(msg.aceinna_acc_x);
            mtlt305d.acc_y = mtlt305d_aceinna_accel_aceinna_acc_y_decode(msg.aceinna_acc_y);
            mtlt305d.acc_z = mtlt305d_aceinna_accel_aceinna_acc_z_decode(msg.aceinna_acc_z);
            2
        },
        MTLT305D_ACEINNA_ANGLE_RATE_FRAME_ID => unsafe {
            let mut msg = std::mem::zeroed();
            mtlt305d_aceinna_angle_rate_unpack(
                &mut msg,
                &frame.data[0],
                MTLT305D_ACEINNA_ANGLE_RATE_LENGTH.into(),
            );
            mtlt305d.gyro_x = mtlt305d_aceinna_angle_rate_aceinna_gyro_x_decode(msg.aceinna_gyro_x);
            mtlt305d.gyro_y = mtlt305d_aceinna_angle_rate_aceinna_gyro_y_decode(msg.aceinna_gyro_y);
            mtlt305d.gyro_z = mtlt305d_aceinna_angle_rate_aceinna_gyro_z_decode(msg.aceinna_gyro_z);
            3
        },
        _ => {
            println!("{}", id);
            0
        },
    }
    .into();
    ret
}

fn main() {
    let fd = can::can_open("can0");
    if fd < 0 {
        println!("can_open failed");
        return;
    }
    let mut mtlt305d = mtlt305d_t::new();
    let mut frame = can::CanFrame::new();
    loop {
        can::can_read(fd, &mut frame);
        let ret = mtlt305d_parser(&frame, &mut mtlt305d);
        if ret == 3 {
            println!("{:?}", mtlt305d);
        }
    }
}
```

编译运行

```bash
$ cargo build

# cargo run -p play_ffi_cantools
# cargo run --bin play_ffi_cantools
# ./target/debug/play_ffi_cantools
$ cargo run -p play_ffi_cantools
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/play_ffi_cantools`
mtlt305d_t { acc_x: 1.0, acc_y: 2.0, acc_z: 3.0, gyro_x: 4.0, gyro_y: 5.0, gyro_z: 6.0, pitch: 0.0, roll: 0.0 }
mtlt305d_t { acc_x: 2.0, acc_y: 3.0, acc_z: 4.0, gyro_x: 5.0, gyro_y: 6.0, gyro_z: 7.0, pitch: 7.0, roll: 8.0 }
mtlt305d_t { acc_x: 3.0, acc_y: 4.0, acc_z: 5.0, gyro_x: 6.0, gyro_y: 7.0, gyro_z: 8.0, pitch: 8.0, roll: 9.0 }
mtlt305d_t { acc_x: 4.0, acc_y: 5.0, acc_z: 6.0, gyro_x: 7.0, gyro_y: 8.0, gyro_z: 9.0, pitch: 9.0, roll: 10.0 }
mtlt305d_t { acc_x: 5.0, acc_y: 6.0, acc_z: 7.0, gyro_x: 8.0, gyro_y: 9.0, gyro_z: 10.0, pitch: 10.0, roll: 11.0 }
mtlt305d_t { acc_x: 6.0, acc_y: 7.0, acc_z: 8.0, gyro_x: 9.0, gyro_y: 10.0, gyro_z: 11.0, pitch: 11.0, roll: 12.0 }
mtlt305d_t { acc_x: 7.0, acc_y: 8.0, acc_z: 9.0, gyro_x: 10.0, gyro_y: 11.0, gyro_z: 12.0, pitch: 12.0, roll: 13.0 }
mtlt305d_t { acc_x: 8.0, acc_y: 9.0, acc_z: 10.0, gyro_x: 11.0, gyro_y: 12.0, gyro_z: 13.0, pitch: 13.0, roll: 14.0 }
mtlt305d_t { acc_x: 9.0, acc_y: 10.0, acc_z: 11.0, gyro_x: 12.0, gyro_y: 13.0, gyro_z: 14.0, pitch: 14.0, roll: 15.0 }
mtlt305d_t { acc_x: 10.0, acc_y: 11.0, acc_z: 12.0, gyro_x: 13.0, gyro_y: 14.0, gyro_z: 15.0, pitch: 15.0, roll: 16.0 }
mtlt305d_t { acc_x: 1.0, acc_y: 2.0, acc_z: 3.0, gyro_x: 4.0, gyro_y: 5.0, gyro_z: 6.0, pitch: 16.0, roll: 17.0 }
mtlt305d_t { acc_x: 2.0, acc_y: 3.0, acc_z: 4.0, gyro_x: 5.0, gyro_y: 6.0, gyro_z: 7.0, pitch: 7.0, roll: 8.0 }
```

可以看到解析结果正如模拟器所发...

## Github CSDN

[rust_note/playground at main · weifengdq/rust_note (github.com)](https://github.com/weifengdq/rust_note/tree/main/playground)

[RUST 环境 UDP UART CANFD_weifengdq的博客-CSDN博客](https://blog.csdn.net/weifengdq/article/details/126393809?spm=1001.2014.3001.5501), 欢迎常来我的博客