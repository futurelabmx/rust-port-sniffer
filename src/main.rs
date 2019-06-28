use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX:u16 = 65535;

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
                } else if flag.contains("-j") {
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(s) => s,
                        Err(_) => return Err("No es una dirección IP válida")
                    };

                    let threads = match args[2].parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("Fallo al leer el número de hilos")
                    };
                    return Ok(Arguments{threads, flag, ipaddr});
                } else {
                    return Err("Error de Sintáxis");
                }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port +1;
    loop {
       
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} error leyendo argumentos {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }
}
