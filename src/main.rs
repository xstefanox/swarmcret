use crate::command_runner::CommandError::ExecutionFailed;
use crate::command_runner::{run_command, CommandError};
use crate::secret_loader::FilesystemError::{NotDirectory, NotFile, Unknown, Unreadable};
use crate::secret_loader::{load_all, FilesystemError};
use crate::CommandError::NoCommandToExecute;
use std::fmt::{Display, Formatter};
use std::path::Path;

mod command_runner;
mod secret_loader;

#[cfg(test)]
mod test_macros;

fn main() -> Result<(), String> {
    let result = load_and_run();

    match result {
        Ok(_) => Ok(()),
        Err(error) => match error {
            SecretAdapterError::Filesystem(_) => Err(format!("{}", error)),
            SecretAdapterError::Command(command_error) => match command_error {
                NoCommandToExecute => Ok(()),
                ExecutionFailed(cause) => Err(format!("{}", cause)),
            },
        },
    }
}

fn load_and_run() -> Result<(), SecretAdapterError> {
    let root = Path::new("/");
    let secrets = load_all(root)?;
    run_command(&secrets)?;
    Ok(())
}

pub enum SecretAdapterError {
    Filesystem(FilesystemError),
    Command(CommandError),
}

macro_rules! to_string {
    ($path:expr) => {
        $path.to_str().unwrap_or("")
    };
}

impl Display for SecretAdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            SecretAdapterError::Filesystem(error) => match error {
                NotDirectory(path) => format!("not a directory: '{}'", to_string!(path)),
                NotFile(path) => format!("not a file: '{}'", to_string!(path)),
                Unreadable(path) => format!("unreadable: '{}'", to_string!(path)),
                Unknown(error) => format!("unknown error while reading: '{}'", error.to_string()),
            },
            SecretAdapterError::Command(error) => match error {
                NoCommandToExecute => "no command to execute".to_string(),
                ExecutionFailed(error) => {
                    format!("error while executing process: '{}'", error.to_string())
                }
            },
        };
        write!(f, "{}", message)
    }
}
