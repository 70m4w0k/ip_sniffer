use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in &args {
        println!("{}", i);
    }

    println!("{:?}", args);
}
