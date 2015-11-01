#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! # Diffusion
//! Diffusion is a static library that provides several transport with a unified interface for
//! messages based sub-pub style communication.


mod file;
mod multicast;

use std::convert::From;
use std::{error, fmt};

/// represents errors that can be encountered during the usage of of reader and writer.
#[derive(Debug)]
pub enum Error {
    /// indicates corruption when initializing the reader. This can only happens in a file.
    CorruptSegmentHeader,
    /// indicates corruption when reading a message. This can only happens in a file.
    CorruptMsgHeader,
    /// indicates possibily a curruption. `usize` means the number of bytes it need in addition to
    /// what is already in there. This can only happens in a file.
    InsufficientLength(usize),
    /// indicates there is an IO error happening during reading or writing. This can happen in all
    /// transport types.
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
            Error::IoError(ref cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(Box::new(err))
    }
}

/// is an alias for crate level result derived from the crate level `Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// is the general trait for all readers.
pub trait Reader {
    /// returns a message if there is one. And returns any error if it cannot return the message.
    /// It returns a `Ok(None)` if file reader has reached to the end of the file, or there is no
    /// new message currently, in which case user should retry.
    fn read(&mut self) -> Result<Option<Vec<u8>>>;
}

/// is the general trait for all writers.
pub trait Writer {
    /// returns `Ok(())` if write is successful.
    fn write(&mut self, buf: &[u8]) -> Result<()>;
}

pub use file::{FileReader, FileWriter};
pub use multicast::{MulticastReader, MulticastWriter};
