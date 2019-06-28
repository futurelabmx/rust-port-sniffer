use std::env;
use std::net::IpAddr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let flag = args[1].clone();
    let threads = args[2].clone();
    let ipaddr = args[3].clone();
}
