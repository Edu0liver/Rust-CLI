use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{ Sender, channel };
use std::thread;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {

        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            })
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want\r\n-h ou -help to show this help message");
                return Err("help");

            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");

            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not valid IPADDR, must be a IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };

                return Ok(Arguments {
                    flag,
                    ipaddr,
                    threads
                });

            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|error| {
        if error.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, error);
            process::exit(0);
        }
    });

    

}
 