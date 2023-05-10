use bincode::{deserialize, serialize};
use rusty_seed_core::file::hash::FileHash;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::DatabaseError;

// TODO: remove active field from here, we will keep that inside metadata, it's not good that we will have to update it in 2 different locations. I should just be present in one place
#[derive(Debug, Serialize, Deserialize)]
pub struct SeedFile {
    path: PathBuf,
    active: bool,
}

pub struct SledDatabase {
    db: sled::Db,
}

impl SledDatabase {
    pub fn open(mut path: PathBuf) -> Self {
        path.push("seed-files-database");
        let db = sled::open(path).unwrap();
        Self { db }
    }

    pub fn add_seed_file(&mut self, hash: FileHash, path: PathBuf) {
        let seed_file = SeedFile {
            path: path,
            active: true,
        };
        let serialized_seed = serialize(&seed_file).unwrap();
        let serialized_hash = serialize(&hash).unwrap();
        self.db.insert(serialized_hash, serialized_seed).unwrap();
    }

    pub fn get_seed_file(&self, hash: FileHash) -> SeedFile {
        let hash = serialize(&hash).unwrap();
        let seed_file: SeedFile = deserialize(&self.db.get(hash).unwrap().unwrap()).unwrap();
        seed_file
    }

    pub fn remove_seed_file(&self, hash: FileHash) -> Result<(), DatabaseError> {
        let hash = serialize(&hash).unwrap();
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

    pub fn get_all_seed_file(&self) -> Vec<(FileHash, SeedFile)> {
        let mut seed_files: Vec<(FileHash, SeedFile)> = Vec::new();
        self.db.iter().for_each(|item| {
            if let Ok((key, value)) = item {
                let hash: FileHash = deserialize(&key).unwrap();
                let seed: SeedFile = deserialize(&value).unwrap();
                seed_files.push((hash, seed))
            }
        });
        seed_files
    }

    #[allow(unused_variables)]
    pub fn mark_inactive(&mut self, hash: FileHash) {}
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use rusty_seed_core::file::hash::FileHash;

    use super::SledDatabase;

    #[test]
    fn test_add_and_get_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(".test-db");
        let mut seed_db = SledDatabase::open(path.clone());

        let file_hash_one = FileHash::from_string("1234".to_owned());
        let file_path_one = PathBuf::from("path/for/test/one");

        let file_hash_two = FileHash::from_string("5678".to_owned());
        let file_path_two = PathBuf::from("path/for/test/two");

        seed_db.add_seed_file(file_hash_one.clone(), file_path_one);
        seed_db.add_seed_file(file_hash_two.clone(), file_path_two);

        let file_one = seed_db.get_seed_file(file_hash_one);
        let file_two = seed_db.get_seed_file(file_hash_two);

        assert_eq!("path/for/test/one", file_one.path.to_str().unwrap());
        assert_eq!("path/for/test/two", file_two.path.to_str().unwrap());

        let seed_files = seed_db.get_all_seed_file();
        for file in seed_files {
            println!("{:?} {:?}", file.0, file.1);
        }

        std::fs::remove_dir_all(path).unwrap();
    }
}
