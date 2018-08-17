extern crate diffusion;
extern crate tempfile;

use std::io::{Seek, SeekFrom};

use diffusion::{FileReader, FileWriter, Reader, Writer};
use tempfile::NamedTempFile;

#[test]
fn multiple_message() {
    let mut tempfile = NamedTempFile::new().expect("cannot create temp file");
    {
        let mut writer = FileWriter::new(tempfile.as_file_mut()).expect("cannot create file writer");
        writer.write_multiple(&["Hello, ".as_bytes(), "world.".as_bytes()]).expect("cannot write message 1");
        writer.write("This is diffusion.".as_bytes()).expect("cannot write message 2");
        writer.write_multiple(&["Have a good day, ".as_bytes(), "friend.".as_bytes()]).expect("cannot write message 3");
    }
    tempfile.as_file().seek(SeekFrom::Start(0)).expect("cannot seek to 0");
    {
        let mut reader = FileReader::new(tempfile.as_file()).expect("cannot create file reader");
        let message_1 = reader.read();
        assert_eq!(Ok(Some("Hello, world.".to_owned().into_bytes())), message_1);
        let message_2 = reader.next();
        assert_eq!(Some(Ok("This is diffusion.".to_owned().into_bytes())), message_2);
        let message_3 = reader.read();
        assert_eq!(Ok(Some("Have a good day, friend.".to_owned().into_bytes())), message_3);
    }
}
