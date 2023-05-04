use std::path::PathBuf;

#[derive(Debug)]
pub enum FileSystem {
    File {
        name: String,
        path: PathBuf,
        file_size: u64,
    },
    Directory {
        name: String,
        entries: Vec<FileSystem>,
    },
}

impl FileSystem {
    fn from_path(path: PathBuf) -> FileSystem {
        if path.is_file() {
            FileSystem::File {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                path: path.clone(),
                file_size: std::fs::metadata(path).unwrap().len(),
            }
        } else if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            let entries = std::fs::read_dir(&path)
                .unwrap()
                .map(|entry| FileSystem::from_path(entry.unwrap().path()))
                .collect();
            FileSystem::Directory { name, entries }
        } else {
            panic!("")
        }
    }
}
