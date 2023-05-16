use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use bincode::{deserialize, serialize};
use rusty_seed_core::api::message::{LocalRequest, LocalResponse};

pub struct CliClient {
    stream: TcpStream,
}

impl CliClient {
    pub fn from(port: u16) -> Self {
        let address = format!("127.0.0.1:{}", port);
        let stream = TcpStream::connect(address).unwrap();
        Self { stream }
    }

    pub fn list_seeding_paths(&mut self) -> LocalResponse {
        let request = serialize(&LocalRequest::GetSeedFiles).unwrap();
        self.stream.write(&request[..]).unwrap();

        let mut response = [0u8; 1024];
        self.stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let bytes_read = self.stream.read(&mut response).unwrap();

        deserialize(&response[..bytes_read]).unwrap()
    }
}
