use sock::Socks5Server;
use log4rs;
use log::{error, info, warn};
use clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    bind_addr: String,

    #[clap(short, long, default_value_t = 8080)]
    port: u16,
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let args = Args::parse();

    info!("Attempting to listen on: {bind_addr}:{port}", bind_addr=args.bind_addr.to_string(), port=args.port);
    let s = Socks5Server {
        bind_addr: format!("{bind_addr}:{port}", bind_addr=args.bind_addr.to_string(), port=args.port),
    };

    if let Err(err) = s.start() {
        error!("{:?}", &err);
    }
}
