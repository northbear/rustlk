// mod config {
    use std::env;
    use std::net::SocketAddr;

    pub enum ExecMode {
        Server(SocketAddr),
        Client(SocketAddr),
    }

    pub struct Params {
        pub program: String,
        pub mode: ExecMode,
    }

    pub struct ConfError {
        msg: String
    }

    impl ConfError {
        pub fn msg(&self) -> &String {
            &self.msg
        }
    }

    pub fn parse() -> Result<Params, ConfError> {
        let opts: Vec<String> = env::args().collect();
        // let mut params = Params{};
        let sock_address = SocketAddr::from(([0, 0, 0, 0], 8888));

        // params.mode = ExecMode::Client(sock_address);

        Ok(Params{
            program: opts[0].clone(),
            mode: ExecMode::Client(sock_address),
        })
        // Err(ConfError{ msg: String::from("Config Error") })
    }
// }
