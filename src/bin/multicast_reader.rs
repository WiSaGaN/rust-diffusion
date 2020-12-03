use diffusion::Reader;
use diffusion::MulticastReader;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} multicast_ip multicast_port", args[0]);
        return;
    }
    let port: u16 = std::str::FromStr::from_str(&args[2]).unwrap();
    let ipv4: std::net::Ipv4Addr = std::str::FromStr::from_str(&args[1]).unwrap();
    println!("Connecting to {}:{}", args[1], port);
    let mut reader = MulticastReader::new(std::net::SocketAddrV4::new(ipv4, port)).unwrap();
    loop {
        let value = reader.read().unwrap();
        match value {
            Some(data) => println!("{}", String::from_utf8(data).unwrap()),
            None => break,
        }
    }
}
