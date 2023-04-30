pub mod sleddb;
pub mod error;
use sleddb::SledDatabase;

pub struct Database {
    sled: SledDatabase
}

#[allow(unused_variables)]
impl Database {
    // Open and return existing database, if not present, it create and return a new one
    fn open() {
        todo!()
    }

    // creates a new database and return 
    fn new() {
        todo!()
    }
}