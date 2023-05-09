use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use sha1::{digest::core_api::CoreWrapper, Digest, Sha1, Sha1Core};

use crate::utils::FILE_READ_WRITE_BUFFER_SIZE;

use super::metadata::FileSystem;

fn hash(file_system: FileSystem, hasher: &mut CoreWrapper<Sha1Core>) {
    match file_system {
        FileSystem::File {
            name: _,
            path,
            file_size: _,
        } => hash_file(path, hasher),
        FileSystem::Directory { name: _, entries } => {
            for entry in entries {
                hash(entry, hasher);
            }
        }
        FileSystem::UnsupportedType => unreachable!(),
    }
}

fn hash_file(path: PathBuf, hasher: &mut CoreWrapper<Sha1Core>) {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut buffer = vec![0; FILE_READ_WRITE_BUFFER_SIZE];

    loop {
        let bytes_read = buf_reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        Digest::update(hasher, &buffer[..bytes_read]);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileHash {
    pub hash: String,
}

impl FileHash {
    pub fn from(file_system: FileSystem) -> Self {
        let mut hasher = Sha1::new();
        hash(file_system, &mut hasher);
        let result = hasher.finalize();
        Self {
            hash: format!("{:x}", result),
        }
    }

    pub fn from_string(hash: String) -> Self {
        Self { hash }
    }
}

impl AsRef<[u8]> for FileHash {
    fn as_ref(&self) -> &[u8] {
        self.hash.as_bytes()
    }
}

#[cfg(test)]
mod test {
    use crate::{file::metadata::FileMetadata, utils::default_database_path};

    use super::FileHash;

    #[test]
    fn hash_file() {
        let mut path = default_database_path();
        path.push("test-dir");

        let file_system = FileMetadata::from(path).unwrap().file_system;
        let file_hash = FileHash::from(file_system);
        println!("{}", file_hash.hash);
    }
}
