use bincode::{deserialize, serialize};
use rusty_seed_core::file::hash::FileHash;
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

    pub fn add_seed_file(&self, hash: FileHash, path: PathBuf) -> Result<(), DatabaseError> {
        let seed_file = SeedFileInfo {
            path: path,
            active: true,
        };
        let serialized_hash = serialize(&hash).unwrap();
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
                let hash: FileHash = deserialize(&key).unwrap();
                let seed: SeedFileInfo = deserialize(&value).unwrap();
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

    use super::SeedDatabase;

    #[test]
    fn test_add_and_get_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(".test-db");
        let seed_db = SeedDatabase::open(path.clone());

        let file_hash_one = FileHash::from_string("1234".to_owned());
        let file_path_one = PathBuf::from("path/for/test/one");

        let file_hash_two = FileHash::from_string("5678".to_owned());
        let file_path_two = PathBuf::from("path/for/test/two");

        seed_db
            .add_seed_file(file_hash_one.clone(), file_path_one)
            .unwrap();
        seed_db
            .add_seed_file(file_hash_two.clone(), file_path_two)
            .unwrap();

        let file_one = seed_db.get_seed_file(file_hash_one).unwrap();
        let file_two = seed_db.get_seed_file(file_hash_two).unwrap();

        assert_eq!("path/for/test/one", file_one.path.to_str().unwrap());
        assert_eq!("path/for/test/two", file_two.path.to_str().unwrap());

        let seed_files = seed_db.get_all_seed_file();
        for file in seed_files {
            println!("{:?} {:?}", file.0, file.1);
        }

        std::fs::remove_dir_all(path).unwrap();
    }
}
