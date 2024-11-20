use assert2::{check, let_assert};
use crate::command_runner::CommandError::NoCommandToExecute;
use crate::command_runner::{parse_command_line, Process};

macro_rules! svec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn do_not_build_a_process_if_no_arguments_given() {
    let args = svec!["self"];

    let result = parse_command_line(args);

    let_assert!(Err(error) = result);
    check!(error == NoCommandToExecute);
}

#[test]
fn the_first_argument_should_be_set_as_the_executable() {
    let args = svec!["self", "my_bin"];

    let result = parse_command_line(args);

    let_assert!(Ok(process) = result);
    check!(process == Process {
            executable: "my_bin".into(),
            args: svec![]
        });
}

#[test]
fn the_following_arguments_should_be_set_as_executable_arguments() {
    let args = svec!["self", "my_bin", "arg1", "arg2"];

    let result = parse_command_line(args);

    let_assert!(Ok(process) = result);
    check!(process == Process {
            executable: "my_bin".into(),
            args: svec!["arg1", "arg2"]
        });
}
