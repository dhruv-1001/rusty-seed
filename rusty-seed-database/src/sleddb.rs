use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct SledDatabase {
    db: sled::Db,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    path: String,
    active: bool,
}

#[allow(unused_variables)]
impl SledDatabase {
    pub fn open(path: &Path) -> Self {
        let db = sled::open(path).unwrap();
        Self { db }
    }

    pub fn add_file(&mut self, hash: String, path: &Path) {
        let file = File {
            path: path.to_str().unwrap().to_string(),
            active: true,
        };
        let serialized_file = serialize(&file).unwrap();
        self.db.insert(hash, serialized_file).unwrap();
    }

    pub fn get_path(&self, hash: String) -> File {
        deserialize(&self.db.get(hash).unwrap().unwrap()).unwrap()
    }

    pub fn remove_file(&self, hash: String) {}

    pub fn remove_file_from_server(&self, hash: Option<String>) {}

    pub fn replace_with_hash(&self, hash: String, path: &Path) {}

    pub fn replace_with_path(&self, hash: String, path: &Path) {}

    pub fn get_all_files(&self, hash: String) {}

    pub fn get_active_files(&self) {}

    pub fn mark_inactive(&mut self, hash: String) {}
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::SledDatabase;

    #[test]
    fn test() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(".test-db");
        let mut sled_db = SledDatabase::open(path.as_path());
        let file_path = Path::new("path/for/test");
        sled_db.add_file("1234".to_string(), file_path);
        let file = sled_db.get_path("1234".to_string());
        assert_eq!("path/for/test".to_string(), file.path);
        std::fs::remove_dir_all(path).unwrap();
    }
}
