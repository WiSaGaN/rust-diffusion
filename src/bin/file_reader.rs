extern crate diffusion;
use diffusion::Reader;
use diffusion::FileReader;
fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        println!("Usage: {} dfsn_filename", args[0]);
        return;
    }
    println!("Test started.");
    let mut reader = FileReader::new(&Path::new(args[1].as_slice())).unwrap();
    loop {
        match reader.try_read() {
            Ok(value) => println!("{}", String::from_utf8(value).unwrap()),
            Err(..) => break,
        }
    }
    println!("Test finished.");
}
