#[derive(Debug)]
pub enum DatabaseError {
    SeedFileNotFound,
    NoRecordFound,
    DatabaseNotFound,
    CustomError { error: String },
}
