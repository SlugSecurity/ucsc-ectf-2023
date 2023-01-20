#!/usr/bin/env sh

if [ -z "$1" ]
then
    echo "Missing project argument."
    echo "Usage: $0 <project>"
    exit 1
fi

mkdir -p cache/cargo
podman build --tag ectf .
podman run --rm --privileged -v $(pwd)/:/mnt -it ectf /bin/sh -c "trap 'kill $(jobs -p)' EXIT && openocd -f board/ti_ek-tm4c123gxl.cfg & cargo run --bin $1"
