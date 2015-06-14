use std;
use std::io::{Read, Write};

use super::{Error, Result, Reader, Writer};

const FILE_HEADER : &'static [u8] = b"DFSN";

pub struct FileReader<T> where T: Read {
    file: T,
}

impl<T> FileReader<T> where T: Read {
    pub fn new(mut file:T) -> Result<FileReader<T>> {
        let mut header = vec![0u8; FILE_HEADER.len()];
        let read_length = try!(file.read(&mut header));
        if read_length == FILE_HEADER.len() && &*header.into_boxed_slice() == FILE_HEADER {
            return Ok(FileReader{file: file});
        } else {
            return Err(Error::CorruptSegmentHeader);
        }
    }
}

impl<T> Reader for FileReader<T> where T: Read {
    fn read(&mut self) -> Result<Option<Vec<u8>>> {
        let mut header = vec![0u8; std::mem::size_of::<i32>()];
        let header_read_length = try!(self.file.read(&mut header));
        if header_read_length == std::mem::size_of::<i32>() {
            let header_ptr : *const i32 = unsafe { std::mem::transmute(&header[0]) };
            let body_length_number = unsafe { std::ptr::read::<i32>(header_ptr) };
            let body_length = body_length_number as usize;
            let mut remaining_length = body_length;
            let mut full_buffer = Vec::with_capacity(body_length);
            while remaining_length > 0 {
                let mut buffer = vec![0u8; remaining_length];
                let read_length = try!(self.file.read(&mut buffer));
                if read_length == 0 {
                    return Err(Error::InsufficientLength(remaining_length));
                } else {
                    remaining_length -= read_length;
                }
                buffer.truncate(read_length);
                full_buffer.extend(buffer);
            }
            return Ok(Some(full_buffer));
        } else if header_read_length == 0 {
            return Ok(None);
        } else {
            return Err(Error::CorruptMsgHeader);
        }
    }
}

pub struct FileWriter<T> where T: Write {
    file: T,
}

impl<T> FileWriter<T> where T: Write {
    pub fn new(mut file: T) -> Result<FileWriter<T>> {
        try!(file.write(FILE_HEADER));
        Ok(FileWriter{file: file})
    }
}

impl<T> Writer for FileWriter<T> where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<()> {
        let value = buf.len() as i32;
        let header_ptr : *const u8 = unsafe { std::mem::transmute(&value) };
        let header_length = std::mem::size_of::<i32>();
        let slice = unsafe { std::slice::from_raw_parts(header_ptr, header_length) };
        try!(self.file.write(slice));
        try!(self.file.write(buf));
        Ok(())
    }
}
