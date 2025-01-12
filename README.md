# datemon

Small tool to monitor the system date and initiate countermeasures, including a reboot, if it jumps too far ahead. This is a workaround for problems on some ARM SOC, in particular for Pine64 boards which apparently have problems properly counting seconds.

See, e.g. <https://forum.armbian.com/topic/7423-pine64-massive-datetime-clock-problem/>

A small tool to monitor the system date and initiate a reboot if it jumps too far ahead. This is a workaround
for problems on some ARM SOC, in particular for Pine64 borads which apperantly have problems properly 
counting seconds
## Features

- Monitors system date/time for unexpected jumps
- Configurable threshold for time jumps
- Can trigger system reboot when threshold exceeded
- Lightweight service that runs in background
