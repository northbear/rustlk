mod config;
use config::Mode;

fn main() {
    let conf = config::new();
    match conf.mode() {
        Ok(md) => match md {
            Mode::Server(bind) => println!("Server is binded to {}", bind),
            Mode::Client(ip)   => println!("Client should be connected to {}", ip),
            Mode::Usage        => println!("{}", conf.usage()),
        },
        Err(e) => {
            println!("{}", conf.usage());
            println!("Error: {}", e.0);
            return
        }
    }
}
