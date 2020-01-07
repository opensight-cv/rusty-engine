if [ "${CARGO}" == "cross" ]; then
    # set up docker
    cargo install cross
    sudo systemctl start docker
    echo Building docker image
    $(pwd)/cross-build.sh
fi
${CARGO} ${TASK} --target ${TARGET}
