
// extern crate tokio;


use std::io;
use std::io::{Write, BufRead};
use std::sync::mpsc;
// use std::net::{SocketAddr};
// use std::net::{TcpListener,TcpStream};

mod config;
    
enum Event {
    Ctrl(String),
    Message(String),
}

use std::net::TcpListener;
use std::thread;
// use std::sync::mpsc;

fn console_input_process() -> mpsc::Receiver<String> {
    let (to_process, from_console) = mpsc::channel();
    
    thread::spawn(move || {
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

fn tcp_server_process(bind_ip: &str) -> (mpsc::Sender<String>, mpsc::Receiver<String>) {

    let (to_process, from_sock) = mpsc::channel();
    let (to_sock, from_process) = mpsc::channel();
    
    // let bind_addr: SocketAddr = bind.parse()
    //     .expect("wrong bind ip socket address provided");
    let listener = TcpListener::bind(bind_ip)
        .expect("cannot bind to provided socket ip address");
    
    thread::spawn(move || {
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) => {
                    let mut read_sock = socket.try_clone()
                        .expect("unable to clone ");

                    println!("new client: {:?}", addr);

                    socket.write(b"Welcome to chat server!!!\n")
                        .expect("cannot write to attached socket");
                    
                    thread::spawn(move || {
                        let fp = from_process;
                        loop {
                            match fp.try_recv() {
                                Ok(message) => {
                                    println!("inside socket thread: {}", message)
                                    // socket.write(&message.into_bytes())
                                    //    .expect("cannot write message to the sock");
                                },
                                Err(e) => println!("error receiving from channel: {}", e),
                            }
                        }
                    });
                    
                    // let mut input = String::new();
                    let mut reader = io::BufReader::new(read_sock);

                    for line in reader.lines() {
                        match line {
                            Ok(input) => to_process.send(input)
                                .expect("cannot send msg to proccess"),
                            Err(_) => {},
                        }
                    }
                    println!("Session processing completed!");
                }, 
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    });
    (to_sock, from_sock)
}

fn main() {

    let from_console = console_input_process();
    let (to_peer, from_peer) = tcp_server_process("0.0.0.0:8888"); 
    // let (to_peer, from_peer) = tcp_client_process("127.0.0.1:8888".parse().unwrap()); 
    
    loop {

        match from_peer.try_recv() {
            Ok(received) => {
                println!("peer: {}", &received);
            },
            Err(_e) => {}, // panic!(format!("error on receiving from socket: {}", e)),
        }
        
        match from_console.try_recv() {
            Ok(message) => {
                if let Some(0) = message.find("/exit") {
                    println!("Bingo!!! It goes out...");
                    break
                }
                
                to_peer.send(message)
                    .expect("cannot send message to the peer");

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
