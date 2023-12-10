use super::{getsockopt_int, setsockopt_int};
use libc::{c_int, IPPROTO_IP, IPPROTO_IPV6, IPV6_DONTFRAG, IP_DONTFRAG};
use std::io;

pub fn dontfrag_v4(socket: c_int) -> io::Result<bool> {
    getsockopt_int(socket, IPPROTO_IP, IP_DONTFRAG).map(|v| v != 0)
}

pub fn dontfrag_v6(socket: c_int) -> io::Result<bool> {
    getsockopt_int(socket, IPPROTO_IPV6, IPV6_DONTFRAG).map(|v| v != 0)
}

pub fn set_dontfrag_v4(socket: c_int, v: bool) -> io::Result<()> {
    setsockopt_int(socket, IPPROTO_IP, IP_DONTFRAG, v as c_int)
}

pub fn set_dontfrag_v6(socket: c_int, v: bool) -> io::Result<()> {
    setsockopt_int(socket, IPPROTO_IPV6, IPV6_DONTFRAG, v as c_int)
}
