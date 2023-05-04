#[derive(Debug)]
pub enum FileError {
    InvalidFilePath,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFilePath => write!(f, "Invalid File Path"),
        }
    }
}
