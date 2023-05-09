pub mod error;
pub mod sleddb;
use std::path::Path;

use rusty_seed_core::file::hash::FileHash;
use sleddb::SledDatabase;

pub struct SeedDatabase {
    sled: SledDatabase,
}

#[allow(unused_variables)]
impl SeedDatabase {
    // Open and return existing database, if not present, it create and return a new one
    pub fn open(path: &Path) -> Self {
        SeedDatabase {
            sled: SledDatabase::open(path),
        }
    }

    pub fn add_seed_file(&mut self, hash: FileHash, path: std::path::PathBuf) {
        self.sled.add_seed_file(hash, path);
    }
}
