use crate::sys_common;
use std::{
    io,
    mem::{self, MaybeUninit},
    net::UdpSocket,
    os::raw::c_int,
};
use windows_sys::{
    core::{PCSTR, PSTR},
    Win32::Networking::WinSock::{
        getsockopt, setsockopt, socklen_t, IPPROTO_IP, IPPROTO_IPV6, IPV6_DONTFRAG,
        IP_DONTFRAGMENT, SOCKET,
    },
};

pub type RawSocket = SOCKET;

impl sys_common::AsRawSocket for UdpSocket {
    #[inline(always)]
    fn as_raw_socket(&self) -> RawSocket {
        std::os::windows::io::AsRawSocket::as_raw_socket(self) as SOCKET
    }
}

#[cfg(feature = "tokio")]
impl sys_common::AsRawSocket for tokio::net::UdpSocket {
    #[inline(always)]
    fn as_raw_socket(&self) -> RawSocket {
        std::os::windows::io::AsRawSocket::as_raw_socket(self) as SOCKET
    }
}

pub fn dontfrag_v4(socket: SOCKET) -> io::Result<bool> {
    // Safety: IP_DONTFRAGMENT is defined within IPPROTO_IP
    let r = unsafe { getsockopt_int(socket, IPPROTO_IP, IP_DONTFRAGMENT) };
    r.map(|v| v != 0)
}

pub fn dontfrag_v6(socket: SOCKET) -> io::Result<bool> {
    // Safety: IPV6_DONTFRAG is defined within IPPROTO_IPV6
    let r = unsafe { getsockopt_int(socket, IPPROTO_IPV6, IPV6_DONTFRAG) };
    r.map(|v| v != 0)
}

pub fn set_dontfrag_v4(socket: SOCKET, v: bool) -> io::Result<()> {
    // Safety: IP_DONTFRAGMENT is defined within IPPROTO_IP
    unsafe { setsockopt_int(socket, IPPROTO_IP, IP_DONTFRAGMENT, v as c_int) }
}

pub fn set_dontfrag_v6(socket: SOCKET, v: bool) -> io::Result<()> {
    // Safety: IPV6_DONTFRAG is defined within IPPROTO_IPV6
    unsafe { setsockopt_int(socket, IPPROTO_IPV6, IPV6_DONTFRAG, v as c_int) }
}

/// # Safety
///
/// `name` must be an option defined within `level`.
unsafe fn getsockopt_int(fd: SOCKET, level: c_int, name: c_int) -> io::Result<c_int> {
    let mut option = MaybeUninit::<c_int>::uninit();
    let mut len = mem::size_of::<c_int>() as socklen_t;

    // Safety: `name` being defined within `level` is guaranteed by the
    // setsockopt_int safety contract.
    // The pointer arguments are all well-defined.
    let r = unsafe { getsockopt(fd, level, name, option.as_mut_ptr() as PSTR, &mut len) };
    
    if r == 0 {
        assert_eq!(len, mem::size_of::<c_int>() as socklen_t);
        // Safety: the operation succeeded and the size is equal to c_int.
        // So, option is initialized.
        Ok(unsafe { option.assume_init() })
    } else {
        Err(io::Error::last_os_error())
    }
}

/// # Safety
///
/// `name` must be an option defined within `level`.
unsafe fn setsockopt_int(fd: SOCKET, level: c_int, name: c_int, option: c_int) -> io::Result<()> {
    let len = mem::size_of::<c_int>() as socklen_t;
    
    // Safety: `name` being defined within `level` is guaranteed by the
    // setsockopt_int safety contract.
    // The pointer arguments are all well-defined.
    let r = unsafe { setsockopt(fd, level, name, &option as *const c_int as PCSTR, len) };

    if r == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
