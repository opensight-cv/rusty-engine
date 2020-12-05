FROM ubuntu:18.04

ENV DEBIAN_FRONTEND noninteractive

# use ports.ubuntu.com
RUN sed 's/http:\/\/\(.*\).ubuntu.com\/ubuntu\//[arch-=amd64,i386] http:\/\/ports.ubuntu.com\/ubuntu-ports\//g' /etc/apt/sources.list > /etc/apt/sources.list.d/ports.list
RUN sed -i 's/http:\/\/\(.*\).ubuntu.com\/ubuntu\//[arch=amd64,i386] http:\/\/\1.archive.ubuntu.com\/ubuntu\//g' /etc/apt/sources.list
RUN dpkg --add-architecture armhf
RUN apt-get -y update
# basic build tools
RUN apt-get install --assume-yes --no-install-recommends \
    autoconf \
    automake \
    binutils \
    ca-certificates \
    curl \
    file \
    gcc \
    g++ \
    git \
    libc6-dev \
    libtool \
    m4 \
    make \
    pkg-config

# cross-compiler chain
RUN apt-get install --assume-yes --no-install-recommends \
    g++-arm-linux-gnueabihf \
    libc6-dev-armhf-cross
ENV PKG_CONFIG_ALLOW_CROSS 1
ENV PKG_CONFIG_PATH "${PKG_CONFIG_PATH}:/usr/lib/arm-linux-gnueabihf/pkgconfig"
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc

# gstreamer
RUN apt-get install --no-install-recommends -yq libgstreamer1.0-dev:armhf \
    libgstreamer-plugins-base1.0-dev:armhf \
    gstreamer1.0-plugins-base:armhf \
    gstreamer1.0-plugins-good:armhf \
    gstreamer1.0-plugins-bad:armhf \
    gstreamer1.0-plugins-ugly:armhf \
    gstreamer1.0-libav:armhf \
    libgstrtspserver-1.0-dev:armhf
