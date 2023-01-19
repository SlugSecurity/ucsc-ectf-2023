#!/usr/bin/env sh
mkdir -p cache/cargo
podman build --tag rust-template .
podman run --rm --privileged -v $(pwd)/:/mnt -it rust-template