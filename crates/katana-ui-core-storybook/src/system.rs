use std::ffi::OsStr;
use std::io::{self, Write};
use std::process::{Command as StdCommand, Stdio};

pub(crate) struct ProcessService;

impl ProcessService {
    pub(crate) fn create_command<S>(program: S) -> StdCommand
    where
        S: AsRef<OsStr>,
    {
        let mut command = StdCommand::new(program);
        apply_silent_policy(&mut command);
        command
    }
}

pub(crate) struct ProcessCommand;

impl ProcessCommand {
    pub(crate) fn write_stdin(program: &str, payload: &[u8]) -> Result<(), io::Error> {
        let mut child = ProcessService::create_command(program)
            .stdin(Stdio::piped())
            .spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(payload)?;
        }
        let status = child.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::other(format!(
                "{program} failed with status {status}"
            )))
        }
    }

    pub(crate) fn read_stdout(program: &str) -> Result<String, io::Error> {
        let output = ProcessService::create_command(program)
            .stdout(Stdio::piped())
            .output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(io::Error::other(format!(
                "{program} failed with status {}",
                output.status
            )))
        }
    }
}

#[cfg(test)]
const _: ProcessCommand = ProcessCommand;

#[cfg(test)]
const _: fn(&str, &[u8]) -> Result<(), io::Error> = ProcessCommand::write_stdin;

#[cfg(test)]
const _: fn(&str) -> Result<String, io::Error> = ProcessCommand::read_stdout;

#[cfg(windows)]
fn apply_silent_policy(command: &mut StdCommand) {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn apply_silent_policy(_command: &mut StdCommand) {}
