use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DatabaseError {
    SeedFileNotFound,
    NoRecordFound,
    DatabaseNotFound,
    CustomError { error: String },
}
