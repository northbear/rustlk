// extern crate getopts;

use std::env;
use std::net::SocketAddr;

// use self::getopts::{Options,Matches};

const USAGE_MESSAGE: &'static str = "\
Simple single-channel chat utility written in Rust
Usage:
\trustlk [ -s] IPADDR:PORT
\trustlk -h

Options:
\tIPADDR:PORT\trun as client and connect to server at IPADDR:PORT
\t-s\t\trun as server and bind to IPADDR:PORT
\t-h\t\tprint help (this page)
";

pub enum Mode {
    Server(String),
    Client(String),
    Usage(String),
    WrongConf(String),
}

pub struct ConfError(pub String);

pub struct Config {
    args: Vec<String>
}

pub fn new() -> Config {
    let args: Vec<String> = env::args().collect();
    Config{ args: args }
}


fn valid_ip(s: &String) -> bool {
    match s.parse::<SocketAddr>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl Config {

    pub fn usage(&self) -> String {
        USAGE_MESSAGE.to_string()
    }
    
    pub fn mode(&self) -> Mode {
        let opts = &(self.args);
        
        if opts.len() == 3 {
            if opts[1] == "-s" && valid_ip(&opts[2]) {
                let ipaddr = opts[2].clone();
                return Mode::Server(ipaddr)
            } else {
                return Mode::WrongConf(String::from("wrong parameters given"))
            }
        } else if opts.len() == 2 {
            if opts[1] == "-h" {
                return Mode::Usage(USAGE_MESSAGE.to_string())
            } else if valid_ip(&opts[1]) {
                let ipaddr = opts[1].clone();
                return Mode::Client(ipaddr)
            } else {
                return Mode::WrongConf(String::from("wrong parameters given"))
            }
        } else {
            return Mode::Usage(USAGE_MESSAGE.to_string())
        }
    }  
}
