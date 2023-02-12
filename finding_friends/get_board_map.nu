#!/usr/bin/env nu

# Get list of all boards.
let boards = (lsusb -d 1cbe:00fd | split row "\n" | where { |i| $i != '' } | each { |i| $"/dev/bus/usb/($i | str replace 'Bus (\d+) Device (\d+).+' '$1/$2')" } | wrap usb_path)

# Get the USB device buses and ports for use with OpenOCD.
let boards = ($boards | merge ($boards.usb_path | each { |i| udevadm info -q path $i | path basename | str trim } | wrap bus_port))

# Get tty device of each board.
let boards = ($boards | merge ($boards.usb_path | each { |i| udevadm info $i | find ID_PATH= | get 0 | ansi strip | str replace 'E: ID_PATH=' '' | ls $"/dev/serial/by-path/($in):1.0" | get 0.name} | wrap tty))

let boards = ($boards | sort-by tty)

print $boards