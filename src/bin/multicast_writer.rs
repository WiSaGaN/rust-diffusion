extern crate diffusion;
use diffusion::Writer;
use diffusion::MulticastWriter;
fn main() {
    let args = std::os::args();
    if args.len() < 3 {
        println!("Multicast message once every second.");
        println!("Usage: {} multicast_ip multicast_port", args[0]);
        return;
    }
    let port : u16 = std::str::FromStr::from_str(args[2].as_slice()).unwrap();
    println!("Connecting to {}:{}", args[1], port);
    let mut writer = MulticastWriter::new((args[1].as_slice(), port)).unwrap();
    let step = 1i64;
    let mut count = std::iter::count(0i64, step);
    for time in count {
        let message = "This is message No. ".to_string() + time.to_string().as_slice();
        writer.try_write(message.as_bytes());
        println!("{}", message);
        std::io::timer::sleep(std::time::Duration::seconds(step));
    }
}
