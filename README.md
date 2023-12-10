# dontfrag-rs

Rust library for setting Don't Fragment option (DF) on UDP sockets.

## Platform support

* BSD-like: macOS, iOS, tvOS, watchOS, FreeBSD, Dragonfly, OpenBSD, NetBSD
* Linux-like: Linux, L4RE, Android, Emscripten
* Windows

PRs for additional platforms are welcome!

## Optional features

* `tokio`: Adds support for `tokio::net::UdpSocket`.
