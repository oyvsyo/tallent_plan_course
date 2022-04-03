use std::path::Path;
use clap::{AppSettings, Parser, Subcommand};
use kvs::{KvStore};
use std::error::Error;

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Key-Value memory storage, String:String"
)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
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

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    let mut kvs_obj = KvStore::open(path).expect("Cant create store");

    match &cli.command {
        Commands::Set { key, value } => {
            kvs_obj.set(key.to_owned(), value.to_owned());
        }
        Commands::Get { key } => {
            match kvs_obj.get(key.to_string()) {
                Ok(maybe_index) => match maybe_index {
                    Some(value) => {
                        println!("{}",value);
                    }
                    None => {
                        println!("Key not found");
                    }
                }
                Err(_) => panic!("bruh")
            }
        }
        Commands::Rm { key } => {
            match kvs_obj.remove(key.to_string()) {
                Err(E) => {
                    print!("Key not found");
                    panic!()
                },
                _ => ()
            };
        }
        _ => panic!("Invalid command. Use one of: [set, get, rm, help]")
    }

}