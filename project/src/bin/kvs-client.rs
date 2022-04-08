use clap::{Parser};
use kvs::{KVSClient, CLICommands};

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
    command: CLICommands,
}


fn main() {
    let cli = Cli::parse();

    let mut client = KVSClient::new(cli.addr)
        .expect("cant create server");
    let cmd_result = client.send_cmd(cli.command).expect("IO error");
    println!("{}", cmd_result);
}
