use crate::command_runner::CommandError::NoCommandToExecute;
use crate::command_runner::{parse_command_line, Process};
use crate::{assert_err, assert_ok};

macro_rules! svec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn do_not_build_a_process_if_no_arguments_given() {
    let args = svec!["self"];

    let process = parse_command_line(args);

    assert_err!(NoCommandToExecute, process);
}

#[test]
fn the_first_argument_should_be_set_as_the_executable() {
    let args = svec!["self", "my_bin"];

    let process = parse_command_line(args);

    assert_ok!(
        Process {
            executable: "my_bin".into(),
            args: svec![]
        },
        process
    );
}

#[test]
fn the_following_arguments_should_be_set_as_executable_arguments() {
    let args = svec!["self", "my_bin", "arg1", "arg2"];

    let process = parse_command_line(args);

    assert_ok!(
        Process {
            executable: "my_bin".into(),
            args: svec!["arg1", "arg2"]
        },
        process
    );
}
