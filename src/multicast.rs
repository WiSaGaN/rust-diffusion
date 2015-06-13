use std;
use std::net::{ToSocketAddrs, UdpSocket};
use std::io;

use super::{Error, Result, Reader, Writer};

pub struct MulticastWriter<T> where T: ToSocketAddrs {
    socket: UdpSocket,
    multicast_addr: T,
}

impl<T> MulticastWriter<T> where T: ToSocketAddrs + Clone {
    pub fn new(addr: T) -> Result<MulticastWriter<T>> {
        let socket = try!(UdpSocket::bind(addr.clone()));
        Ok(MulticastWriter{ socket: socket, multicast_addr: addr })
    }
}

impl<T> Writer for MulticastWriter<T> where T: ToSocketAddrs + Clone {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        try!(self.socket.send_to(buf, self.multicast_addr.clone()));
        Ok(())
    }
}

pub struct MulticastReader {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl MulticastReader {
    pub fn new<A>(addr: A) -> Result<MulticastReader> where A: ToSocketAddrs + Clone {
        let socket = try!(UdpSocket::bind(addr.clone()));
        match try!(addr.to_socket_addrs()).next() {
            Some(addr) => try!(socket.join_multicast(&addr.ip())),
            None => return Err(Error::from(io::Error::new(io::ErrorKind::InvalidInput,
                                   "no addresses to join multicast"))),
        }
        let buf_size = 1536usize;
        let mut buf = std::vec::Vec::with_capacity(buf_size);
        buf.resize(buf_size, 0u8);
        Ok(MulticastReader{ socket: socket, buf: buf.clone() })
    }
}

impl Reader for MulticastReader {
    fn read(&mut self) -> Result<Option<Vec<u8>>> {
        let (length, _) = try!(self.socket.recv_from(&mut self.buf));
        let mut data = self.buf.clone();
        data.truncate(length);
        Ok(Some(data))
    }
}
