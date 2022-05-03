use crate::secret_loader::FilesystemError::{NotDirectory, NotFile, Unknown, Unreadable};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[cfg(test)]
#[path = "./secret_loader_test.rs"]
mod secret_loader_test;

struct Secret {
    name: OsString,
    value: String,
}

#[derive(Debug, PartialEq)]
pub enum FilesystemError {
    NotDirectory(PathBuf),
    NotFile(PathBuf),
    Unreadable(PathBuf),
    Unknown(io::ErrorKind),
}

macro_rules! merge {
    ($first:expr,$second:expr) => {
        $first.into_iter().chain($second).collect()
    };
}

pub fn load_all(root: &Path) -> Result<HashMap<OsString, String>, FilesystemError> {
    let configs = load_from_path(root, "var/run/configs")?;
    let secrets = load_from_path(root, "var/run/secrets")?;
    Ok(merge!(configs, secrets))
}

fn load_from_path(
    root: &Path,
    secrets_path: &str,
) -> Result<HashMap<OsString, String>, FilesystemError> {
    let mut full_secrets_path = root.to_path_buf();
    full_secrets_path.push(secrets_path);

    if !full_secrets_path.exists() {
        return Ok(HashMap::new());
    }

    if !full_secrets_path.is_dir() {
        return Err(NotDirectory(full_secrets_path.clone()));
    }

    let entries: Vec<DirEntry> = get_secret_directory_entries(&full_secrets_path)?;
    let mut result = HashMap::new();

    for entry in entries {
        let entry_path = entry.path();
        let secret_name = get_secret_name(&entry_path)?;
        let secret = read_secret(&entry_path, secret_name)?;
        result.insert(secret.name, secret.value);
    }

    return Ok(result);
}

fn get_secret_directory_entries(secrets_path: &PathBuf) -> Result<Vec<DirEntry>, FilesystemError> {
    match fs::read_dir(&secrets_path) {
        Ok(entries) => entries
            .collect::<Result<Vec<DirEntry>, io::Error>>()
            .map_err(|io_error| Unknown(io_error.kind())),
        Err(_) => Err(Unreadable(secrets_path.clone())),
    }
}

fn get_secret_name(secret_file_path: &PathBuf) -> Result<OsString, FilesystemError> {
    if secret_file_path.is_dir() {
        Err(NotFile(secret_file_path.clone()))
    } else {
        Ok(secret_file_path.file_name().unwrap().into())
    }
}

fn read_secret(secret_file_path: &PathBuf, name: OsString) -> Result<Secret, FilesystemError> {
    return match fs::read_to_string(&secret_file_path) {
        Ok(value) => Ok(Secret { name, value }),
        Err(_) => Err(Unreadable(secret_file_path.clone())),
    };
}
