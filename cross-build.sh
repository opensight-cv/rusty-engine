# bind last argument in portable manner
for last in "$@"; do :; done

docker_qemu="docker/qemu-arm-static"
rm -f ${docker_qemu}
cp $(which qemu-arm-static) ${docker_qemu}
${DOCKER} build -t rusty-engine:latest docker/
rm ${docker_qemu}

if [ "${last}" == "run" ]; then
    ${DOCKER} run --rm --privileged \
        --volume "$(pwd)":/docking-bay \
        --name "opsi-rusty-engine-arm" \
        rusty-engine:latest \
        bash -e -o pipefail -c \
        "uname -a"
fi
