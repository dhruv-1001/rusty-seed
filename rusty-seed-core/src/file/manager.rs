use std::{
    cmp::min,
    fs::{File, OpenOptions},
    io::{Read, Seek},
    os::unix::prelude::FileExt,
    path::PathBuf,
};

use crate::utils::FILE_READ_WRITE_BUFFER_SIZE_FOR_TCP;

use super::{error::FileError, metadata::FileMetadata};

type Size = u64;

pub struct FileManager {
    file_metadata: FileMetadata,
    files: Vec<(PathBuf, Size)>,
}

impl FileManager {
    pub fn new(file_metadata: FileMetadata) -> Self {
        let mut files = Vec::new();
        file_metadata.file_system.fill_file_with_size(&mut files);
        Self {
            file_metadata,
            files,
        }
    }

    // Returns byte vector, along with start_index and end_index of bytes read
    pub fn read(&self, index: u64, path: PathBuf) -> Result<(Vec<u8>, u64, u64), FileError> {
        let mut file_path = self.file_metadata.file_path.clone();
        file_path.push(path.clone());

        let mut file: Option<(PathBuf, Size)> = None;
        for (path, size) in &self.files {
            if path.eq(&file_path) {
                file = Some((path.clone(), *size));
                break;
            }
        }
        let file_size = match file {
            Some((_, size)) => size,
            None => {
                return Err(FileError::InvalidFilePath);
            }
        };

        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return Err(FileError::InvalidFilePath),
        };
        file.seek(std::io::SeekFrom::Start(index)).unwrap();

        let bytes_to_read = min(
            file_size - index,
            FILE_READ_WRITE_BUFFER_SIZE_FOR_TCP as u64,
        );

        let mut buffer: Vec<u8> = vec![0; bytes_to_read as usize];
        file.read_exact(&mut buffer).unwrap();

        Ok((buffer, index, index + bytes_to_read - 1))
    }

    pub fn write(&self, index: u64, bytes: Vec<u8>, path: PathBuf) -> Result<(), FileError> {
        let file = match OpenOptions::new().write(true).open(path) {
            Ok(file) => file,
            Err(_) => return Err(FileError::InvalidFilePath),
        };
        file.write_all_at(&bytes[..], index).unwrap();
        Ok(())
    }
}
