use std::io::{self, Write};
use std::process::{Command as StdCommand, Stdio};

pub(crate) struct ProcessCommand;

impl ProcessCommand {
    pub(crate) fn write_stdin(program: &str, payload: &[u8]) -> Result<(), io::Error> {
        let mut child = StdCommand::new(program).stdin(Stdio::piped()).spawn()?;
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
        let output = StdCommand::new(program).stdout(Stdio::piped()).output()?;
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
