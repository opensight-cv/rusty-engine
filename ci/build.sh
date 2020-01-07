#!/usr/bin/env bash

if [ "${CARGO}" == "cross" ]; then
    sudo apt-get install -y qemu-user-static
    # set up docker
    echo Installing cross
    cargo install --force cross
    sudo systemctl start docker
    echo Building docker image
    $(pwd)/cross-build.sh
fi
eval "$CARGO $TASK --target $TARGET"
