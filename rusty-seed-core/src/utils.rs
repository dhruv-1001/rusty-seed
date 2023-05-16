use std::path::PathBuf;

pub const FILE_READ_WRITE_BUFFER_SIZE: usize = 1024 * 1024; // 1 MiB
pub const FILE_READ_WRITE_BUFFER_SIZE_FOR_TCP: usize = 1024 * 64; // 64 KiB

pub fn default_database_path() -> PathBuf {
    let mut path = PathBuf::from(env!("HOME"));
    path.push(".rustyseed");
    path
}

pub fn default_download_path() -> PathBuf {
    let mut path = PathBuf::from(env!("HOME"));
    path.push("Downloads");
    path
}
