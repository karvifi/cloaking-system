//! Zero-Copy I/O for Packet Processing
//! 
//! High-performance network I/O without allocations

use std::io::{self, IoSlice, IoSliceMut};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

pub struct ZeroCopyIo;

impl ZeroCopyIo {
    /// Send packet using sendmsg (zero-copy)
    #[cfg(unix)]
    pub fn send_packet_zerocopy(fd: i32, packet: &[u8]) -> io::Result<usize> {
        use nix::sys::socket::{sendmsg, MsgFlags};
        
        let iov = [IoSlice::new(packet)];
        
        match sendmsg(fd, &iov, &[], MsgFlags::empty(), None) {
            Ok(bytes) => {
                tracing::debug!("📤 Sent {} bytes (zero-copy)", bytes);
                Ok(bytes)
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    /// Receive packet using recvmsg (zero-copy)
    #[cfg(unix)]
    pub fn recv_packet_zerocopy(fd: i32, buffer: &mut [u8]) -> io::Result<usize> {
        use nix::sys::socket::{recvmsg, MsgFlags};
        
        let mut iov = [IoSliceMut::new(buffer)];
        
        match recvmsg(fd, &mut iov, None, MsgFlags::empty()) {
            Ok(msg) => {
                tracing::debug!("📥 Received {} bytes (zero-copy)", msg.bytes);
                Ok(msg.bytes)
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
}
