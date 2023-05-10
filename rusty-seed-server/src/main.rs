use clap::Parser;
use rusty_seed_core::utils::default_database_path;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use validator::DBValidator;

use rusty_seed_database::SeedDatabase;

mod local;
mod server;
mod validator;

#[derive(Parser, Debug)]
#[command(version)]
struct CliOpts {
    /// Path to database [default: $HOME/.rustyseed]
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let opts = CliOpts::parse();
    let path = match opts.path {
        Some(path) => path,
        None => default_database_path(),
    };
    // TODO: load database(s)
    let seed_database = Arc::new(Mutex::new(SeedDatabase::open(path.clone())));
    // TODO: verify downloads
    DBValidator::validate(seed_database, path);

    // TODO: send request to client to continue uncomplete downloads
    // TODO: start listening to incoming connections connections and start seeding active paths
    let server_handle = thread::spawn(|| {
        server::run();
    });

    // TODO: start listening on a port for requests from cli & client
    let local_handle = thread::spawn(|| {
        local::run();
    });

    local_handle.join().unwrap();
    server_handle.join().unwrap();
}
