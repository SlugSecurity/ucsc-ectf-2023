#!/usr/bin/env sh
mkdir -p cache/cargo
podman build --tag ectf .
podman run --rm --privileged -v $(pwd)/:/mnt -it ectf
