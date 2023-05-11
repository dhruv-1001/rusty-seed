use bincode::{deserialize, serialize};
use rusty_seed_core::file::{hash::FileHash, metadata::FileMetadata};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::DatabaseError;

// TODO: remove active field from here, we will keep that inside metadata, it's not good that we will have to update it in 2 different locations. I should just be present in one place
#[derive(Debug, Serialize, Deserialize)]
pub struct SeedFileInfo {
    pub path: PathBuf,
    pub active: bool,
}

pub struct SeedDatabase {
    db: sled::Db,
}

impl SeedDatabase {
    pub fn open(mut path: PathBuf) -> Self {
        path.push("seed-files-database");
        let db = sled::open(path).unwrap();
        Self { db }
    }

    pub fn add_seed_file(&self, file_metadata: FileMetadata) -> Result<(), DatabaseError> {
        let seed_file = SeedFileInfo {
            path: file_metadata.file_path,
            active: true,
        };
        let serialized_hash = serialize(&file_metadata.file_hash.hash).unwrap();
        let serialized_seed_file = serialize(&seed_file).unwrap();
        match self.db.insert(serialized_hash, serialized_seed_file) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(DatabaseError::CustomError {
                    error: e.to_string(),
                })
            }
        }
    }

    pub fn check_if_seeding(&self, file_hash: FileHash) -> Result<bool, DatabaseError> {
        match self.get_seed_file(file_hash) {
            Ok(seed_file_info) => Ok(seed_file_info.active),
            Err(e) => return Err(e),
        }
    }

    fn get_seed_file(&self, file_hash: FileHash) -> Result<SeedFileInfo, DatabaseError> {
        let hash: Vec<u8> = serialize(&file_hash).unwrap();
        let seed_file = match self.db.get(hash) {
            Ok(seed_file) => seed_file,
            Err(e) => {
                return Err(DatabaseError::CustomError {
                    error: e.to_string(),
                })
            }
        };
        match seed_file {
            Some(seed_file) => {
                let seed_file = deserialize(&seed_file).unwrap();
                Ok(seed_file)
            }
            None => Err(DatabaseError::NoRecordFound),
        }
    }

    pub fn remove_seed_file(&self, file_hash: FileHash) -> Result<(), DatabaseError> {
        let hash = serialize(&file_hash).unwrap();
        let found = match self.db.contains_key(hash.clone()) {
            Ok(found) => found,
            Err(e) => {
                return Err(DatabaseError::CustomError {
                    error: e.to_string(),
                })
            }
        };
        if !found {
            return Err(DatabaseError::SeedFileNotFound);
        }
        match self.db.remove(hash) {
            Ok(_) => {}
            Err(e) => {
                return Err(DatabaseError::CustomError {
                    error: e.to_string(),
                })
            }
        };
        Ok(())
    }

    pub fn get_all_seed_file(&self) -> Vec<(FileHash, SeedFileInfo)> {
        let mut seed_files: Vec<(FileHash, SeedFileInfo)> = Vec::new();
        self.db.iter().for_each(|item| {
            if let Ok((key, value)) = item {
                let hash: String = deserialize(&key).unwrap();
                let file_hash = FileHash::from_string(hash);
                let seed: SeedFileInfo = deserialize(&value).unwrap();
                seed_files.push((file_hash, seed))
            }
        });
        seed_files
    }

    #[allow(unused_variables)]
    pub fn mark_inactive(&mut self, hash: FileHash) {}
}
