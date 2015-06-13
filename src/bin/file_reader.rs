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
    let file = std::fs::File::open(&Path::new(&args[1])).unwrap();
    let mut reader = FileReader::new(file).unwrap();
    loop {
        let value = reader.read().unwrap();
        match value {
            Some(data) => println!("{}", String::from_utf8(data).unwrap()),
            None => break,
        }
    }
    println!("Test finished.");
}
