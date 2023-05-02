use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use rusty_seed_core::message::Message;

pub mod file;

fn main() {
    // TODO: start listening on a port for requests from cli & client
    let (local_sender, local_receiver) = mpsc::channel::<Message>();
    let local_receiver = Arc::new(Mutex::new(local_receiver));

    let handle = thread::spawn(|| {});

    // TODO: load database(s)
    // TODO: send request to client to continue uncomplete downloads
    // TODO: verify downloads
    // TODO: start listening to incoming connections connections and start seeding active paths
}
