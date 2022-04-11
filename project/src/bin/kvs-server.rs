use clap::Parser;
use env_logger::{Env, Target};
use kvs::{KvStore, KvsServer, SledStore};
use std::io::{Read, Write};
use std::path::Path;

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
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        // .target(Target::Stdout)
        .init();

    let cli = Cli::parse();

    // Check engine in dir
    check_engine(&cli.engine);
    let path = Path::new(".");
    match cli.engine.as_str() {
        "kvs" => {
            let storage = KvStore::open(path).expect("Cant create kvs store");
            let mut server = KvsServer::new(cli.addr, storage).expect("cant create server");
            server.listen();
        }
        "sled" => {
            let storage = SledStore::open(path).expect("Cant create sled store");
            let mut server = KvsServer::new(cli.addr, storage).expect("cant create server");
            server.listen();
        }
        _ => panic!("Only kvs engine is an option"),
    };
}

fn check_engine(engine: &str) {
    if Path::new(LOCK_FILE).exists() {
        let mut file = std::fs::File::open(LOCK_FILE).expect("Cant open");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Cant read from file");
        if contents != engine {
            panic!("Use previous engine: --engine={}", contents);
        }
    } else {
        let mut file = std::fs::File::create(LOCK_FILE).expect("cant create file");
        file.write_all(engine.as_bytes())
            .expect("Cant write to file");
        log::info!("Created lock file with engine -- {}", engine);
    }
}
