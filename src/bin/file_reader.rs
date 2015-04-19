#![feature(convert)]
extern crate diffusion;
use std::path::Path;
use diffusion::Reader;
use diffusion::FileReader;
fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} dfsn_filename", args[0]);
        return;
    }
    println!("Test started.");
    let mut reader = FileReader::new(&Path::new(args[1].as_str())).unwrap();
    loop {
        match reader.try_read() {
            Ok(value) => println!("{}", String::from_utf8(value).unwrap()),
            Err(..) => break,
        }
    }
    println!("Test finished.");
}
