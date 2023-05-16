use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use bincode::{deserialize, serialize};
use rusty_seed_core::api::message::{LocalRequest, LocalResponse};
use rusty_seed_database::Database;
use tracing::{info, warn};

pub fn run(port: u16, database: Arc<Mutex<Database>>) -> io::Result<()> {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;

    info!("Listening to local requests on port: {}", port);

    for stream in listener.incoming() {
        let local_database = Arc::clone(&database);
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream, local_database);
                });
            }
            Err(e) => {
                warn!("Failed to establish connection: {}", e);
            }
        }
    }

    Ok(())
}

#[allow(unused_variables)]
fn handle_connection(mut stream: TcpStream, database: Arc<Mutex<Database>>) {
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer).unwrap();

    let request: LocalRequest = deserialize(&buffer).unwrap();
    info!(
        "Incoming request {:?} from {:?}",
        request,
        stream.peer_addr().unwrap()
    );
    let response = match request {
        LocalRequest::GetSeedFiles => {
            let response = LocalResponse::SeedFiles {
                test: "Hello".to_owned(),
            };
            serialize(&response).unwrap()
        }
    };

    stream.write(&response[..]).unwrap();
}
