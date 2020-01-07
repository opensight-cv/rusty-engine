if [ $1 == "cross" ]; then
    # set up docker
    echo Building docker image
    $(pwd)/cross-build.sh
fi
${CARGO} ${TASK} --target ${TARGET}
