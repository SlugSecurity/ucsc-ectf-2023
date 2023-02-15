#!/usr/bin/env nu

# Variables.
let PID = (bash -c 'echo -n $PPID')
let TARGET_DIR = (cargo metadata --format-version 1 | from json | get target_directory)
let TARGET_TRIPLE = "thumbv7em-none-eabihf"
let BIN_LOCATION = $"($TARGET_DIR)/($TARGET_TRIPLE)/debug"
let PROGRAM_CMD = { |usb bin| $"openocd -d0 -f board/ti_ek-tm4c123gxl.cfg -c 'tcl_port disabled' -c 'telnet_port disabled' -c 'gdb_port disabled' -c 'adapter usb location ($usb)' -c 'init' -c 'program ($BIN_LOCATION)/($bin) reset exit'" }

if "BOARDS" in $env {
    print "You already have a lock on a pair of boards! Try again in another terminal if you really need more boards."
    exit 1
}

# Remove friendless friend-locks.
try {
    if not (open /tmp/friendlock1 | into int) in (ps).pid {
        rm /tmp/friendlock1
    }
}

flock /tmp/friendlock0 nu -c $"while \('/tmp/friendlock1' | path exists\) {}; ($PID) out> /tmp/friendlock1; chmod 664 /tmp/friendlock1"

# Get list of all boards.
source ./finding_friends/get_board_map.nu

# Find free boards.
let free_boards = ($boards | where { |i| (flock -n $i.usb_path echo 1) == "1" })

# Take all but the last board.
let ping_boards = ($free_boards | first (($free_boards | length) - 1))

# Take the last board.
let pong_board = ($free_boards | last)

print "Ping Boards"
print $ping_boards

print "Pong Board"
print $pong_board

# Flash pong board.
cargo build --bin friendly_pong err> /dev/null
do -i { sh -c (do $PROGRAM_CMD $pong_board.bus_port friendly_pong) err> /dev/null } 

# Flash ping boards, loading each with a different BUS_PORT value.
$ping_boards | each { |i|
    let-env BUS_PORT = ($i.bus_port);
    cargo build --bin friendly_ping err> /dev/null;
    do -i { sh -c (do $PROGRAM_CMD $i.bus_port friendly_ping) err> /dev/null };
    sleep 50ms
}

# Finding friends!
let bp = (head -n 5 $pong_board.tty | str trim | split row "\n" | get 2)
let friend = try {
    $boards | where { |i| $bp == $i.bus_port } | first
} catch {
    print "No free board pairs could be found! Try again!"
    exit 1
}

# Set environment variables to pass to child shell.
let-env BOARDS = ([$pong_board, $friend] | to nuon)

print $"Your boards are ($pong_board.bus_port) and ($friend.bus_port)."

# Lock both USB devices and spawn child shell.
flock $pong_board.usb_path flock $friend.usb_path nu -c "rm /tmp/friendlock1; sh -c $env.SHELL"

print $"Releasing ($pong_board.bus_port) and ($friend.bus_port)."
