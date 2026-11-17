use super::{getsockopt_int, setsockopt_int};
use libc::{c_int, IPPROTO_IPV6, IPV6_DONTFRAG};
#[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly")))]
use libc::{IPPROTO_IP, IP_DONTFRAG};
use std::io;

#[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly")))]
pub fn dontfrag_v4(socket: c_int) -> io::Result<bool> {
    getsockopt_int(socket, IPPROTO_IP, IP_DONTFRAG).map(|v| v != 0)
}

pub fn dontfrag_v6(socket: c_int) -> io::Result<bool> {
    getsockopt_int(socket, IPPROTO_IPV6, IPV6_DONTFRAG).map(|v| v != 0)
}

#[cfg(not(any(target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly")))]
pub fn set_dontfrag_v4(socket: c_int, v: bool) -> io::Result<()> {
    setsockopt_int(socket, IPPROTO_IP, IP_DONTFRAG, v as c_int)
}

pub fn set_dontfrag_v6(socket: c_int, v: bool) -> io::Result<()> {
    setsockopt_int(socket, IPPROTO_IPV6, IPV6_DONTFRAG, v as c_int)
}

// These kernels do not expose FreeBSD's per-socket IPv4 IP_DONTFRAG option.
// Do not invent an option number or report success without changing the socket.
#[cfg(any(target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly"))]
pub fn dontfrag_v4(_socket: c_int) -> io::Result<bool> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "IPv4 Don't Fragment socket option is unavailable",
    ))
}

#[cfg(any(target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly"))]
pub fn set_dontfrag_v4(_socket: c_int, _v: bool) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "IPv4 Don't Fragment socket option is unavailable",
    ))
}
