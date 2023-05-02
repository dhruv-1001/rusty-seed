pub mod error;
pub mod sleddb;
use std::path::Path;

use sleddb::SledDatabase;

pub struct Database {
    sled: SledDatabase,
}

#[allow(unused_variables)]
impl Database {
    // Open and return existing database, if not present, it create and return a new one
    pub fn open(path: &Path) -> Self {
        Database {
            sled: SledDatabase::open(path),
        }
    }

    pub fn add_file(&mut self, hash: String, path: &Path) {
        self.sled.add_file(hash, path);
    }
}
