use crate::secret_loader::load_from_path;
use crate::FilesystemError::NotDirectory;
use crate::FilesystemError::NotFile;
use crate::FilesystemError::Unreadable;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use assert2::{check, let_assert};

#[test]
fn no_secrets_directory_should_produce_an_empty_map() {
    let root = tempfile::tempdir().unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Ok(secrets) = result);
    check!(secrets.is_empty());
}

#[test]
fn fail_if_secrets_path_is_not_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let var_run_path = root.path().join(Path::new("var/run"));
    fs::create_dir_all(var_run_path.as_path()).unwrap();
    let secrets_path = var_run_path.join(Path::new("secrets"));
    File::create(&secrets_path).unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Err(error) = result);
    check!(error == NotDirectory(secrets_path));
}

#[test]
fn empty_secrets_directory_should_produce_an_empty_map() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("var/run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Ok(secrets) = result);
    check!(secrets.is_empty());
}

#[test]
fn fail_if_secret_entry_is_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("var/run/secrets/something"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Err(error) = result);
    check!(error == NotFile(secrets_path));
}

#[test]
fn a_valid_secret_should_be_returned() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("var/run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();
    let secret_path = secrets_path.join(Path::new("A_SECRET"));
    let secret_file = File::create(&secret_path).unwrap();
    write!(&secret_file, "{}", "a_secret_value").unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Ok(secrets) = result);
    check!(secrets == HashMap::from([("A_SECRET".into(), "a_secret_value".to_string())]));
}

#[test]
fn fail_if_secrets_directory_is_not_readable() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("var/run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();
    let mut perms = fs::metadata(&secrets_path).unwrap().permissions();
    perms.set_mode(0644);
    fs::set_permissions(&secrets_path, perms).unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Err(error) = result);
    check!(error == Unreadable(secrets_path));
}

#[test]
fn fail_if_secret_file_is_not_readable() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("var/run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();
    let secret_path = secrets_path.join(Path::new("A_SECRET"));
    File::create(&secret_path).unwrap();
    let mut perms = fs::metadata(&secret_path).unwrap().permissions();
    perms.set_mode(0000);
    fs::set_permissions(&secret_path, perms).unwrap();

    let result = load_from_path(root.path(), "var/run/secrets");

    let_assert!(Err(error) = result);
    check!(error == Unreadable(secret_path));
}
