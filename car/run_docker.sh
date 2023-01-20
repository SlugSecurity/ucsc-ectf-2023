#!/usr/bin/env sh
mkdir -p cache/cargo
docker build --tag ectf .
docker run --rm --privileged -v $(pwd)/:/mnt -it ectf
