use std::io::File;
use std::num::ToPrimitive;
use std::io::net::ip;
use std::io::net::udp::UdpSocket;

static FILE_HEADER: &'static str = "DFSN";

pub enum TryReadError {
    Empty,
    Error,
}

impl Copy for TryReadError {
}

pub trait Reader {
    fn try_read(&mut self) -> Result<Vec<u8>, TryReadError>;
}

pub enum TryWriteError {
    Full,
    Error,
}

impl Copy for TryWriteError {
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
                match file.read_exact(FILE_HEADER.len()) {
                    Ok(file_header) => if file_header == FILE_HEADER.as_bytes() {
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
        match self.file.read_le_i32() {
            Ok(body_length_number) => {
                match body_length_number.to_uint() {
                    Some(body_length) => {
                        match self.file.read_exact(body_length) {
                            Ok(value) => return Ok(value),
                            Err(..) => return Err(TryReadError::Error),
                        };
                    },
                    None => return Err(TryReadError::Error),
                };
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
                match file.write_str(FILE_HEADER) {
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
        match buf.len().to_i32() {
            Some(value) => match self.file.write_le_i32(value) {
                Ok(..) => match self.file.write(buf) {
                    Ok(..) => return Ok(()),
                    Err(..) => return Err(TryWriteError::Error),
                },
                Err(..) => return Err(TryWriteError::Error),
            },
            None => return Err(TryWriteError::Error),
        }
    }
}

pub struct MulticastWriter {
    socket: UdpSocket,
    multicast_addr: ip::SocketAddr,
}

impl MulticastWriter {
    pub fn new<A: ip::ToSocketAddr>(addr: A) -> Option<MulticastWriter> {
        match addr.to_socket_addr() {
            Ok(value) => {
                match UdpSocket::bind(value) {
                    Ok(socket) => Some(MulticastWriter{ socket: socket, multicast_addr: value }),
                    Err(..) => None,
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
