#!/usr/bin/env bash

# Check for project argument.
if [ -z "$1" ]
then
    echo "Error: missing project argument."
    echo "Usage: $0 <project>"
    exit 1
fi

# Setup the container.
mkdir -p cache/cargo
podman build --tag ectf .
podman run --rm --privileged -it ectf ./runner.sh $1
