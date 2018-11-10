
// extern crate tokio;


use std::io;
use std::io::{Write, Read, BufRead};
use std::sync::mpsc;
use std::net::{SocketAddr};
// use std::net::{TcpListener,TcpStream};

mod config;
    
use config::ExecMode;

// struct SockPair(impl io::Read, impl io::Write);

// use tokio::prelude::*;
// use tokio::net::TcpListener;
// use tokio::io::{copy,stdin,stdout};

// trait Channel {    
// }

// struct IoChannel{
//     sender: mpsc::Sender<String>,
//     receiver: mpsc::Receiver<String>,
// }



// fn start_server(sa: &SocketAddr) -> Result<IoChannel, io::Error> {
//     let ch = mpsc::channel();
//     let ioc = IoChannel{ sender: ch.0, receiver: ch.1 };
//     println!("{}", sa.to_string());
//     Ok(ioc)
// }

// fn start_client(conn: &SocketAddr) -> Result<IoChannel, io::Error> {
//     let ch = mpsc::channel();
//     let ioc = IoChannel{ sender: ch.0, receiver: ch.1 };
//     // let sender = TcpStream::connect(conn).unwrap();
//     println!("{}", conn.to_string());
//     Ok(ioc)
// }

// struct ConfError();

enum Event {
    Ctrl(String),
    Message(String),
}

use std::net::TcpListener;
use std::thread;
// use std::sync::mpsc;

fn console_input_process() -> mpsc::Receiver<String> {
    let (to_process, from_console) = mpsc::channel();
    
    let console_input = thread::spawn(move || {
        loop {
            let mut text_message = String::new();

            print!("Type message: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut text_message)
                .expect("message input error");

            to_process.send(text_message)
                .expect("cannot send data from console to process");
        }
    });
    from_console
}

fn tcp_server_process(bind: &str) -> (mpsc::Sender<String>, mpsc::Receiver<String>) {

    let bind_addr: SocketAddr = bind.parse()
        .expect("wrong bind ip socket address provided");
    let listener = TcpListener::bind("127.0.0.1:8888")
        .expect("cannot bind to provided socket ip address");
    thread::spawn(move || {
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    let mut read_sock = socket.try_clone()
                        .expect("unable to clone ");
                    println!("new client: {:?}", addr);
                    // let (sock_inc, sock_out) = socket.split();
                    socket.write(b"Hello, world!!!\n")
                        .expect("cannot write to attached socket");
                    
                    let mut input = String::new();
                    let mut reader = io::BufReader::new(read_sock);
                    // read_sock.read_to_string(&mut input)
                    //     .expect("cannot get string from the socket");
                    for line in reader.lines() {
                        match line {
                            Ok(input) => println!("Received bytes: {}", &input),
                            Err(_) => {},
                        }
                    }
                    println!("Session processing completed!");
                }, 
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    });
    mpsc::channel()
}

fn main() {
    let mut msg_counter = 0;

    let from_console = console_input_process();
    let (to_peer, from_peer) = tcp_server_process("0.0.0.0:8888"); 
    // let (to_peer, from_peer) = tcp_client_process("127.0.0.1:8888".parse().unwrap()); 
    
    loop {
        let received = String::from("some message received");
        // 1let mut text_message = String::new();
        // print!("Type message: ");
        // io::stdout().flush();
        // io::stdin().read_line(&mut text_message)
        //     .expect("message input error");

        match from_console.try_recv() {
            Ok(message) => {
                if let Some(0) = message.find("/exit") {
                    println!("Bingo!!! It goes out...");
                    break
                }
                
                if let Some(0) = message.find("/count") {
                    println!("Counter: {}", msg_counter);
                    continue
                }
                
                // if let Some(0) = text_message.find("/get") {
                if msg_counter % 3 == 0 {
                    println!("Message received: {}", received);
                }

                println!("Message typed: {}", message);
                
                msg_counter += 1;
            }
            Err(_) => {},
        }

    }    
}


// fn run_app() {
//         let result = match config::parse() {
//         Ok(params) => {
//             let ioc = match params.mode {
//                 ExecMode::Server(bind) => {
//                     println!("Start server!");
//                     start_server(&bind).expect(&format!("cannot bind to local ip {}", &bind))
//                 },
//                 ExecMode::Client(conn) => {
//                     println!("Start client!");
//                     start_client(&conn).expect(&format!("cannot connect to server {}", &conn))
//                 },
//             };
//         },
//         Err(conf_err) => {
//             println!("Configuration fault: {}", conf_err.msg());
//             panic!("Panic because of ");
//         },
//     };
// }
