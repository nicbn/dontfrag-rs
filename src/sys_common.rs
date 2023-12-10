use crate::sys;

/// Used to get the underlying OS handle for the socket.
pub trait AsRawSocket {
    fn as_raw_socket(&self) -> sys::RawSocket;
}
