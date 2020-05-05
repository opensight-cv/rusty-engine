FROM arm32v7/rust:1.43

WORKDIR /usr/src/rusty-engine
COPY . .

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get -y update
# gstreamer
RUN apt-get install --no-install-recommends -yq libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav:armhf \
    libgstrtspserver-1.0-dev
