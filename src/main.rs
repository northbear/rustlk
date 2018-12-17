extern crate ctrlc;

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
//use std::process;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

mod config;
use config::Mode;

type ExecFlag = Arc<AtomicBool>;
type Message = String;

fn run_server(ip: &str, exec: ExecFlag) -> mpsc::Sender<Message> {
    let (app_side, ch) = mpsc::channel();

    let listener = TcpListener::bind(&ip)
        .expect(&format!("cannot bind to ip socket {}", &ip));

    let _child = thread::spawn(move || {
        loop {
            if let Ok((stream, _)) = listener.accept() {
                let exec = exec.clone();
                conn_handler(stream, &ch, exec);
            }
        }
    });
    // return (app_side, child);
    return app_side;
}

fn run_client(ip: &str, exec: ExecFlag) -> mpsc::Sender<Message> {
    let (app_side, ch) = mpsc::channel();
    let stream = TcpStream::connect(&ip)
        .expect(&format!("cannot connect to ip {}", &ip));

    let _child = thread::spawn(move || {
                conn_handler(stream, &ch, exec);
    });
    // return (app_side, child);
    return app_side;
}

fn main() {

    let conf = config::new();
    let execute = Arc::new(AtomicBool::new(true));
    let sender: mpsc::Sender<Message>;
    // let mut proc: thread::JoinHandle<()>;
    
    match conf.mode() {
        Mode::Server(ip_sock) => {
            let execute = execute.clone();
            sender = run_server(&ip_sock, execute);
        },
        Mode::Client(ip_sock) => {
            let execute = execute.clone();
            sender = run_client(&ip_sock, execute);
        },
        Mode::WrongConf(_) | Mode::Usage(_) => {
            println!("{}", conf.usage());
            return;
        },
    }

    // let exec = execute.clone();
    // ctrlc::set_handler(move || {
    //     println!("start processing handler Ctrl-C");
    //     exec.store(false, Ordering::Relaxed);
    //     process::exit(1);
    // }).expect("error setting handler of ctrl-c");
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut line = line.unwrap();
        line.push('\n');
        
        sender.send(line).unwrap();
    }
}

fn conn_handler(mut stream: TcpStream, msgs: &mpsc::Receiver<String>, exec: ExecFlag) {
    let resp = String::from("** received\n");
    stream.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
    
    while exec.load(Ordering::Relaxed) {
        let mut buffer = [0; 512];

        if let Ok(length) = stream.read(&mut buffer) {
            if length == 0 { break };

            let received = String::from_utf8_lossy(&buffer[..]);
            print!("{}", received);
            let _ = stream.write(resp.as_bytes());
        }
        
        if let Ok(message) = msgs.try_recv() {
            stream.write(message.as_bytes()).unwrap();
        }
    }
}
