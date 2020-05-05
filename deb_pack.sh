#!/usr/bin/env bash
set -e

for last in "$@"; do :; done

if [ "${last}" == "clean" ]; then
    rm pkg-debian/DEBIAN/md5sums
    rm -rf pkg-debian/usr
    exit
fi

mkdir -p pkg-debian/usr/bin
mkdir -p pkg-debian/usr/share/doc/rusty-engine

# copy files into place
cp ./target/armv7-unknown-linux-gnueabihf/release/rusty-engine pkg-debian/usr/bin
cp ./README.md pkg-debian/usr/share/doc/rusty-engine
cat > pkg-debian/usr/share/doc/rusty-engine/copyright <<EOF
Upstream Name: rusty-engine
Source: https://github.com/opensight-cv/rusty-engine
Copyright: Caleb Xavier Berger <caleb.x.berger@gmail.com>
License: MIT
MIT License
 .
Copyright (c) 2020 OpenSight contributors
 .
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
 .
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
 .
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 .
EOF


cd pkg-debian
find . -type f ! -regex '.*?debian-binary.*' ! -regex '.*?DEBIAN.*' -printf '%P ' | xargs md5sum > DEBIAN/md5sums
cd ..
dpkg -b pkg-debian/ rusty-engine_0.2.3_armhf.deb
