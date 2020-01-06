`rusty-engine`
===
:gear::steam_locomotive:

`rusty-engine` is a Rust rewrite/redesign/reimagining of The Quadrangles' [`potential-engine` RTSP server. ](https://github.com/BHSSFRC/potential-engine) `rusty-engine` aims to be more easily extensible and added to from a developer standpoint, while Rust helps to cover most of the corner cases and ensure everything fits together. (It's also just plainly more accessible to a novice than C/C++.)

Licensing
---
`rusty-engine` is made available under the MIT license. While it uses proprietary codecs, it interacts with these at arm's length via Gstreamer and does not include any implementation of an H.264 encoder or other proprietary encoders in itself.