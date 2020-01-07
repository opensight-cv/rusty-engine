#!/usr/bin/env bash
set -e

if [ "${CARGO}" == "cross" ]; then
    sudo apt-get install -y qemu-user-static
    # set up docker
    echo Installing cross
    cargo install cross || true
    sudo systemctl start docker
    echo Building docker image
    $(pwd)/cross-build.sh
fi
eval "$CARGO $TASK --target $TARGET"
