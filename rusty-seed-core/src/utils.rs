use std::path::PathBuf;

pub const FILE_READ_WRITE_BUFFER_SIZE: usize = 8192;
pub const FILE_READ_WRITE_BUFFER_SIZE_FOR_TCP: usize = 64;

pub fn default_database_path() -> PathBuf {
    let mut path = PathBuf::from(env!("HOME"));
    path.push(".rustyseed");
    path
}
