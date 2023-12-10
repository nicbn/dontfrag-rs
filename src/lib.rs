//! # dontfrag
//!
//! Library for setting Don't Fragment option (DF) on UDP sockets.
//!
//! Example usage:
//!
//! ```no_run
//! use dontfrag::UdpSocketExt;
//!
//! let socket = UdpSocket::bind("127.0.0.1:3400").unwrap();
//! socket.set_dontfrag_v4(true).unwrap();
//! ```
//!
//! ## Optional features
//! 
//! * `tokio`: Adds support for `tokio::net::UdpSocket`.

#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(doc, feature(doc_cfg))]

use std::{io, net::UdpSocket};
use sys_common::AsRawSocket;

mod sys;
mod sys_common;

mod private {
    pub trait Sealed {}
}

/// Extension trait that provides methods for getting and setting the
/// Don't Fragment (DF) option on UDP sockets.
pub trait UdpSocketExt: private::Sealed {
    /// Gets the value of the `IP_DONTFRAG` (or equivalent) option for this
    /// socket.
    ///
    /// For more information about this option, see
    /// [`UdpSocketExt::set_dontfrag_v4`].
    fn dontfrag_v4(&self) -> io::Result<bool>;

    /// Gets the value of the `IPV6_DONTFRAG` (or equivalent) option for this
    /// socket.
    ///
    /// For more information about this option, see
    /// [`UdpSocketExt::set_dontfrag_v6`].
    fn dontfrag_v6(&self) -> io::Result<bool>;

    /// Sets the value of the `IP_DONTFRAG` (or equivalent) option for this
    /// socket.
    ///
    /// This option sets the Don't Fragment flag (DF) on IP packets.
    ///
    /// # Platform-specific behavior
    ///
    /// On BSD-like systems (including Darwin), this sets the `IP_DONTFRAG`
    /// option.
    ///
    /// On Linux-like systems, this sets the `IP_MTU_DISCOVER` option to
    /// either `IP_PMTUDISC_DONT` or `IP_PMTUDISC_DO`.
    ///
    /// On Windows, this sets the `IP_DONTFRAGMENT` option.
    fn set_dontfrag_v4(&self, v: bool) -> io::Result<()>;

    /// Sets the value of the `IPV6_DONTFRAG` (or equivalent) option for this
    /// socket.
    ///
    /// This option sets the Don't Fragment flag (DF) on IP packets.
    ///
    /// # Platform-specific behavior
    ///
    /// On BSD-like systems (including Darwin) and Windows, this sets
    /// the `IPV6_DONTFRAG` option.
    ///
    /// On Linux-like systems, this sets the `IPV6_MTU_DISCOVER` option to
    /// either `IPV6_PMTUDISC_DONT` or `IPV6_PMTUDISC_DO`.
    fn set_dontfrag_v6(&self, v: bool) -> io::Result<()>;
}

impl private::Sealed for UdpSocket {}
impl UdpSocketExt for UdpSocket {
    #[inline(always)]
    fn dontfrag_v4(&self) -> io::Result<bool> {
        sys::dontfrag_v4(self.as_raw_socket())
    }

    #[inline(always)]
    fn dontfrag_v6(&self) -> io::Result<bool> {
        sys::dontfrag_v6(self.as_raw_socket())
    }

    #[inline(always)]
    fn set_dontfrag_v4(&self, v: bool) -> io::Result<()> {
        sys::set_dontfrag_v4(self.as_raw_socket(), v)
    }

    #[inline(always)]
    fn set_dontfrag_v6(&self, v: bool) -> io::Result<()> {
        sys::set_dontfrag_v6(self.as_raw_socket(), v)
    }
}

#[cfg(feature = "tokio")]
impl private::Sealed for tokio::net::UdpSocket {}
#[cfg(feature = "tokio")]
#[doc(cfg(feature = "tokio"))]
impl UdpSocketExt for tokio::net::UdpSocket {
    #[inline(always)]
    fn dontfrag_v4(&self) -> io::Result<bool> {
        sys::dontfrag_v4(self.as_raw_socket())
    }

    #[inline(always)]
    fn dontfrag_v6(&self) -> io::Result<bool> {
        sys::dontfrag_v6(self.as_raw_socket())
    }

    #[inline(always)]
    fn set_dontfrag_v4(&self, v: bool) -> io::Result<()> {
        sys::set_dontfrag_v4(self.as_raw_socket(), v)
    }

    #[inline(always)]
    fn set_dontfrag_v6(&self, v: bool) -> io::Result<()> {
        sys::set_dontfrag_v6(self.as_raw_socket(), v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retain_dontfrag_v4() {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        socket.set_dontfrag_v4(true).unwrap();
        assert!(socket.dontfrag_v4().unwrap());
        socket.set_dontfrag_v4(false).unwrap();
        assert!(!socket.dontfrag_v4().unwrap());
    }

    #[test]
    fn retain_dontfrag_v6() {
        let socket = UdpSocket::bind("::1:0").unwrap();
        socket.set_dontfrag_v6(true).unwrap();
        assert!(socket.dontfrag_v6().unwrap());
        socket.set_dontfrag_v6(false).unwrap();
        assert!(!socket.dontfrag_v6().unwrap());
    }
}
