extern crate diffusion;
use diffusion::Reader;
use diffusion::MulticastReader;
fn main() {
    let args = std::os::args();
    if args.len() < 3 {
        println!("Usage: {} multicast_ip multicast_port", args[0]);
        return;
    }
    let port : u16 = std::str::FromStr::from_str(args[2].as_slice()).unwrap();
    println!("Connecting to {}:{}", args[1], port);
    let mut reader = MulticastReader::new((args[1].as_slice(), port)).unwrap();
    loop {
        match reader.try_read() {
            Ok(value) => println!("{}", String::from_utf8(value).unwrap()),
            Err(..) => break,
        }
    }
}
