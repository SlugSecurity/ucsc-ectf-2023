#!/usr/bin/env sh
mkdir -p cache/cargo
docker build --tag rust-template .
docker run --rm --privileged -v $(pwd)/:/mnt -it rust-template