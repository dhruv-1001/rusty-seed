use std::{
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    time::Duration,
};

use bincode::{deserialize, serialize};
use rusty_seed_core::api::message::{LocalRequest, LocalResponse};

pub struct CliClient {
    stream: TcpStream,
}

// TODO: These CLI methods can be made in to a trait, and then we can change the CliClient if we need to, without changing much code

impl CliClient {
    pub fn from(port: u16) -> Self {
        let address = format!("127.0.0.1:{}", port);
        let stream = TcpStream::connect(address).unwrap();
        Self { stream }
    }

    pub fn add_seed(&mut self, path: PathBuf) -> LocalResponse {
        let request = LocalRequest::AddSeed { path };
        let serialized_request = serialize(&request).unwrap();
        self.expect_response(serialized_request)
    }

    pub fn remove_seed(&mut self, path: PathBuf) -> LocalResponse {
        let request = LocalRequest::RemoveSeed { path };
        let serialized_request = serialize(&request).unwrap();
        self.expect_response(serialized_request)
    }

    pub fn list_seeds(&mut self) -> LocalResponse {
        let request = LocalRequest::ListSeeds;
        let serialized_request = serialize(&request).unwrap();
        self.expect_response(serialized_request)
    }

    fn expect_response(&mut self, request: Vec<u8>) -> LocalResponse {
        self.stream.write(&request[..]).unwrap();

        let mut response = [0u8; 1024];
        self.stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let bytes_read = self.stream.read(&mut response).unwrap();

        deserialize(&response[..bytes_read]).unwrap()
    }
}
