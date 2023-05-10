pub mod error;
pub mod seeddb;
pub mod seedfiledb;

use std::{collections::HashMap, path::PathBuf};

use error::DatabaseError;
use rusty_seed_core::file::{hash::FileHash, metadata::FileMetadata};
use seeddb::{SeedDatabase, SeedFileInfo};
use seedfiledb::SeedFileDatabase;

pub struct Database {
    seed_db: SeedDatabase,
    seed_file_db: HashMap<String, SeedFileDatabase>,
}

#[allow(unused_variables)]
impl Database {
    // Open and return existing database, if not present, it create and return a new one
    pub fn open(path: PathBuf) -> Self {
        let seed_db = SeedDatabase::open(path.clone());
        let mut seed_file_db: HashMap<String, SeedFileDatabase> = HashMap::new();
        for (file_hash, _) in seed_db.get_all_seed_file() {
            let db = SeedFileDatabase::open(file_hash.clone(), path.clone());
            seed_file_db.insert(file_hash.hash, db);
        }
        Self {
            seed_db,
            seed_file_db,
        }
    }

    pub fn add_seed_file(&self, file_hash: FileHash, path: PathBuf) -> Result<(), DatabaseError> {
        self.seed_db.add_seed_file(file_hash, path)
    }

    pub fn remove_seed_file(&self, file_hash: FileHash) {
        self.seed_db.remove_seed_file(file_hash.clone()).unwrap();
    }

    /// Get all seed files both active & inactive
    pub fn get_all_seed_files(&self) -> Vec<(FileHash, SeedFileInfo)> {
        self.seed_db.get_all_seed_file()
    }

    pub fn check_if_seeding(&self, file_hash: FileHash) -> Result<bool, DatabaseError> {
        self.seed_db.check_if_seeding(file_hash)
    }

    pub fn save_metadate(&self, file_metadata: FileMetadata) -> Result<(), DatabaseError> {
        match self.seed_file_db.get(&file_metadata.file_hash.hash) {
            Some(seed_file_db) => seed_file_db.save_metadata(file_metadata),
            None => return Err(DatabaseError::NoRecordFound),
        }
    }

    pub fn get_metadata(&self, file_hash: FileHash) -> Result<FileMetadata, DatabaseError> {
        match self.seed_file_db.get(&file_hash.hash) {
            Some(seed_file_db) => seed_file_db.get_metadata(file_hash),
            None => return Err(DatabaseError::NoRecordFound),
        }
    }
}
