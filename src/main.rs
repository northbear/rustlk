// use std::prelude::v1::*;

use std::io;
use std::io::prelude::*;

use std::net::{TcpListener,TcpStream};

use std::thread;
// use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;

mod config;

// static mut THREAD_EXEC: Arc<bool> = Arc::new(true); 

fn run_server(ip: &str, ch: mpsc::Receiver<String>) {
    let listener = TcpListener::bind(&ip)
        .expect(&format!("cannot bind to ip socket {}", &ip));

    thread::spawn(move || {
        loop {
            if let Ok((stream, _)) = listener.accept() {
                conn_handler(stream, &ch);
            }
        }
    });
}

fn run_client(ip: &str, ch: mpsc::Receiver<String>) {
    let stream = TcpStream::connect(&ip)
        .expect(&format!("cannot connect to ip {}", &ip));

    thread::spawn(move || {
                conn_handler(stream, &ch);
    });
}

fn main() {

    let conf = config::new();
    match conf.mode() {
        Mode::Usage(msg) => println!("{}", msg)
    }
    return;
    
    let (app_side, strm_side) = mpsc::channel();

    let _ = run_server("127.0.0.1:7878", strm_side);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut line = line.unwrap();
        line.push('\n');
        
        app_side.send(line).unwrap();
    }
}

fn conn_handler(mut stream: TcpStream, msgs: &mpsc::Receiver<String>) {
    let resp = String::from("** received\n");
    stream.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
    
    loop {
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
