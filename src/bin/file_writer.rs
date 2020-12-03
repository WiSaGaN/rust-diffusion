use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use diffusion::Writer;
use diffusion::FileWriter;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} text_filename dfsn_filename", args[0]);
        return;
    }
    match File::open(&Path::new(&args[1])) {
        Ok(source_text) => {
            let source = std::io::BufReader::new(source_text);
            match File::create(&Path::new(&args[2])) {
                Ok(target) => {
                    let mut writer = FileWriter::new(target).unwrap();
                    for line in source.lines() {
                        writer.write(line.unwrap().trim().as_bytes()).unwrap();
                    }
                }
                Err(err) => println!("Error when creating target file: {:?}", err),
            }
        }
        Err(err) => println!("Error when opening source text file: {:?}", err),
    }
}
