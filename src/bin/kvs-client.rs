use clap::Parser;
use kvs::{DBCommands, KVSClient, ServerResponse};

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Client for Key-Value storage, String:String"
)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1:4000")]
    addr: String,
    #[clap(subcommand)]
    command: DBCommands,
}

fn main() {
    let cli = Cli::parse();

    let mut client = KVSClient::new(cli.addr).expect("cant create server");
    let resp = client.send_cmd(cli.command).expect("IO error");
    match resp {
        ServerResponse::Success { output } => {
            if !output.is_empty() {
                println!("{}", output);
            }
        }
        ServerResponse::Failure { message } => {
            panic!("{}", message);
        }
    }
}
