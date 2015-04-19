#![feature(convert)]
extern crate diffusion;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use diffusion::Writer;
use diffusion::FileWriter;
fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} text_filename dfsn_filename", args[0]);
        return;
    }
    let text_file = std::io::BufReader::new(File::open(&Path::new(args[1].as_str())).unwrap());

    let mut writer = FileWriter::new(&Path::new(args[2].as_str())).unwrap();
    for line in text_file.lines() {
        writer.try_write(line.unwrap().trim().as_bytes()).unwrap();
    }
}
