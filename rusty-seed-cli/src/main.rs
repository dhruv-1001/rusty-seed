use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
struct CliOpts {
    /// Local port to send CLI commands to
    #[arg(short, long, default_value="10000")]
    port: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    /// Add file/dir to seed
    AddPath {
        /// Path to file/fir
        #[clap(long)]
        path: String,
    },

    /// Remove file/dir from seeding
    RemovePath {
        /// Path to file/dir
        #[clap(long)]
        path: String,
    },

    /// List all seeding files/dir
    ListSeedingPaths,
    
    /// List all files
    ListAllPaths,

    /// Download file/dir from avaiable peers
    Download {
        /// Link to that file
        #[clap(long)]
        link: String,
    },

    /// Stops client and server
    Stop,
}

fn main() {
    let args = CliOpts::parse();

    println!("{:?}", args);
}
