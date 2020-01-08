#!/usr/bin/env bash
set -e

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

docker_qemu="docker/qemu-arm-static"
rm -f ${docker_qemu}
cp "$(which qemu-arm-static)" ${docker_qemu}
${DOCKER} build -t rusty-engine:latest docker/
rm ${docker_qemu}

if [ "${last}" == "run" ]; then
    cross build --target armv7-unknown-linux-gnueabihf --release
    ${DOCKER} run --rm --privileged \
        --volume "$(pwd)":/docking-bay \
        rusty-engine:latest \
        bash -e -o pipefail -c \
        "cd /docking-bay; cargo deb"
fi
