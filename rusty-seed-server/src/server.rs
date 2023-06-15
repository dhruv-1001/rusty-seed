use std::{sync::{Arc, Mutex}, net::{TcpListener, TcpStream}, io::{self, Read, Write}, thread};

use bincode::{deserialize, serialize};
use rusty_seed_core::{utils::MESSAGE_HEADER_SIZE, api::message::{ClientRequest, ServerResponse}};
use rusty_seed_core::api::message::MessageHeader;
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
                let message_header: MessageHeader = deserialize(&message_header_buffer).unwrap();
                let MessageHeader(message_size) = message_header;
                // let mut message_buffer = [0u8; message_size];
                let mut message_buffer: Vec<u8> = Vec::with_capacity(message_size as usize);
                stream.read(&mut message_buffer).unwrap();
                let client_request: ClientRequest = deserialize(&message_buffer).unwrap();
                let server_response: ServerResponse = match client_request {
                    ClientRequest::Connect => todo!(),
                    ClientRequest::GetFileMetadata { hash } => {
                        ServerResponse::FileMetadata { hash: hash, metadata: None }
                    },
                    ClientRequest::GetPeers { hash } => todo!(),
                };
                let server_response_bytes = serialize(&server_response).unwrap();
                stream.write(&server_response_bytes).unwrap();
            },
            Err(e) => {
                error!("Error reading from client {}: {}", stream.peer_addr().unwrap(), e);
                return;
            },
        }
    }
}