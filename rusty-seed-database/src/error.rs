#[derive(Debug)]
pub enum DatabaseError {
    SeedFileNotFound,
    NoRecordFound,
    CustomError { error: String },
}
