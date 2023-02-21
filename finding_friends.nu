#!/usr/bin/env nu

# Variables.
let PID = (bash -c 'echo -n $PPID')
let TARGET_DIR = (cargo metadata --format-version 1 | from json | get target_directory)
let TARGET_TRIPLE = "thumbv7em-none-eabihf"
let BIN_LOCATION = $"($TARGET_DIR)/($TARGET_TRIPLE)/release"
let PROGRAM_CMD = { |usb bin| $"openocd -d0 -f board/ti_ek-tm4c123gxl.cfg -c 'tcl_port disabled' -c 'telnet_port disabled' -c 'gdb_port disabled' -c 'adapter usb location ($usb)' -c 'init' -c 'reset halt' -c 'load_image ($BIN_LOCATION)/($bin)' -c 'resume 0x2000026c' -c 'exit'" }

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

$free_boards | each { |pong_board|
    let ping_boards = ($free_boards | where { |i| $i != $pong_board })

    print "Ping Boards"
    print $ping_boards

    print "Pong Board"
    print $pong_board

    # Flash pong board.
    print "Flashing pong board."
    cargo build -r --bin friendly_pong err> /dev/null
    do -i { sh -c (do $PROGRAM_CMD $pong_board.bus_port friendly_pong) err> /dev/null } 

    # Flash ping boards, loading each with a different BUS_PORT value.
    print "Flashing ping boards."
    $ping_boards | each { |i|
        let-env BUS_PORT = ($i.bus_port);
        cargo build -r --bin friendly_ping err> /dev/null;
        do -i { sh -c (do $PROGRAM_CMD $i.bus_port friendly_ping) err> /dev/null };
        sleep 50ms
    }

    # Finding friends!
    print "Finding friends!"

    # Fix the TTY because you slobs can't clean up after yourselves.
    stty -F $pong_board.tty 115200
    try {
        let bp = (timeout 1 head -n 5 $pong_board.tty | str trim | split row "\n" | get 2)
        let friend = ($boards | where { |i| $bp == $i.bus_port } | first)

        # Set environment variables to pass to child shell.
        let-env BOARDS = ([$pong_board, $friend] | to nuon)

        print $"Your boards are ($pong_board.bus_port) and ($friend.bus_port)."

        # Lock both USB devices and spawn child shell.
        flock $pong_board.usb_path flock $friend.usb_path nu -c "rm /tmp/friendlock1; sh -c $env.SHELL"

        # We are satisified and need our friends no longer, also we stop looking for more friends.
        print $"Releasing ($pong_board.bus_port) and ($friend.bus_port)."
        exit 0
    } catch {
        print "Trying again!"
    }
}
