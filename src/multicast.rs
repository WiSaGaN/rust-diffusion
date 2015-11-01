extern crate net2;

use self::net2::UdpSocketExt;

use std;
use std::net::{SocketAddrV4, UdpSocket};

use super::{Reader, Result, Writer};

/// is writer for multicast.
/// `MulticastWriter` uses the natual UDP packet as message boundary.
#[derive(Debug)]
pub struct MulticastWriter {
    socket: UdpSocket,
    multicast_addr: SocketAddrV4,
}

impl MulticastWriter {
    /// returns a new instance of `MulticastWriter`.
    /// `addr` is the address the sending socket binds to, and also the address that it sends to.
    pub fn new(addr: SocketAddrV4) -> Result<MulticastWriter> {
        let socket = try!(UdpSocket::bind(&addr));
        Ok(MulticastWriter {
            socket: socket,
            multicast_addr: addr,
        })
    }
}

impl Writer for MulticastWriter {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        try!(self.socket.send_to(buf, &self.multicast_addr));
        Ok(())
    }
}

/// is reader for multicast.
/// Reads the UDP packet multicasted from writer. Each packet is a message.
#[derive(Debug)]
pub struct MulticastReader {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl MulticastReader {
    /// returns a new instance of `MulticastReader`.
    /// Binds to `addr`.
    pub fn new(addr: SocketAddrV4) -> Result<MulticastReader> {
        let socket = try!(UdpSocket::bind(&addr));
        try!(socket.join_multicast_v4(&addr.ip(), &std::net::Ipv4Addr::new(0u8, 0u8, 0u8, 0u8)));
        Ok(MulticastReader {
            socket: socket,
            buf: vec![0u8; 1536usize],
        })
    }
}

impl Reader for MulticastReader {
    fn read(&mut self) -> Result<Option<Vec<u8>>> {
        let (length, _) = try!(self.socket.recv_from(&mut self.buf));
        let (data, _) = self.buf.split_at(length);
        Ok(Some(Vec::from(data)))
    }
}
