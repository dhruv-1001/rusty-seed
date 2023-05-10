use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use rusty_seed_core::file::hash::FileHash;
use rusty_seed_database::{seeddb::SeedFile, Database};

pub struct DBValidator;

impl DBValidator {
    pub fn validate(seed_database: Arc<Mutex<Database>>, database_path: PathBuf) {
        let seed_files = seed_database.lock().unwrap().get_all_seed_files();
        let mut validating_threads = Vec::new();
        for (hash, seed_file) in seed_files {
            let seed_database_clone = Arc::clone(&seed_database);
            let path = database_path.clone();
            let thread = thread::spawn(move || {
                validate_seed_file(hash, seed_file, seed_database_clone, path.clone());
            });
            validating_threads.push(thread);
        }
        for thread in validating_threads {
            thread.join().unwrap();
        }
    }
}

#[allow(unused_variables)]
fn validate_seed_file(
    file_hash: FileHash,
    seed_file: SeedFile,
    seed_database: Arc<Mutex<Database>>,
    mut database_path: PathBuf,
) {
    // TODO: Read metadata from the database
    database_path.push(file_hash.hash);
}

#[allow(unused)]
fn file_exists() -> bool {
    return true;
}

#[allow(unused)]
fn same_file_hash() -> bool {
    return true;
}
