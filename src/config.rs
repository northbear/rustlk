extern crate getopts;

use std::env;
use std::net::SocketAddr;

use self::getopts::{Options,Matches};

const USAGE_MESSAGE: &'static str = "\
Simple single-channel chat utility written in Rust
Usage: rustlk [ -s IPADDR:PORT | -c IPADDR:PORT | -h ]\
";

pub enum Mode {
    Server(SocketAddr),
    Client(SocketAddr),
    Usage,
    WrongConf(String),
}

pub struct ConfError(pub String);

pub struct Config {
    options: Options
}

pub fn new() -> Config {
    Config{ options: setup_options() }
}

fn setup_options() -> Options {
    let mut parser = Options::new();
    parser.optopt("s", "", "run as server and bind to given socket", "IPADDR:PORT");
    parser.optopt("c", "", "run as client and connect to server provided", "IPADDR:PORT");
    parser.optflag("h", "help", "provides usage info");
    
    parser
}
    
fn parse_ip(s: String) -> Result<SocketAddr, ConfError> {
    let sock: SocketAddr = match s.parse() {
        Ok(addr) => addr,
        Err(e)   => return Err(ConfError(e.to_string())),
    };
    Ok(sock)
}

fn parse_options(m: Matches) -> Mode {
    if m.opt_present("h") {
        return Mode::Usage
    }
    if m.opt_present("s") && m.opt_present("c") {
        let msg = String::from("only one of client or server modes should be specified");
        return Mode::WrongConf(msg)
    }
    match m.opt_str("s") {
        Some(bind) => {
            match parse_ip(bind) {
                Ok(bind)          => return Mode::Server(bind),
                Err(ConfError(s)) => return Mode::WrongConf(s),
            }
        },
        None => {}
    };
    match m.opt_str("c") {
        Some(ip) => {
            match parse_ip(ip) {
                Ok(ip)          => return Mode::Client(ip),
                Err(ConfError(s)) => return Mode::WrongConf(s),
            }
        },
        None => {}
    };
    Mode::WrongConf(String::from("no required options provided"))
}

impl Config {

    pub fn usage(&self) -> String {
        self.options.usage(USAGE_MESSAGE)
    }
    
    pub fn mode(&self) -> Mode {
        let opts: Vec<String> = env::args().collect();
        let parser = setup_options();

        let mode: Mode = match parser.parse(&opts[1..]) {
            Ok(m) => parse_options(m),
            Err(e) => return Mode::WrongConf(e.to_string()),
        };
        mode
    }  
}
