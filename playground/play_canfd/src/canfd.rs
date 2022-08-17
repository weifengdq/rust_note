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
        flags: 3,
        __res0: 0,
        __res1: 0,
        data: [0; 64],
    };
    write(fd, &mut frame);
    read(fd, &mut frame);
    println!(
        "can_id: {:08x}, len: {}, flags: {}, __res0: {}, __res1: {}, data: {:02x?}",
        frame.can_id, frame.len, frame.flags, frame.__res0, frame.__res1, frame.data
    );
    close(fd);
}

// $ cansend vxcan0 12345678##3.11.22.33.44.55.66.77.88.99.AA.BB.CC.DD.EE.FF
// can_id: 92345678, len: 16, flags: 3, __res0: 0, __res1: 0, data: [11, 22, 33, 44, 55,
// 66, 77, 88, 99, aa, bb, cc, dd, ee, ff, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
// 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
// 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
