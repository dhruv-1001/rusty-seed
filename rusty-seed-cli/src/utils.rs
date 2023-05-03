use std::{io::Write, path::PathBuf};

pub fn generate_test_file(
    name: String,
    mut path: PathBuf,
    size: usize,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(&path)?;
    path.push(name);
    let mut file = std::fs::File::create(path)?;

    let buffer = [0u8; 8192];
    let buffer_len = buffer.len();
    let mut remaining_size = size;

    while remaining_size > buffer_len {
        file.write_all(&buffer[..(buffer_len)])?;
        remaining_size -= buffer_len
    }
    file.write_all(&buffer[..(remaining_size)])?;

    Ok(())
}

pub fn generate_test_dir(
    name: String,
    mut path: PathBuf,
    num_files: usize,
    size: usize,
) -> Result<(), std::io::Error> {
    path.push(&name);
    std::fs::create_dir_all(&path)?;

    if num_files == 1 {
        generate_test_file(name, path, size)
    } else {
        Ok(())
    }
}
