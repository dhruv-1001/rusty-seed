pub mod error;
pub mod seeddb;
pub mod seedfiledb;

use std::path::PathBuf;

use rusty_seed_core::file::hash::FileHash;
use seeddb::{SeedFile, SledDatabase};

pub struct SeedDatabase {
    sled: SledDatabase,
}

#[allow(unused_variables)]
impl SeedDatabase {
    // Open and return existing database, if not present, it create and return a new one
    pub fn open(path: PathBuf) -> Self {
        SeedDatabase {
            sled: SledDatabase::open(path),
        }
    }

    pub fn add_seed_file(&mut self, hash: FileHash, path: PathBuf) {
        self.sled.add_seed_file(hash, path);
    }

    /// Get all seed files both active & inactive
    pub fn get_all_seed_files(&self) -> Vec<(FileHash, SeedFile)> {
        self.sled.get_all_seed_file()
    }
}
