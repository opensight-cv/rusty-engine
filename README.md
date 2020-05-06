`rusty-engine`
===
:gear::steam_locomotive:

`rusty-engine` is a Rust rewrite/redesign/reimagining of The Quadrangles' [`potential-engine` RTSP server. ](https://github.com/BHSSFRC/potential-engine) `rusty-engine` aims to be more easily extensible and added to from a developer standpoint, while Rust helps to cover most of the corner cases and ensure everything fits together. (It's also just plainly more accessible to a novice than C/C++.)

## Dependencies
Aside from what `cargo` grabs automatically, `rusty-engine` requires some C packages to link to GStreamer. On Debian and Debian derivatives, these can be fetched with the following `apt` command:
```bash
sudo apt update && sudo apt install libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    libgstrtspserver-1.0-dev
```
## Cross-Compiling for Raspberry Pi
(because Travis won't give ARM 32 VMs...)

Running `cross-build.sh` with [`cross`](https://github.com/rust-embedded/cross) and Docker installed should (eventually) produce an ARM 32 executable file suitable for use with Raspbian in `target/arm7-unknown-linux-gnueabihf/release`.

If you want an installable `.deb` file, run `deb-pack.sh` **after running `cross-build.sh`.**

However, `rusty-engine` is not incredibly heavyweight and can almost certainly be compiled on your Pi without the use of any particular configuration beyond [installing the dependencies.](#dependencies)

Licensing
---
`rusty-engine` is made available under the MIT license. While it uses proprietary codecs, it interacts with these at arm's length via Gstreamer and does not include any implementation of an H.264 encoder or other proprietary encoders in itself.
