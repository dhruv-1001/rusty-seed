use clap::{Parser, Subcommand};
use std::path::PathBuf;
use utils::{generate_test_dir, generate_test_file};

mod utils;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliOpts {
    /// Local port to send CLI commands to
    #[arg(short, long, default_value = "10000")]
    pub port: usize,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Add file/dir to seed
    AddPath {
        /// Path to file/fir
        #[clap(long)]
        path: PathBuf,
    },

    /// Remove file/dir from seeding
    RemovePath {
        /// Path to file/dir
        #[clap(long)]
        path: PathBuf,
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

    /// Generates a file for testing
    GenerateTestFile {
        /// File name to be generated
        #[clap(long, default_value = "test-file")]
        name: String,

        /// Path to generate file to
        #[clap(long, default_value = "~/.rusty-seed")]
        path: PathBuf,

        /// File size in bytes
        #[clap(long)]
        size: usize,
    },

    /// Generates a directory for testing
    GenerateTestDir {
        /// Directory name to be generated
        #[clap(long, default_value = "test-dir")]
        name: String,

        /// Path to generate directory to
        #[clap(long, default_value = "~/.rusty-seed")]
        path: PathBuf,

        /// Number of files to be generated
        #[clap(long)]
        num_files: usize,

        /// Directory size in bytes
        #[clap(long)]
        size: usize,
    },
}

pub struct RustySeedCli;

impl RustySeedCli {
    pub fn new() -> Self {
        RustySeedCli
    }

    pub fn run(&self) {
        let opts = CliOpts::parse();

        handle_subcommand(opts);
    }
}

fn handle_subcommand(opts: CliOpts) {
    // TODO: create client that will send requests to listener on server and client

    match opts.command {
        Command::AddPath { path } => {
            println!("{}", path.as_path().to_str().unwrap().to_string())
        }
        Command::RemovePath { path } => {
            println!("{}", path.as_path().to_str().unwrap().to_string())
        }
        Command::ListSeedingPaths => todo!(),
        Command::ListAllPaths => todo!(),
        Command::Download { link } => {
            println!("{}", link)
        }
        Command::Stop => todo!(),
        Command::GenerateTestFile { name, path, size } => {
            match generate_test_file(name, path, size) {
                Ok(()) => {
                    println!("Success!")
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
        } => generate_test_dir(name, path, num_files, size),
    }
}