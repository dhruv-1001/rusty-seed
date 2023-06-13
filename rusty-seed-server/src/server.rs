use std::{sync::{Arc, Mutex}, net::{TcpListener, TcpStream}, io::{self, Read}, thread};

use rusty_seed_core::utils::MESSAGE_HEADER_SIZE;
use rusty_seed_database::Database;
use tracing::{info, warn, error};

pub fn run(port: u16, database: Arc<Mutex<Database>>) -> io::Result<()> {
    
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;   

    info!("Listening to requests on port: {}", port);

    for stream in listener.incoming() {
        let server_database = Arc::clone(&database);
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream, server_database);
                });
            },
            Err(e) => {
                warn!("Failed to establish connection: {}", e);
            },
        }
    }

    Ok(())
}

#[allow(unused_variables)]
fn handle_connection(mut stream: TcpStream, database: Arc<Mutex<Database>>) {
    let mut message_header_buffer = [0u8; MESSAGE_HEADER_SIZE];
    loop {
        match stream.read(&mut message_header_buffer) {
            Ok(size) => {
                if size == 0 {
                    error!("Client disconnected: {}", stream.peer_addr().unwrap());
                    break;
                }
            },
            Err(e) => {
                error!("Error reading from client {}: {}", stream.peer_addr().unwrap(), e);
                return;
            },
        }
    }
}