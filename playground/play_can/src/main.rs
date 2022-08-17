extern crate ifstructs;
extern crate iptool;
extern crate libc;

use std::ffi::CString;

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
