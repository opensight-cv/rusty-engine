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
# bind last argument in portable manner
for last in "$@"; do :; done

# build image
${DOCKER} build -t rusty-engine:latest docker/

if [ "${last}" == "run" ]; then
    ${DOCKER} run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rusty-engine:latest cargo build --release
    ${DOCKER} run --rm --privileged \
        --volume "$(pwd)":/docking-bay \
        rusty-engine:latest \
        bash -e -o pipefail -c \
        "cd /docking-bay; cargo deb"
fi
