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
        }
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
