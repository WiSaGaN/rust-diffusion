extern crate diffusion;
use diffusion::Reader;
use diffusion::FileReader;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} dfsn_filename", args[0]);
        return;
    }
    let input: Option<Box<dyn std::io::Read>> = if args[1] == "-" {
        Some(Box::new(std::io::stdin()))
    } else {
        Some(Box::new(std::fs::File::open(&std::path::Path::new(&args[1])).unwrap()))
    };
    let mut reader = FileReader::new(input.unwrap()).unwrap();
    loop {
        let value = reader.read().unwrap();
        match value {
            Some(data) => {
                if data.is_ascii() {
                    println!("{}", String::from_utf8(data).unwrap());
                } else {
                    println!("Binary data with length = {}", data.len());
                }
            }
            None => break,
        }
    }
}
