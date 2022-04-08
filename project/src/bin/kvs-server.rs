use clap::{Parser};
use kvs::{KvStore, KvsServer};
use std::path::Path;

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Server for Key-Value storage, String:String"
)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1:4000")]
    addr: String,
    #[clap(short, long)]
    engine: String,
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(".");

    let storage = match cli.engine.as_str() {
        "kvs" => KvStore::open(path).expect("Cant create store"),
        _ => panic!("Only kvs engine is an option")
    };
    let addr = cli.addr;
    let mut server = KvsServer::new(addr, storage)
        .expect("cant create server");
    server.listen();
}
