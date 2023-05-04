use std::path::PathBuf;

pub fn default_database_path() -> PathBuf {
    let mut path = PathBuf::from(env!("HOME"));
    path.push(".rustyseed");
    path
}