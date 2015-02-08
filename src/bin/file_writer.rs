extern crate diffusion;
use diffusion::Writer;
use diffusion::FileWriter;
fn main() {
    let args = std::os::args();
    if args.len() < 3 {
        println!("Usage: {} text_filename dfsn_filename", args[0]);
        return;
    }
    let mut text_file = std::old_io::BufferedReader::new(std::old_io::File::open(&Path::new(args[1].as_slice())));

    let mut writer = FileWriter::new(&Path::new(args[2].as_slice())).unwrap();
    for line in text_file.lines() {
        writer.try_write(line.unwrap().trim().as_bytes());
    }
}
