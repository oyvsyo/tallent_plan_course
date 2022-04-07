use clap::{Parser, Subcommand};
use kvs::{KvStore, KvsEngine};
use std::path::Path;

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Client for Key-Value Storage, String:String"
)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1:4000")]
    addr: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up value by key into KVS
    Set { key: String, value: String },
    /// Get value by key
    Get { key: String },
    /// Removes value by key
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(".");
    let mut kvs_obj = KvStore::open(path).expect("Cant create store");

    match &cli.command {
        Commands::Set { key, value } => {
            kvs_obj
                .set(key.to_owned(), value.to_owned())
                .expect("Cant set value");
        }
        Commands::Get { key } => match kvs_obj.get(key.to_string()) {
            Ok(maybe_index) => match maybe_index {
                Some(value) => {
                    println!("{}", value);
                }
                None => {
                    println!("Key not found");
                }
            },
            Err(_) => panic!("bruh"),
        },
        Commands::Rm { key } => {
            match kvs_obj.remove(key.to_string()) {
                Err(_) => {
                    print!("Key not found");
                    panic!()
                }
                _ => (),
            };
        }
    }
}
