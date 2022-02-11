use clap::{AppSettings, Parser, Subcommand};

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

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Commands::Set { key, value } => {
            panic!("unimplemented Set {} {}", key, value)
        }
        Commands::Get { key } => {
            panic!("unimplemented Get {}", key)
        }
        Commands::Rm { key } => {
            panic!("unimplemented Rm {}", key)
        }
    }
}
