use std::{io::Write, path::PathBuf};

use rand::Rng;

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
        generate_random_file(path.clone(), size)?
    }

    let nested_files = num_files / 2;
    let root_files = num_files - nested_files;
    let file_size = size / num_files;

    for _ in 0..root_files {
        generate_random_file(path.clone(), file_size).unwrap();
    }

    path.push(&generate_random_string(6));
    std::fs::create_dir_all(&path)?;

    for _ in 0..(nested_files - 1) {
        generate_random_file(path.clone(), file_size).unwrap();
    }
    let remaining_size = file_size + size % num_files;
    generate_random_file(path, remaining_size).unwrap();

    Ok(())
}

fn generate_random_file(path: PathBuf, size: usize) -> Result<(), std::io::Error> {
    generate_test_file(generate_random_string(6), path, size)
}

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let bytes = (0..length)
        .map(|_| rng.gen_range(0..CHARSET.len()))
        .map(|i| CHARSET[i])
        .collect::<Vec<_>>();
    String::from_utf8(bytes).unwrap()
}
