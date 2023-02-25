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
    openocd -f $BOARD_CFG -c "tcl_port disabled" -c "telnet_port disabled" -c "gdb_port 3333" -c "adapter usb location $b" -c "init" -c "exit" 2> /dev/null &&
    BUS_PORT=$b &&
    break
done

if [ $BUS_PORT = "" ]; then
    echo "All of your boards are in use!"
    exit 1
fi

# Build the project.
cargo build --bin $1

# Extract the vector table.
arm-none-eabi-objcopy -O binary --only-section=.vector_table /mnt/target/thumbv7em-none-eabihf/debug/${1} /tmp/vt.bin

# Start OpenOCD.
openocd -f $BOARD_CFG -c "tcl_port disabled" -c "telnet_port disabled" -c "gdb_port 3333" -c "adapter usb location $BUS_PORT" -c 'init' -c 'program /tmp/vt.bin 0x0' 2> /dev/null &

# Run the project in GDB.
echo -e "\033[0;31mYou are currently debugging $BUS_PORT.\033[0m"
arm-none-eabi-gdb -q /mnt/target/thumbv7em-none-eabihf/debug/${1}

exit 0
