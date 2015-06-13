#![feature(convert)]
extern crate diffusion;
use diffusion::Writer;
use diffusion::MulticastWriter;
fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Multicast message once every second.");
        println!("Usage: {} multicast_ip multicast_port", args[0]);
        return;
    }
    let port : u16 = std::str::FromStr::from_str(args[2].as_str()).unwrap();
    println!("Connecting to {}:{}", args[1], port);
    let mut writer = MulticastWriter::new((args[1].as_str(), port)).unwrap();
    let step = 1i64;
    for time in 0i64..step {
        let message = "This is message No. ".to_string() + time.to_string().as_str();
        writer.write(message.as_bytes()).unwrap();
        println!("{}", message);
        std::thread::sleep_ms((step * 1000) as u32);
    }
}
