use clap::{Parser};
use kvs::{KvStore, KvsServer};
use std::path::Path;
use std::io::{Read, Write};

const LOCK_FILE: &str = "./.kvs.lock";

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Server for Key-Value storage, String:String"
)]
struct Cli {
    #[clap(short, long, default_value_t = String::from("127.0.0.1:4000"))]
    addr: String,
    #[clap(short, long)]
    engine: String,
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(".");

    // Check engine in dir
    check_engine(&cli.engine);

    let storage = match cli.engine.as_str() {
        "kvs" => KvStore::open(path).expect("Cant create store"),
        _ => panic!("Only kvs engine is an option")
    };
    let mut server = KvsServer::new(cli.addr, storage)
        .expect("cant create server");
    server.listen();
}

fn check_engine(engine: &str) {
    if Path::new(LOCK_FILE).exists() {
        let mut file = std::fs::File::open(LOCK_FILE)
            .expect("Cant open");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Cant read from file");
        assert_eq!(contents, engine);
    } else {
        let mut file = std::fs::File::create(LOCK_FILE)
            .expect("cant create file");
        file.write_all(engine.as_bytes())
            .expect("Cant write to file");
    }
}