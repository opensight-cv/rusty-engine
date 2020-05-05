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
${DOCKER} build -t rusty-engine:latest .

if [ "${last}" == "run" ]; then
    ${DOCKER} run --rm --user "$(id -u)":"$(id -g)" -v "$(pwd)":/usr/src/rusty-engine -w /usr/src/rusty-engine rusty-engine:latest cargo build --release
    # skip this for now
    #${DOCKER} run --rm --privileged \
    #    --volume "$(pwd)":/usr/src/rusty-engine \
    #    -w /usr/src/rusty-engine
    #    rusty-engine:latest \
    #    cargo deb
fi
