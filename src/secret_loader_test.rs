use crate::secret_loader::load_all;
use crate::secret_loader::load_from_path;
use std::ffi::OsString;
use crate::FilesystemError::NotDirectory;
use crate::FilesystemError::NotFile;
use crate::FilesystemError::Unreadable;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use assert2::{assert, check};

#[test]
fn load_all_with_no_directories_should_produce_an_empty_map() {
    let root = tempfile::tempdir().unwrap();

    let result = load_all(root.path());

    assert!(let Ok(secrets) = result);
    check!(secrets.is_empty());
}

#[test]
fn load_all_with_only_configs_should_return_configs() {
    let root = tempfile::tempdir().unwrap();
    let configs_path = root.path().join("run/configs");
    fs::create_dir_all(&configs_path).unwrap();
    let config_file = File::create(configs_path.join("A_CONFIG")).unwrap();
    write!(&config_file, "a_config_value").unwrap();

    let result = load_all(root.path());

    assert!(let Ok(values) = result);
    check!(values == HashMap::from([(OsString::from("A_CONFIG"), "a_config_value".to_string())]));
}

#[test]
fn load_all_with_only_secrets_should_return_secrets() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join("run/secrets");
    fs::create_dir_all(&secrets_path).unwrap();
    let secret_file = File::create(secrets_path.join("A_SECRET")).unwrap();
    write!(&secret_file, "a_secret_value").unwrap();

    let result = load_all(root.path());

    assert!(let Ok(values) = result);
    check!(values == HashMap::from([(OsString::from("A_SECRET"), "a_secret_value".to_string())]));
}

#[test]
fn load_all_merges_configs_and_secrets() {
    let root = tempfile::tempdir().unwrap();
    let configs_path = root.path().join("run/configs");
    fs::create_dir_all(&configs_path).unwrap();
    let config_file = File::create(configs_path.join("A_CONFIG")).unwrap();
    write!(&config_file, "a_config_value").unwrap();
    let secrets_path = root.path().join("run/secrets");
    fs::create_dir_all(&secrets_path).unwrap();
    let secret_file = File::create(secrets_path.join("A_SECRET")).unwrap();
    write!(&secret_file, "a_secret_value").unwrap();

    let result = load_all(root.path());

    assert!(let Ok(values) = result);
    check!(
        values
            == HashMap::from([
                (OsString::from("A_CONFIG"), "a_config_value".to_string()),
                (OsString::from("A_SECRET"), "a_secret_value".to_string()),
            ])
    );
}

#[test]
fn load_all_secrets_override_configs_on_conflict() {
    let root = tempfile::tempdir().unwrap();
    let configs_path = root.path().join("run/configs");
    fs::create_dir_all(&configs_path).unwrap();
    let config_file = File::create(configs_path.join("SHARED_KEY")).unwrap();
    write!(&config_file, "config_value").unwrap();
    let secrets_path = root.path().join("run/secrets");
    fs::create_dir_all(&secrets_path).unwrap();
    let secret_file = File::create(secrets_path.join("SHARED_KEY")).unwrap();
    write!(&secret_file, "secret_value").unwrap();

    let result = load_all(root.path());

    assert!(let Ok(values) = result);
    check!(values == HashMap::from([(OsString::from("SHARED_KEY"), "secret_value".to_string())]));
}

#[test]
fn no_secrets_directory_should_produce_an_empty_map() {
    let root = tempfile::tempdir().unwrap();

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Ok(secrets) = result);
    check!(secrets.is_empty());
}

#[test]
fn fail_if_secrets_path_is_not_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let var_run_path = root.path().join(Path::new("run"));
    fs::create_dir_all(var_run_path.as_path()).unwrap();
    let secrets_path = var_run_path.join(Path::new("secrets"));
    File::create(&secrets_path).unwrap();
    println!("{:?}", secrets_path);

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Err(error) = result);
    check!(error == NotDirectory(secrets_path));
}

#[test]
fn empty_secrets_directory_should_produce_an_empty_map() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Ok(secrets) = result);
    check!(secrets.is_empty());
}

#[test]
fn fail_if_secret_entry_is_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("run/secrets/something"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Err(error) = result);
    check!(error == NotFile(secrets_path));
}

#[test]
fn a_valid_secret_should_be_returned() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();
    let secret_path = secrets_path.join(Path::new("A_SECRET"));
    let secret_file = File::create(&secret_path).unwrap();
    write!(&secret_file, "{}", "a_secret_value").unwrap();

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Ok(secrets) = result);
    check!(secrets == HashMap::from([("A_SECRET".into(), "a_secret_value".to_string())]));
}

#[test]
fn fail_if_secrets_directory_is_not_readable() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();
    let mut perms = fs::metadata(&secrets_path).unwrap().permissions();
    perms.set_mode(0644);
    fs::set_permissions(&secrets_path, perms).unwrap();

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Err(error) = result);
    check!(error == Unreadable(secrets_path));
}

#[test]
fn fail_if_secret_file_is_not_readable() {
    let root = tempfile::tempdir().unwrap();
    let secrets_path = root.path().join(Path::new("run/secrets"));
    fs::create_dir_all(secrets_path.as_path()).unwrap();
    let secret_path = secrets_path.join(Path::new("A_SECRET"));
    File::create(&secret_path).unwrap();
    let mut perms = fs::metadata(&secret_path).unwrap().permissions();
    perms.set_mode(0000);
    fs::set_permissions(&secret_path, perms).unwrap();

    let result = load_from_path(root.path(), "run/secrets");

    assert!(let Err(error) = result);
    check!(error == Unreadable(secret_path));
}
