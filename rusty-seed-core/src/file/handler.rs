use std::{
    cmp::min,
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

use crate::utils::FILE_READ_WRITE_BUFFER_SIZE_FOR_TCP;

use super::{error::FileError, metadata::FileMetadata};

type Size = u64;

pub struct FileHandler {
    file_metadata: FileMetadata,
    files: Vec<(PathBuf, Size)>,
}

impl FileHandler {
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

        let mut file = File::open(file_path).unwrap();
        file.seek(std::io::SeekFrom::Start(index)).unwrap();

        let bytes_to_read = min(
            file_size - index,
            FILE_READ_WRITE_BUFFER_SIZE_FOR_TCP as u64,
        );

        let mut buffer: Vec<u8> = vec![0; bytes_to_read as usize];
        file.read_exact(&mut buffer).unwrap();

        Ok((buffer, index, index + bytes_to_read - 1))
    }
}
