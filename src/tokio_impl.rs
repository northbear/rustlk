use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::io::{copy,stdin,stdout};

pub fn start_server(sa: &SocketAddr) {
    let listener = TcpListener::bind(sa).unwrap();

    let server = listener.incoming()
        .map_err (|e| {})
        .for_each ( |sock| {
            let (incoming, outgoing) = sock.split();

            let copier = copy(incoming, stdout())
                .then(|r| { Ok(()) })
                .map_err( |e| {
                });

            tokio::spawn(copier);
            Ok(())
        });
    println!("{}", sa.to_string())
}

