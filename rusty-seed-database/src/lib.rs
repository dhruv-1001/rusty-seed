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
    database_path: PathBuf,
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
            database_path: path,
        }
    }

    pub fn add_seed_file(&mut self, file_metadata: FileMetadata) -> Result<(), DatabaseError> {
        self.seed_db.add_seed_file(file_metadata.clone())?;
        let seed_file_db =
            SeedFileDatabase::open(file_metadata.file_hash.clone(), self.database_path.clone());
        seed_file_db.save_metadata(file_metadata.clone())?;
        self.seed_file_db
            .insert(file_metadata.file_hash.hash, seed_file_db);
        Ok(())
    }

    pub fn remove_seed_file(&mut self, file_hash: FileHash) {
        self.seed_db.remove_seed_file(file_hash.clone()).unwrap();
        self.seed_file_db.remove(&file_hash.hash);
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
            None => Err(DatabaseError::NoRecordFound),
        }
    }

    pub fn get_metadata(&self, file_hash: FileHash) -> Result<FileMetadata, DatabaseError> {
        match self.seed_file_db.get(&file_hash.hash) {
            Some(seed_file_db) => seed_file_db.get_metadata(file_hash),
            None => Err(DatabaseError::NoRecordFound),
        }
    }
}

#[cfg(test)]
mod test {
    use rusty_seed_core::{file::metadata::FileMetadata, utils::default_database_path};

    use crate::Database;

    #[test]
    fn save_metadata() {
        let database_path = default_database_path();
        let mut database = Database::open(database_path.clone());

        let mut path_dir1 = database_path.clone();
        path_dir1.push("dir1");
        let mut path_dir2 = database_path.clone();
        path_dir2.push("dir2");
        let mut path_dir3 = database_path.clone();
        path_dir3.push("dir3");
        let mut path_dir4 = database_path.clone();
        path_dir4.push("dir4");
        let mut path_dir5 = database_path.clone();
        path_dir5.push("dir5");
        let mut path_dir6 = database_path.clone();
        path_dir6.push("dir6");

        println!("1");
        let metadata_dir1 = FileMetadata::from(path_dir1, None).unwrap();
        database.add_seed_file(metadata_dir1).unwrap();
        println!("2");
        let metadata_dir2 = FileMetadata::from(path_dir2, None).unwrap();
        database.add_seed_file(metadata_dir2).unwrap();
        println!("3");
        let metadata_dir3 = FileMetadata::from(path_dir3, None).unwrap();
        database.add_seed_file(metadata_dir3).unwrap();
        println!("4");
        let metadata_dir4 = FileMetadata::from(path_dir4, None).unwrap();
        database.add_seed_file(metadata_dir4).unwrap();
        println!("5");
        let metadata_dir5 = FileMetadata::from(path_dir5, None).unwrap();
        database.add_seed_file(metadata_dir5).unwrap();
        println!("6");
        let metadata_dir6 = FileMetadata::from(path_dir6, None).unwrap();
        database.add_seed_file(metadata_dir6).unwrap();
    }

    #[test]
    fn read_metadata() {
        let database_path = default_database_path();
        let database = Database::open(database_path.clone());

        for (file_hash, _) in database.get_all_seed_files() {
            println!("{:#?}", database.get_metadata(file_hash).unwrap());
        }
    }
}
