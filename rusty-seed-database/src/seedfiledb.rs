use std::path::PathBuf;

use rusty_seed_core::file::hash::FileHash;

pub struct SeedFileDatabase {
    db: sled::Db,
}

impl SeedFileDatabase {
    pub fn open(file_hash: FileHash, mut path: PathBuf) -> Self {
        path.push(file_hash.hash);
        path.push("metadata");
        let db = sled::open(path).unwrap();
        Self { db }
    }
}
