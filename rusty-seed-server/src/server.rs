use std::{sync::{Arc, Mutex}, net::{TcpListener, TcpStream}, io, thread};

use rusty_seed_database::Database;
use tracing::{info, warn};

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

}