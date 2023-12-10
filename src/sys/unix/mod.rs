use crate::sys_common::AsRawSocket;
use libc::{c_int, c_void, getsockopt, setsockopt, socklen_t};
use std::{
    io,
    mem::{self, MaybeUninit},
    net::UdpSocket,
    os::fd::AsRawFd,
};

cfg_if::cfg_if! {
    if #[cfg(any(
        target_os = "linux",
        target_os = "l4re",
        target_os = "android",
        target_os = "emscripten",
    ))] {
        mod linux_like;
        pub use self::linux_like::*;
    } else if #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd",
    ))] {
        mod bsd;
        pub use self::bsd::*;
    } else {
        compile_error!("dontfrag: unsupported platform");
    }
}

pub type RawSocket = c_int;

impl AsRawSocket for UdpSocket {
    #[inline(always)]
    fn as_raw_socket(&self) -> RawSocket {
        self.as_raw_fd()
    }
}

#[cfg(feature = "tokio")]
impl AsRawSocket for tokio::net::UdpSocket {
    #[inline(always)]
    fn as_raw_socket(&self) -> RawSocket {
        self.as_raw_fd()
    }
}

fn getsockopt_int(fd: c_int, level: c_int, name: c_int) -> io::Result<c_int> {
    let mut option = MaybeUninit::<c_int>::uninit();
    let mut len = mem::size_of::<c_int>() as socklen_t;

    // Safety: the pointer arguments are all well-defined.
    let r = unsafe {
        getsockopt(
            fd,
            level,
            name,
            option.as_mut_ptr() as *mut c_void,
            &mut len,
        )
    };
    
    if r == 0 {
        assert_eq!(len, mem::size_of::<c_int>() as socklen_t);
        // Safety: the operation succeeded and the size is equal to c_int.
        // So, option is initialized.
        Ok(unsafe { option.assume_init() })
    } else {
        Err(io::Error::last_os_error())
    }
}

fn setsockopt_int(fd: c_int, level: c_int, name: c_int, option: c_int) -> io::Result<()> {
    let len = mem::size_of::<c_int>() as socklen_t;
    
    // Safety: the pointer arguments are all well-defined.
    let r = unsafe {
        setsockopt(
            fd,
            level,
            name,
            &option as *const c_int as *const c_void,
            len,
        )
    };

    if r == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
