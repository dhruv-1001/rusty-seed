use std::path::PathBuf;

use bincode::{deserialize, serialize};
use rusty_seed_core::file::{hash::FileHash, metadata::FileMetadata};

use crate::error::DatabaseError;

pub struct SeedFileDatabase {
    metadatadb: sled::Db,
}

impl SeedFileDatabase {
    pub fn open(file_hash: FileHash, mut path: PathBuf) -> Self {
        path.push(file_hash.hash);
        path.push("metadata");
        let metadatadb = sled::open(path).unwrap();
        Self { metadatadb }
    }

    pub fn save_metadata(&self, file_metadata: FileMetadata) -> Result<(), DatabaseError> {
        let serialized_hash = serialize(&file_metadata.file_hash.hash).unwrap();
        let serialized_file_metadata = serialize(&file_metadata).unwrap();
        match self
            .metadatadb
            .insert(serialized_hash, serialized_file_metadata)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(DatabaseError::CustomError {
                    error: e.to_string(),
                })
            }
        }
    }

    pub fn get_metadata(&self, file_hash: FileHash) -> Result<FileMetadata, DatabaseError> {
        let serialized_hash = serialize(&file_hash.hash).unwrap();
        let metadata = match self.metadatadb.get(serialized_hash) {
            Ok(metadata) => metadata,
            Err(e) => {
                return Err(DatabaseError::CustomError {
                    error: e.to_string(),
                })
            }
        };
        match metadata {
            Some(metadata) => {
                let file_metadata: FileMetadata = deserialize(&metadata).unwrap();
                Ok(file_metadata)
            }
            None => Err(DatabaseError::NoRecordFound),
        }
    }
}
