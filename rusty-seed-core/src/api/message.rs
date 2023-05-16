use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::file::{hash::FileHash, metadata::FileMetadata};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientRequest {
    Connect,

    GetFileMetadata { hash: FileHash },

    GetPeers { hash: FileHash },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerResponse {
    ConnectAck,

    FileMetadata {
        hash: FileHash,
        metadata: Option<FileMetadata>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LocalRequest {
    AddSeed { path: PathBuf },
    RemoveSeed { path: PathBuf },
    ListSeeds,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LocalResponse {
    AddSeed { test: String },
    RemoveSeed { test: String },
    SeedFiles { test: String },
}
