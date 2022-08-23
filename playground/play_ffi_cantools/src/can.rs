// 允许未使用的代码
#![allow(dead_code)]

extern crate ifstructs;
extern crate iptool;
extern crate libc;

use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub struct CanFrame {
    pub can_id: u32,
    pub can_dlc: u8,
    pub __pad: u8,
    pub __res0: u8,
    pub __res1: u8,
    pub data: [u8; 8],
}

impl CanFrame {
    pub fn new() -> CanFrame {
        CanFrame {
            can_id: 0,
            can_dlc: 0,
            __pad: 0,
            __res0: 0,
            __res1: 0,
            data: [0; 8],
        }
    }
}

pub fn can_open(ifname: &str) -> i32 {
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

pub fn can_set_timeout(fd: i32, timeout_ms: u32) -> i32 {
    unsafe {
        let mut tv: libc::timeval = std::mem::zeroed();
        tv.tv_sec = (timeout_ms / 1000) as i64;
        tv.tv_usec = ((timeout_ms % 1000) * 1000) as i64;
        let ret = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            &tv as *const libc::timeval as *const libc::c_void,
            std::mem::size_of::<libc::timeval>() as libc::socklen_t,
        );
        if ret < 0 {
            println!("setsockopt error");
            std::process::exit(1);
        }
        return ret as i32;
    }
}

pub fn can_close(fd: i32) -> i32 {
    unsafe {
        return libc::close(fd as libc::c_int);
    }
}

pub fn can_write(fd: i32, frame: &mut CanFrame) -> i32 {
    unsafe {
        let ret = libc::write(
            fd as libc::c_int,
            frame as *mut CanFrame as *const libc::c_void,
            std::mem::size_of::<CanFrame>() as libc::size_t,
        );
        return ret as i32;
    }
}

pub fn can_read(fd: i32, frame: &mut CanFrame) -> i32 {
    unsafe {
        let ret = libc::read(
            fd as libc::c_int,
            frame as *mut CanFrame as *mut libc::c_void,
            std::mem::size_of::<CanFrame>() as libc::size_t,
        );
        return ret as i32;
    }
}

pub fn can_test() {
    let fd = can_open("can0");
    can_set_timeout(fd, 1000);
    let mut frame = CanFrame {
        can_id: 0x123,
        can_dlc: 8,
        __pad: 0,
        __res0: 0,
        __res1: 0,
        data: [0, 1, 2, 3, 4, 5, 6, 7],
    };
    can_write(fd, &mut frame);
    can_read(fd, &mut frame);
    println!(
        "can_id: {:08x}, can_dlc: {}, __pad: {}, __res0: {}, __res1: {}, data: {:02x?}",
        frame.can_id, frame.can_dlc, frame.__pad, frame.__res0, frame.__res1, frame.data
    );
    can_close(fd);
}
