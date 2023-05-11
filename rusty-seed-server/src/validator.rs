use std::{
    sync::{Arc, Mutex},
    thread,
};

use rusty_seed_core::file::{hash::FileHash, metadata::FileMetadata};
use rusty_seed_database::{error::DatabaseError, seeddb::SeedFileInfo, Database};
use tracing::{error, info};

pub struct DBValidator;

impl DBValidator {
    pub fn validate(database: Arc<Mutex<Database>>) {
        let database_lock = database.lock().unwrap();
        let seed_files = database_lock.get_all_seed_files();
        drop(database_lock);
        let mut validating_threads = Vec::new();
        for (hash, seed_file_info) in seed_files {
            if !seed_file_info.active {
                continue;
            }
            let seed_database_clone = Arc::clone(&database);
            let thread = thread::spawn(move || {
                validate_seed_file(hash, seed_file_info, seed_database_clone);
            });
            validating_threads.push(thread);
        }
        for thread in validating_threads {
            thread.join().unwrap();
        }
    }
}

fn validate_seed_file(
    file_hash: FileHash,
    seed_file_info: SeedFileInfo,
    database: Arc<Mutex<Database>>,
) {
    info!("Validating seed {:?}", seed_file_info.path);
    let database_lock = database.lock().unwrap();
    info!(
        "Loading metadata for {:?} from database",
        seed_file_info.path
    );
    let saved_metadata = database_lock.get_metadata(file_hash.clone());
    drop(database_lock);

    if !seed_file_info.path.exists() {
        error!(
            "Seed {:?} not found [REMOVING FROM DATABASE]",
            seed_file_info.path
        );
        let mut database_lock = database.lock().unwrap();
        database_lock.remove_seed_file(file_hash.clone());
    }

    let saved_metadata = match saved_metadata {
        Ok(metadata) => metadata,
        Err(e) => {
            error!(
                "Cannot load metadata for {:?}: {:?}",
                seed_file_info.path, e
            );
            return;
        }
    };

    let file_metadata = match FileMetadata::from(seed_file_info.path.clone()) {
        Ok(file_metadata) => file_metadata,
        Err(e) => {
            error!(
                "Cannot generate metadata for {:?}: {:?}",
                seed_file_info.path, e
            );
            return;
        }
    };

    if file_metadata != saved_metadata {
        error!(
            "Changes found for seed {:?} [REMOVING FROM DATABASE]",
            seed_file_info.path
        );
        database.lock().unwrap().remove_seed_file(file_hash);
    }

    info!("Validated seed {:?}", seed_file_info.path);
}

#[allow(unused_variables, unused)]
fn handle_database_error(database_error: DatabaseError, database: Arc<Mutex<Database>>) {
    match database_error {
        DatabaseError::SeedFileNotFound => {}
        DatabaseError::NoRecordFound => {}
        DatabaseError::CustomError { error } => {}
        DatabaseError::DatabaseNotFound => {}
    }
}
