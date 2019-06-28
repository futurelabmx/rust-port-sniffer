use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("No hay suficientes argumentos");
        } else if args.len() > 4 {
            return Err("Demasiados argumentos");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") &&
                args.len() == 2 {
                println!("Uso: -j para asignar hilos de ejecución
                \r\n      -h o --help para mostrar este mensaje");
                } else if flag.contains("-h") || flag.contains("--help") {
                    return Err("Demasiados argumentos");
                }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
}
