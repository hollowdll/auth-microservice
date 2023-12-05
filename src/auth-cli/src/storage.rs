use std::{
    path::{Path, PathBuf},
    fs::{
        File,
        read_to_string,
    },
    io::{self, Write},
    env::current_exe,
};

/// JWT access token is stored in this file.
pub const JWT_FILE_NAME: &str = "jwt_access_token";

/// Writes a buffer to a file overwriting it.
/// Creates the file if it doesn't exist.
fn write_buf_to_file(buf: &[u8], file_path: &Path) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(buf)?;

    Ok(())
}

/// Gets file path to the executable's parent dir
fn get_exec_parent_dir() -> io::Result<PathBuf> {
    let mut dir = current_exe()?;
    dir.pop();

    Ok(dir)
}

/// Stores JWT access token to a file
/// in the executable's parent directory.
pub fn store_jwt(jwt: &[u8]) -> io::Result<()> {
    let dir = get_exec_parent_dir()?;
    let file_path = dir.join(JWT_FILE_NAME);

    write_buf_to_file(jwt, &file_path)?;

    Ok(())
}

/// Gets JWT access token from a file
/// in the executable's parent directory.
pub fn get_jwt() -> io::Result<String> {
    let dir = get_exec_parent_dir()?;
    let file_path = dir.join(JWT_FILE_NAME);

    if !file_path.is_file() {
        File::create(&file_path)?;
    }
    let jwt = read_to_string(file_path)?;

    Ok(jwt)
}