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

BUS_PORT=""
# Find which board is open.
for b in $BOARDS; do
    openocd -f $BOARD_CFG -c "tcl_port disabled" -c "telnet_port disabled" -c "gdb_port 3333" -c "adapter usb location $b" -c "init" -c "exit" &&
    BUS_PORT=$b &&
    break
done

if [ $BUS_PORT = "" ]; then
    echo "All of your boards are in use!"
    exit 1
fi

# Start OpenOCD.
openocd -f $BOARD_CFG -c "tcl_port disabled" -c "telnet_port disabled" -c "gdb_port 3333" -c "adapter usb location $b" &

# Build and run the project in GDB with the temporary .gdbinit file.
cargo run --bin $1

exit 0
