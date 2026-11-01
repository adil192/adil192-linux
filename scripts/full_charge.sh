#!/bin/bash
# Disables the 80% charge limit until the next reboot.
echo 100 | sudo tee /sys/class/power_supply/BAT0/charge_control_end_threshold
echo  95 | sudo tee /sys/class/power_supply/BAT0/charge_control_start_threshold
