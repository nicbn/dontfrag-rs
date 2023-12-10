use super::{getsockopt_int, setsockopt_int};
use libc::{
    c_int, IPPROTO_IP, IPPROTO_IPV6, IPV6_MTU_DISCOVER, IPV6_PMTUDISC_DO, IPV6_PMTUDISC_DONT,
    IP_MTU_DISCOVER, IP_PMTUDISC_DO, IP_PMTUDISC_DONT,
};
use std::io;

pub fn dontfrag_v4(socket: c_int) -> io::Result<bool> {
    getsockopt_int(socket, IPPROTO_IP, IP_MTU_DISCOVER).map(|v| v == IP_PMTUDISC_DO)
}

pub fn dontfrag_v6(socket: c_int) -> io::Result<bool> {
    getsockopt_int(socket, IPPROTO_IPV6, IPV6_MTU_DISCOVER).map(|v| v == IPV6_PMTUDISC_DO)
}

pub fn set_dontfrag_v4(socket: c_int, v: bool) -> io::Result<()> {
    setsockopt_int(
        socket,
        IPPROTO_IP,
        IP_MTU_DISCOVER,
        if v { IP_PMTUDISC_DO } else { IP_PMTUDISC_DONT },
    )
}

pub fn set_dontfrag_v6(socket: c_int, v: bool) -> io::Result<()> {
    setsockopt_int(
        socket,
        IPPROTO_IPV6,
        IPV6_MTU_DISCOVER,
        if v {
            IPV6_PMTUDISC_DO
        } else {
            IPV6_PMTUDISC_DONT
        },
    )
}
