use clap::{Parser, Subcommand};
use cliclient::CliClient;
use rusty_seed_core as core;
use rusty_seed_core::api::message::LocalResponse;
use rusty_seed_core::utils::default_download_path;
use std::{path::PathBuf, time::SystemTime};
use utils::{generate_test_dir, generate_test_file};

mod cliclient;
mod utils;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliOpts {
    /// Local port to send CLI commands to
    #[arg(short, long, default_value = "10001")]
    pub port: u16,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Add file/dir to seed
    AddSeed {
        /// Path to file/fir
        #[clap(long)]
        path: PathBuf,
    },

    /// Remove file/dir from seeding
    RemoveSeed {
        /// Path to file/dir
        #[clap(long)]
        path: PathBuf,
    },

    /// List all seeding files/dir
    ListSeeds,

    /// Download file/dir from avaiable peers
    Download {
        /// Link to that file
        #[clap(long)]
        link: String,

        /// Path to download seed to [default: $HOME/Downloads]
        #[clap(long)]
        download_path: Option<PathBuf>,
    },

    /// Stops client and server
    Stop,

    /// Stops client
    StopClient,

    /// Stops server
    StopServer,

    /// Generates a file for testing
    GenerateTestFile {
        /// File name to be generated
        #[clap(long, default_value = "test-file")]
        name: String,

        /// Path to generate file to [default: $HOME/.rustyseed]
        #[clap(long)]
        path: Option<PathBuf>,

        /// File size in bytes
        #[clap(long, default_value = "2048")]
        size: usize,
    },

    /// Generates a directory for testing
    GenerateTestDir {
        /// Directory name to be generated
        #[clap(long, default_value = "test-dir")]
        name: String,

        /// Path to generate directory to [default: $HOME/.rustyseed]
        #[clap(long)]
        path: Option<PathBuf>,

        /// Number of files to be generated
        #[clap(long, default_value = "10")]
        num_files: usize,

        /// Directory size in bytes
        #[clap(long, default_value = "2048")]
        size: usize,
    },
}

#[derive(Default)]
pub struct RustySeedCli;

impl RustySeedCli {
    pub fn new() -> Self {
        RustySeedCli::default()
    }

    pub fn run(&self) {
        let opts = CliOpts::parse();

        handle_subcommand(opts);
    }
}

fn handle_subcommand(opts: CliOpts) {
    // TODO: create API to send requests to listener on server and client
    let mut cli_client = CliClient::from(opts.port);
    match opts.command {
        Command::AddSeed { path } => {
            let response: LocalResponse = cli_client.add_seed(path);
            println!("\n{:#?}", response);
        }
        Command::RemoveSeed { path } => {
            let response: LocalResponse = cli_client.remove_seed(path);
            println!("\n{:#?}", response);
        }
        Command::ListSeeds => {
            let response: LocalResponse = cli_client.list_seeds();
            if let LocalResponse::SeedFiles { seed_files } = response {
                println!("\n All Seeds:\n");
                for (file_hash, path) in seed_files {
                    println!("path: {:?}\thash: {}", path, file_hash);
                }
            } else {
                unreachable!()
            }
        }
        Command::Download {
            link,
            download_path,
        } => {
            let download_path = match download_path {
                Some(download_path) => download_path,
                None => default_download_path(),
            };
            println!("{}: {:?}", link, download_path);
        }
        Command::Stop => todo!(),
        Command::StopClient => todo!(),
        Command::StopServer => todo!(),
        Command::GenerateTestFile { name, path, size } => {
            let path = match path {
                Some(path) => path,
                None => {
                    println!("No path provided, using default path: $HOME/.rustyseed",);
                    core::utils::default_database_path()
                }
            };
            let time = SystemTime::now();
            match generate_test_file(name, path, size) {
                Ok(()) => {
                    let time = SystemTime::now().duration_since(time).unwrap();
                    println!("Success in {} s", time.as_secs_f32());
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }
        Command::GenerateTestDir {
            name,
            path,
            num_files,
            size,
        } => {
            let path = match path {
                Some(path) => path,
                None => {
                    println!("No path provided, using default path: $HOME/.rustyseed",);
                    core::utils::default_database_path()
                }
            };
            let time = SystemTime::now();
            match generate_test_dir(name, path, num_files, size) {
                Ok(()) => {
                    let time = SystemTime::now().duration_since(time).unwrap();
                    println!("Success in {} s", time.as_secs_f32());
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }
    }
}
