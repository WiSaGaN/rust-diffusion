use diffusion::Writer;
use diffusion::MulticastWriter;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Multicast message once every second.");
        println!("Usage: {} multicast_ip multicast_port", args[0]);
        return;
    }
    let port: u16 = std::str::FromStr::from_str(&args[2]).unwrap();
    let ipv4: std::net::Ipv4Addr = std::str::FromStr::from_str(&args[1]).unwrap();
    println!("Connecting to {}:{}", args[1], port);
    let mut writer = MulticastWriter::new(std::net::SocketAddrV4::new(ipv4, port)).unwrap();
    let step = 1i64;
    for time in 0i64..step {
        let message = "This is message No. ".to_string() + &time.to_string();
        writer.write(message.as_bytes()).unwrap();
        println!("{}", message);
        sleep(Duration::from_secs(1u64));
    }
}
