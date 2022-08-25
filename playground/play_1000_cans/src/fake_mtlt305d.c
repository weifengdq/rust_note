#include <linux/can.h>
#include <linux/can/raw.h>
#include <net/if.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>
#include "mtlt305d.h"

struct mtlt305d_t {
    double pitch;
    double roll;
    double accx;
    double accy;
    double accz;
    double gyrox;
    double gyroy;
    double gyroz;
};

int socketcan_init(char *path)
{
  int s;
  struct ifreq ifr;
  struct sockaddr_can addr;

  /* open socket */
  if ((s = socket(PF_CAN, SOCK_RAW, CAN_RAW)) < 0) {
    perror("socket");
    exit(1);
  }

  strcpy(ifr.ifr_name, path);
  ioctl(s, SIOCGIFINDEX, &ifr);

  memset(&addr, 0, sizeof(addr));
  addr.can_family = AF_CAN;
  addr.can_ifindex = ifr.ifr_ifindex;

  if (bind(s, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
    perror("bind");
    exit(2);
  }

  return s;
}

int mtlt305d_send(int s, struct mtlt305d_t *mtlt305d)
{
    int ret = 0;
    struct can_frame frame;
    frame.__pad = 0;
    frame.__res0 = 0;
    // frame.__res1 = 0;

    struct mtlt305d_aceinna_angles_t x0cf02980;
    x0cf02980.aceinna_pitch = mtlt305d_aceinna_angles_aceinna_pitch_encode(mtlt305d->pitch);
    x0cf02980.aceinna_roll = mtlt305d_aceinna_angles_aceinna_roll_encode(mtlt305d->roll);
    mtlt305d_aceinna_angles_pack(frame.data, &x0cf02980, MTLT305D_ACEINNA_ANGLES_LENGTH);
    frame.can_id = 0x0cf02980 | (1u << 31);
    frame.can_dlc = MTLT305D_ACEINNA_ANGLES_LENGTH;
    if(write(s, &frame, sizeof(frame)) != sizeof(frame)) {
        perror("write");
        ret = -1;
    }

    struct mtlt305d_aceinna_accel_t x08f02d80;
    x08f02d80.aceinna_acc_x = mtlt305d_aceinna_accel_aceinna_acc_x_encode(mtlt305d->accx);
    x08f02d80.aceinna_acc_y = mtlt305d_aceinna_accel_aceinna_acc_y_encode(mtlt305d->accy);
    x08f02d80.aceinna_acc_z = mtlt305d_aceinna_accel_aceinna_acc_z_encode(mtlt305d->accz);
    mtlt305d_aceinna_accel_pack(frame.data, &x08f02d80, MTLT305D_ACEINNA_ACCEL_LENGTH);
    frame.can_id = 0x08f02d80 | (1u << 31);
    frame.can_dlc = MTLT305D_ACEINNA_ACCEL_LENGTH;
    if(write(s, &frame, sizeof(frame)) != sizeof(frame)) {
        perror("write");
        ret = -1;
    }

    struct mtlt305d_aceinna_angle_rate_t x0cf02a80;
    x0cf02a80.aceinna_gyro_x = mtlt305d_aceinna_angle_rate_aceinna_gyro_x_encode(mtlt305d->gyrox);
    x0cf02a80.aceinna_gyro_y = mtlt305d_aceinna_angle_rate_aceinna_gyro_y_encode(mtlt305d->gyroy);
    x0cf02a80.aceinna_gyro_z = mtlt305d_aceinna_angle_rate_aceinna_gyro_z_encode(mtlt305d->gyroz);
    mtlt305d_aceinna_angle_rate_pack(frame.data, &x0cf02a80, MTLT305D_ACEINNA_ANGLE_RATE_LENGTH);
    frame.can_id = 0x0cf02a80 | (1u << 31);
    frame.can_dlc = MTLT305D_ACEINNA_ANGLE_RATE_LENGTH;
    if(write(s, &frame, sizeof(frame)) != sizeof(frame)) {
        perror("write");
        ret = -1;
    }
    return ret;
}

int main(int argc, char **argv)
{
    if (argc != 2) {
        printf("Usage: %s <numbers>\n", argv[0]);
        return -1;
    }
    int n = atoi(argv[1]);

    int s[65536];
    for(int i = 0; i < n; i++) {
        char c[10];
        sprintf(c, "vxcan%d", i);
        s[i] = socketcan_init(c);
        // printf("socket created successfully %d\n", s[i]);
    }

    struct mtlt305d_t mtlt305d;
    struct timespec t0, t1;
    int cnt = 0;
    while(1) {
        clock_gettime(CLOCK_MONOTONIC, &t0);
        for(int i = 0; i < n; i++) {
            mtlt305d.accx  = 1 + cnt % 10;
            mtlt305d.accy  = 2 + cnt % 10;
            mtlt305d.accz  = 3 + cnt % 10;
            mtlt305d.gyrox = 4 + cnt % 10;
            mtlt305d.gyroy = 5 + cnt % 10;
            mtlt305d.gyroz = 6 + cnt % 10;
            mtlt305d.pitch = 7 + cnt % 10;
            mtlt305d.roll  = 8 + cnt % 10;
            mtlt305d_send(s[i], &mtlt305d);
        }
        cnt++;
        clock_gettime(CLOCK_MONOTONIC, &t1);
        double dt = t1.tv_sec - t0.tv_sec  + (t1.tv_nsec - t0.tv_nsec) / 1e9;
        if(dt < 0.01) {
            usleep(10000 - dt*1e6);
        }
    }

    for(int i = 0; i < n; i++) {
        close(s[i]);
    }

    return 0;
}

// gcc fake_mtlt305d.c mtlt305d.c -o a.out
// ./a.out 1000