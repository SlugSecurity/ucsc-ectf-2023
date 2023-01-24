#!/usr/bin/env bash

# Check for project argument.
if [ -z "$1" ]
then
    echo "Error: missing project argument."
    echo "Usage: $0 <project>"
    exit 1
fi

# Set variables.
VENDOR_PRODUCT="1cbe:00fd"
BOARD_CFG="board/ti_ek-tm4c123gxl.cfg"

# Kill background jobs on exit.
trap 'kill $(jobs -p) &> /dev/null' EXIT

# Critical section begin.
{
flock 99

# Find ports currently in use.
ports_in_use=`netstat -antu | tail -n +3 | awk '{split($4, parts,":"); print parts[length(parts)]}' | uniq`

declare -A ports_in_use_array

for port in $ports_in_use
do
    ports_in_use_array[$port]=1
done

# Find the first open port between 3333 and 5000.
port=3333

while [[ $port -le 5000 ]]
do
    if [[ ! -v ports_in_use_array[$port] ]]
    then
        echo $port > /tmp/openocd_gdb_port_$$.txt
        break
    fi

    ((port+=1))
done

# Check if a port was found.
if [ ! -f /tmp/openocd_gdb_port_$$.txt ]
then
    echo "Error: no available port was found."
    exit 1
fi

# Find all USB bus/port numbers with the given vendor and product ID, and loop through options.
while read usb
do
    # Run OpenOCD, wait for it to start, and make sure it's still running.
    openocd -f $BOARD_CFG -c "tcl_port disabled" -c "telnet_port disabled" -c "gdb_port $port" -c "adapter usb location $usb" &
    sleep 2
    ps -p $! > /dev/null

    if [ $? -eq 0 ]
    then
        found_device=1
        break
    fi
done < <(lsusb -d $VENDOR_PRODUCT | awk '{gsub(/:/,""); print "/dev/bus/usb/"$2"/"$4}' | xargs -I % sh -c 'udevadm info % | head -n 1 | sed "s:.*/::"')

# No unused device was found.
if [ -z $found_device ]
then
    echo "Error: no unused device was found."
    exit 1
fi

# Critical section end.
} 99< $0

# Create new .gdbinit file with the correct port.
sed "s/3333/$port/" .gdbinit > /tmp/.gdbinit_$$

# Build and run the project in GDB with the temporary .gdbinit file.
cargo run --bin $1 -- -n -x /tmp/.gdbinit_$$

# Clean up.
rm -f /tmp/openocd_gdb_port_$$.txt
rm -f /tmp/.gdbinit_$$

exit 0
