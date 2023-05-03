use std::thread;

// use rusty_seed_core::message::Message;

pub mod local;
pub mod server;

fn main() {
    // TODO: start listening on a port for requests from cli & client
    // let (local_sender, local_receiver) = mpsc::channel::<Message>();

    let local_handle = thread::spawn(|| {
        local::run();
    });

    // TODO: load database(s)
    // TODO: send request to client to continue uncomplete downloads
    // TODO: verify downloads
    // TODO: start listening to incoming connections connections and start seeding active paths
    let server_handle = thread::spawn(|| {
        server::run();
    });

    local_handle.join().unwrap();
    server_handle.join().unwrap();
}
