use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::error::FileError;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum FileSystem {
    File {
        name: String,
        path: PathBuf,
        file_size: u64,
    },
    Directory {
        name: String,
        entries: Vec<FileSystem>,
    },
    UnsupportedType,
}

impl FileSystem {
    fn from_path(path: PathBuf) -> Result<FileSystem, FileError> {
        if !path.exists() {
            Err(FileError::InvalidFilePath)
        }
        else if path.is_file() {
            Ok(FileSystem::File {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                path: path.clone(),
                file_size: std::fs::metadata(path).unwrap().len(),
            })
        } else if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            let entries = std::fs::read_dir(&path)
                .unwrap()
                .map(|entry| FileSystem::from_path(entry.unwrap().path()).unwrap())
                .collect();
            Ok(FileSystem::Directory { name, entries })
        } else {
            Ok(FileSystem::UnsupportedType)
        }
    }

    fn size(&self, mut start: u64) -> u64 {
        match self {
            FileSystem::File { name: _, path: _, file_size } => { 
                start + file_size
            },
            FileSystem::Directory { name: _, entries } => { 
                for entry in entries {
                    start += entry.size(0);
                }
                start
            },
            FileSystem::UnsupportedType => {  
                start
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    file_path: PathBuf,
    file_system: FileSystem,
    seed_size: u64,
    can_seed: bool
}

impl FileMetadata {
    pub fn from(path: PathBuf) -> Result<Self, FileError> {
        let file_system = FileSystem::from_path(path.clone())?;
        Ok(FileMetadata {
            file_path: path.clone(),
            file_system: file_system.clone(),
            seed_size: file_system.size(0),
            can_seed: true,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::utils::default_database_path;

    use super::FileMetadata;

    #[test]
    fn test_generate_meatdata() {
        let mut path = default_database_path();
        path.push("bitcoin");

        let file_metadata = FileMetadata::from(path).unwrap();
        println!("{:#?}", file_metadata);
    }
}