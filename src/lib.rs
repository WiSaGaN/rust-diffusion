#![feature(udp)]
#![feature(ip_addr)]
#![feature(collections)]

mod file;
mod multicast;

use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    CorruptSegmentHeader,
    CorruptMsgHeader,
    InsufficientLength(usize),
    IoError(Box<error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CorruptSegmentHeader => "corrupted segment header",
            Error::CorruptMsgHeader => "corruped message header",
            Error::InsufficientLength(..) => "insufficient length",
            Error::IoError(..) => "I/O error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref cause) => Some(&**cause),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(Box::new(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Reader {
    fn read(&mut self) -> Result<Option<Vec<u8>>>;
}

pub trait Writer {
    fn write(&mut self, buf: &[u8]) -> Result<()>;
}

pub use file::{FileWriter, FileReader};
pub use multicast::{MulticastWriter, MulticastReader};
