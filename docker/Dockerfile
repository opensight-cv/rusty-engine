FROM arm32v7/debian:buster

ARG DEBIAN_FRONTEND=noninteractive

COPY qemu-arm-static /usr/bin/qemu-arm-static

RUN apt-get -y update
# basics
RUN apt-get install -y build-essential curl debhelper g++-arm-linux-gnueabihf libc6-dev-armhf-cross
# rust
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
# TODO: somewhere around here SSL apparently goes nuts?
RUN curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal --default-toolchain stable -y
ENV PATH="${PATH}:/opt/cargo/bin"
ENV RUST_TOOLCHAIN="stable"
RUN rustup toolchain install "${RUST_TOOLCHAIN}"
RUN rustup default "${RUST_TOOLCHAIN}"
# deb generator
RUN cargo install cargo-deb || true
# gstreamer
RUN apt-get install --no-install-recommends -y libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav:armhf \
    libgstrtspserver-1.0-dev
