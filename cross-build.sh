#!/usr/bin/env bash
set -e

# detect if sudo is required on docker
DOCKER="docker"
if ! ${DOCKER} ps >/dev/null 2>&1; then
    DOCKER="sudo docker"
fi
if ! ${DOCKER} ps >/dev/null; then
    echo "error connecting to docker:"
    ${DOCKER} ps
    exit 1
fi

# build image
${DOCKER} build -t rusty-engine:latest .

cross build --release --target armv7-unknown-linux-gnueabihf
