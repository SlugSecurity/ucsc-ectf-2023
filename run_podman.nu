#!/usr/bin/env nu

def main [project: string] {
    if $project == "" {
        print "Error: missing project argument."
          print "Usage: ./run_podman.nu <project>"
        exit 1
    }

    mkdir cache/cargo

    # Collect locked boards from environment.
    source ./grabbed_boards.nu
    
    podman build --tag ectf .
    podman run -v ./:/mnt --rm --privileged -e $"BOARDS=($BOARDS.bus_port | to text)" -it ectf ./runner.sh $project
}
