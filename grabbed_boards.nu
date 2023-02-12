#!/usr/bin/env nu

let BOARDS = ($env.BOARDS | from nuon)

print $BOARDS
