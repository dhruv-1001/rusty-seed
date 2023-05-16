use clap::Parser;
use rusty_seed_core::{tracer::Tracer, utils::default_database_path};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use tracing::info;
use validator::DBValidator;

use rusty_seed_database::Database;

mod local;
mod server;
mod validator;

#[derive(Parser, Debug)]
#[command(version)]
struct CliOpts {
    /// Path to database [default: $HOME/.rustyseed]
    #[arg(short, long)]
    path: Option<PathBuf>,

    #[arg(long, default_value = "10001")]
    local_port: u16,

    /// Port to listen requests from the Internet
    #[arg(long, default_value = "10000")]
    port: u16,
}

fn main() {
    Tracer::init();
    info!("### Starting server");

    let opts = CliOpts::parse();
    let path = match opts.path {
        Some(path) => path,
        None => default_database_path(),
    };

    info!("### Opening database at path {:?}", path);
    let database = Arc::new(Mutex::new(Database::open(path)));

    info!("### Validating database");
    DBValidator::validate(&database);
    info!("### Validation finished");

    // TODO: send request to client to continue uncomplete downloads
    // [maybe client can send request to the server, to get information about what files do the client have to download]
    // TODO: start listening to incoming connections connections and start seeding active paths
    let server_handle = thread::spawn(|| {
        server::run();
    });

    // TODO: start listening on a port for requests from cli & client
    let local_database = Arc::clone(&database);
    let local_handle = thread::spawn(move || {
        local::run(opts.local_port, Arc::clone(&local_database)).unwrap();
    });

    local_handle.join().unwrap();
    server_handle.join().unwrap();
}
