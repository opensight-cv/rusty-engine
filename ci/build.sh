#!/usr/bin/env bash

if [ "${CARGO}" == "cross" ]; then
    # set up docker
    cargo install cross
    sudo systemctl start docker
    echo Building docker image
    $(pwd)/cross-build.sh
fi
echo "$CARGO $TASK --target $TARGET"
eval "$CARGO $TASK --target $TARGET"
