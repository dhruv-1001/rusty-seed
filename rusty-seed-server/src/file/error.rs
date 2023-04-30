#[derive(Debug)]
pub enum Error {
    InvalidFilePath,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFilePath => write!(f, "Invalid File Path"),
        }
    }
}
