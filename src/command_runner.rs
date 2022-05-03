use crate::{ExecutionFailed, FilesystemError, NoCommandToExecute, SecretAdapterError};
use std::collections::HashMap;
use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, io};

#[cfg(test)]
#[path = "./command_runner_test.rs"]
mod command_runner_test;

#[derive(Debug, PartialEq)]
pub enum CommandError {
    NoCommandToExecute,
    ExecutionFailed(io::ErrorKind),
}

#[derive(Debug, PartialEq)]
struct Process {
    executable: String,
    args: Vec<String>,
}

impl From<FilesystemError> for SecretAdapterError {
    fn from(e: FilesystemError) -> Self {
        SecretAdapterError::Filesystem(e)
    }
}

impl From<CommandError> for SecretAdapterError {
    fn from(e: CommandError) -> Self {
        SecretAdapterError::Command(e)
    }
}

pub fn run_command<'a>(secrets: &HashMap<OsString, String>) -> Result<(), SecretAdapterError> {
    let args = env::args().collect();
    let process = parse_command_line(args)?;

    let command_error = Command::new(process.executable)
        .args(process.args)
        .envs(secrets)
        .exec();

    Err(SecretAdapterError::Command(ExecutionFailed(
        command_error.kind(),
    )))
}

fn parse_command_line(mut args: Vec<String>) -> Result<Process, CommandError> {
    args.remove(0);
    if args.is_empty() {
        Err(NoCommandToExecute)
    } else {
        let executable = args.remove(0);

        Ok(Process { executable, args })
    }
}
