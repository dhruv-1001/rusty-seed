pub enum DatabaseError {
    SeedFileNotFound,
    CustomError { error: String },
}
