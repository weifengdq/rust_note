#!/bin/sh
sudo modprobe can_raw
sudo modprobe vxcan

i=0
while [ $i -le 999 ]
do
    echo can$i
    if ip link show can$i > /dev/null 2>&1; then
        # i=$(($i+1))
        # continue
        sudo ip link set dev can$i down
        sudo ip link set dev vxcan$i down
        sudo ip link delete dev can$i type vxcan
    fi
    sudo ip link add dev can$i type vxcan
    sudo ip link set up can$i
    sudo ip link set dev vxcan$i up

    i=$(($i+1))
done