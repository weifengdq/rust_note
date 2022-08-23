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
