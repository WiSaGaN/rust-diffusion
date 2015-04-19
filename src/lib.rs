#![feature(convert)]
#![feature(udp)]
#![feature(ip_addr)]
#![feature(collections)]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::path::Path;

static FILE_HEADER: &'static [u8] = &['D' as u8, 'F' as u8, 'S' as u8, 'N' as u8];

#[derive(Clone, Copy, Debug)]
pub enum TryReadError {
    Empty,
    Error,
}

pub trait Reader {
    fn try_read(&mut self) -> Result<Vec<u8>, TryReadError>;
}

#[derive(Clone, Copy, Debug)]
pub enum TryWriteError {
    Full,
    Error,
}

pub trait Writer {
    fn try_write(&mut self, buf: &[u8]) -> Result<(), TryWriteError>;
}

pub struct FileReader {
    file: File,
}

impl FileReader {
    pub fn new(path: &Path) -> Option<FileReader> {
        match File::open(path) {
            Ok(mut file) => {
                let mut header = vec![0u8; FILE_HEADER.len()];
                match file.read(&mut header) {
                    Ok(read_length) => if read_length == FILE_HEADER.len() && header.as_slice() == FILE_HEADER {
                        return Some(FileReader{file: file});
                    } else {
                        return None;
                    },
                    Err(..) => return None,
                };
            }
            Err(..) => return None,
        }
    }
}

impl Reader for FileReader {
    fn try_read(&mut self) -> Result<Vec<u8>, TryReadError> {
        let mut header = vec![0u8; std::mem::size_of::<i32>()];
        match self.file.read(&mut header) {
            Ok(header_read_length) => if header_read_length == std::mem::size_of::<i32>() {
                let header_ptr : *const i32 = unsafe { std::mem::transmute(&header[0]) };
                let body_length_number = unsafe { std::ptr::read::<i32>(header_ptr) };
                let body_length = body_length_number as usize;
                let mut buffer = vec![0u8; body_length];
                match self.file.read(&mut buffer) {
                    Ok(read_length) => if read_length == body_length {
                        return Ok(buffer);
                    } else {
                        return Err(TryReadError::Error);
                    },
                    Err(..) => return Err(TryReadError::Error),
                };
            } else {
                return Err(TryReadError::Error);
            },
            Err(..) => return Err(TryReadError::Error),
        };
    }
}

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(path: &Path) -> Option<FileWriter> {
        match File::create(path) {
            Ok(mut file) => {
                match file.write(FILE_HEADER) {
                    Ok(..) => return Some(FileWriter{file: file}),
                    Err(..) => return None,
                }
            }
            Err(..) => return None,
        }
    }
}

impl Writer for FileWriter {
    fn try_write(&mut self, buf: &[u8]) -> Result<(), TryWriteError> {
        let value = buf.len() as i32;
        let header_ptr : *const u8 = unsafe { std::mem::transmute(&value) };
        let header_length = std::mem::size_of::<i32>();
        let slice = unsafe { std::slice::from_raw_parts(header_ptr, header_length) };
        match self.file.write(slice) {
            Ok(..) => match self.file.write(buf) {
                Ok(..) => return Ok(()),
                Err(..) => return Err(TryWriteError::Error),
            },
            Err(..) => return Err(TryWriteError::Error),
        }
    }
}

pub struct MulticastWriter {
    socket: UdpSocket,
    multicast_addr: SocketAddr,
}

impl MulticastWriter {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Option<MulticastWriter> {
        match addr.to_socket_addrs() {
            Ok(mut value_iter) => {
                match value_iter.next() {
                    Some(value) => {
                        match UdpSocket::bind(value) {
                            Ok(socket) => Some(MulticastWriter{ socket: socket, multicast_addr: value }),
                            Err(..) => None,
                        }
                    }
                    None => None,
                }
            }
            Err(..) => None,
        }
    }
}

impl Writer for MulticastWriter {
    fn try_write(&mut self, buf: &[u8]) -> Result<(), TryWriteError> {
        match self.socket.send_to(buf, self.multicast_addr) {
            Ok(..) => Ok(()),
            Err(..) => Err(TryWriteError::Error),
        }
    }
}

pub struct MulticastReader {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl MulticastReader {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Option<MulticastReader> {
        match addr.to_socket_addrs() {
            Ok(mut value_iter) => {
                match value_iter.next() {
                    Some(value) => {
                        match UdpSocket::bind(value) {
                            Ok(socket) => {
                                // FIX ME:
                                socket.join_multicast(&value.ip()).unwrap();
                                let buf_size = 1536usize;
                                let mut buf = std::vec::Vec::with_capacity(buf_size);
                                buf.resize(buf_size, 0u8);
                                Some(MulticastReader{ socket: socket, buf: buf.clone() })
                            },
                            Err(..) => None,
                        }
                    }
                    None => None,
                }
            }
            Err(..) => None,
        }
    }
}

impl Reader for MulticastReader {
    fn try_read(&mut self) -> Result<Vec<u8>, TryReadError> {
        match self.socket.recv_from(&mut self.buf) {
            Ok((length,_)) => {
                let mut data = self.buf.clone();
                data.resize(length, 0u8);
                Ok(data)
            }
            Err(..) => Err(TryReadError::Error)
        }
    }
}
