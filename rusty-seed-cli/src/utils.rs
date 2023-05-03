use std::path::PathBuf;

#[allow(unused)]
pub fn generate_test_file(
    name: String,
    mut path: PathBuf,
    size: usize,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&path);
    path.push(name);
    let mut file = std::fs::File::create(path)?;
    file.set_len(size as u64);
    Ok(())
}

#[allow(unused)]
pub fn generate_test_dir(name: String, path: PathBuf, num_files: usize, size: usize) {}
