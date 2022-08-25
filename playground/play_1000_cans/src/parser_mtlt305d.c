#include <ctype.h>
#include <errno.h>
#include <libgen.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include <net/if.h>
#include <sys/epoll.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/uio.h>

#include <linux/can.h>
#include <linux/can/raw.h>

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

# define MAXIFNAMES 1000
static int dindex[MAXIFNAMES];
static int max_devname_len;
static char devname[MAXIFNAMES][IFNAMSIZ+1];
static int idx2dindex(int ifidx, int socket)
{

	int i;
	struct ifreq ifr;

	for (i = 0; i < MAXIFNAMES; i++) {
		if (dindex[i] == ifidx)
			return i;
	}

	/* create new interface index cache entry */

	/* remove index cache zombies first */
	for (i = 0; i < MAXIFNAMES; i++) {
		if (dindex[i]) {
			ifr.ifr_ifindex = dindex[i];
			if (ioctl(socket, SIOCGIFNAME, &ifr) < 0)
				dindex[i] = 0;
		}
	}

	for (i = 0; i < MAXIFNAMES; i++)
		if (!dindex[i]) /* free entry */
			break;

	if (i == MAXIFNAMES) {
		fprintf(stderr, "Interface index cache only supports %d interfaces.\n",
			MAXIFNAMES);
		exit(1);
	}

	dindex[i] = ifidx;

	ifr.ifr_ifindex = ifidx;
	if (ioctl(socket, SIOCGIFNAME, &ifr) < 0)
		perror("SIOCGIFNAME");

	if (max_devname_len < (int)strlen(ifr.ifr_name))
		max_devname_len = strlen(ifr.ifr_name);

	strcpy(devname[i], ifr.ifr_name);

#ifdef DEBUG
	printf("new index %d (%s)\n", i, devname[i]);
#endif

	return i;
}

int mtlt305d_parser(struct can_frame *frame, struct mtlt305d_t *mtlt305d)
{
    uint32_t id = frame->can_id & 0x1FFFFFFF;
    if (id == MTLT305D_ACEINNA_ANGLES_FRAME_ID)
    {
        //0xcf02980u
        struct mtlt305d_aceinna_angles_t msg1;
        mtlt305d_aceinna_angles_unpack(&msg1, frame->data, MTLT305D_ACEINNA_ANGLES_LENGTH);
        mtlt305d->pitch = mtlt305d_aceinna_angles_aceinna_pitch_decode(msg1.aceinna_pitch);
        mtlt305d->roll = mtlt305d_aceinna_angles_aceinna_roll_decode(msg1.aceinna_roll);
    }
    else if (id == MTLT305D_ACEINNA_ACCEL_FRAME_ID)
    {
        //0x8f02d80u
        struct mtlt305d_aceinna_accel_t  msg2;
        mtlt305d_aceinna_accel_unpack(&msg2, frame->data, MTLT305D_ACEINNA_ACCEL_LENGTH);
        mtlt305d->accx = mtlt305d_aceinna_accel_aceinna_acc_x_decode(msg2.aceinna_acc_x);
        mtlt305d->accy = mtlt305d_aceinna_accel_aceinna_acc_y_decode(msg2.aceinna_acc_y);
        mtlt305d->accz = mtlt305d_aceinna_accel_aceinna_acc_z_decode(msg2.aceinna_acc_z);
    }
    else if (id == MTLT305D_ACEINNA_ANGLE_RATE_FRAME_ID)
    {
        //0xcf02a80u
        struct mtlt305d_aceinna_angle_rate_t msg3;
        mtlt305d_aceinna_angle_rate_unpack(&msg3, frame->data, MTLT305D_ACEINNA_ANGLE_RATE_LENGTH);
        mtlt305d->gyrox = mtlt305d_aceinna_angle_rate_aceinna_gyro_x_decode(msg3.aceinna_gyro_x);
        mtlt305d->gyroy = mtlt305d_aceinna_angle_rate_aceinna_gyro_y_decode(msg3.aceinna_gyro_y);
        mtlt305d->gyroz = mtlt305d_aceinna_angle_rate_aceinna_gyro_z_decode(msg3.aceinna_gyro_z);
        // it's the last frame
    }
    else
    {
        // printf("0x%08X\n", id);
    }
    return id;
}

int main(int argc, char **argv)
{
    if (argc != 2) {
        printf("Usage: %s <numbers>\n", argv[0]);
        return -1;
    }
    int n = atoi(argv[1]);

    int s[65536];
    struct epoll_event events_pending[65536];
    struct epoll_event event_setup;
    event_setup.events = EPOLLIN;
    int fd_epoll = epoll_create(1);
    if (fd_epoll < 0) {
        perror("epoll_create");
        return -1;
    }
    for(int i = 0; i < n; i++) {
        char c[10];
        sprintf(c, "can%d", i);
        s[i] = socketcan_init(c);
        // epoll
        event_setup.data.ptr = &s[i];
        if(epoll_ctl(fd_epoll, EPOLL_CTL_ADD, s[i], &event_setup)) {
            perror("failed to add socketcan to epoll");
            return -1;
        }
    }

    struct iovec iov;
    struct can_frame frame;
    struct msghdr msg;
    struct sockaddr_can addr;
    char ctrlmsg[CMSG_SPACE(sizeof(struct timeval) + 3 * sizeof(struct timespec) + sizeof(__u32))];
    iov.iov_base = &frame;
	msg.msg_name = &addr;
	msg.msg_iov = &iov;
	msg.msg_iovlen = 1;
	msg.msg_control = &ctrlmsg;
    
    struct mtlt305d_t mtlt305d;
    int num_events = 0;
    volatile int running = 1;

    // struct timespec t;

    while(running) {
        num_events = epoll_wait(fd_epoll, events_pending, n, -1);
        if(num_events == -1) {
			if (errno != EINTR)
				running = 0;
			continue;
		}
        for (int i = 0; i < num_events; i++) {
			int *fs = (int *)events_pending[i].data.ptr;
			int idx;

			/* these settings may be modified by recvmsg() */
			iov.iov_len = sizeof(frame);
			msg.msg_namelen = sizeof(addr);
			msg.msg_controllen = sizeof(ctrlmsg);
			msg.msg_flags = 0;

            int nbytes = recvmsg(*fs, &msg, 0);
            // 第一次建立索引会比较慢??
            idx = idx2dindex(addr.can_ifindex, *fs);
            if (nbytes == -1) {
                if (errno != EINTR)
                running = 0;
                continue;
            }

            // print can999 parser results
            static char name[16] = "can999";
            if(strncmp(devname[idx], name, sizeof(name)) == 0) {
                int ret = mtlt305d_parser(&frame, &mtlt305d);
                if (ret == MTLT305D_ACEINNA_ANGLE_RATE_FRAME_ID) {
                    struct timespec t;
                    clock_gettime(CLOCK_REALTIME, &t);
                    printf("%ld.%09ld\t%f\t%f\t%f\t%f\t%f\t%f\t%f\t%f\n", \
                        t.tv_sec, t.tv_nsec, \
                        mtlt305d.accx,  mtlt305d.accy,  mtlt305d.accz, \
                        mtlt305d.gyrox, mtlt305d.gyroy, mtlt305d.gyroz, \
                        mtlt305d.pitch, mtlt305d.roll);
                }
            }
        }

    }

    for(int i = 0; i < n; i++) {
        close(s[i]);
    }

    return 0;
}

// gcc parser_mtlt305d.c mtlt305d.c -o b.out
// ./b.out 1000