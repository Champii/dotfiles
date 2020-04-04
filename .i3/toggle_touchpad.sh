#!/bin/bash

device=13
state=$(xinput list-props $device | grep "Device Enabled" | cut -f3)
echo $state

if [ $state == 1 ]; then
xinput --disable "$device"
else
xinput --enable "$device"
fi
