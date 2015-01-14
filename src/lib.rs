use std::io::File;
use std::num::ToPrimitive;

static FILE_HEADER: &'static str = "DFSN";

pub enum TryReadError {
    Empty,
    Disconnected,
}

impl Copy for TryReadError {
}

pub trait Reader {
    fn try_read(&mut self) -> Result<Vec<u8>, TryReadError>;
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
                            Err(..) => return Err(TryReadError::Disconnected),
                        };
                    },
                    None => return Err(TryReadError::Disconnected),
                };
            },
            Err(..) => return Err(TryReadError::Disconnected),
        };
    }
}
