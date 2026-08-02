use std::io::{self, Write};
use std::process::{Command as StdCommand, Stdio};

pub(crate) struct ProcessCommand;

impl ProcessCommand {
    pub(crate) fn write_stdin(program: &str, payload: &[u8]) -> Result<(), io::Error> {
        let mut child = StdCommand::new(program).stdin(Stdio::piped()).spawn()?;
        child
            .stdin
            .as_mut()
            .into_iter()
            .try_for_each(|stdin| stdin.write_all(payload))?;
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

#[cfg(test)]
mod tests {
    use super::ProcessCommand;
    use crate::test_assert::KucTestExpect;

    #[test]
    fn process_command_covers_success_failure_and_spawn_errors() {
        assert!(ProcessCommand::write_stdin("/bin/cat", b"payload").is_ok());
        assert!(ProcessCommand::write_stdin("/usr/bin/false", b"").is_err());
        assert!(ProcessCommand::write_stdin("/missing/kuc-command", b"").is_err());

        assert_eq!(
            "",
            ProcessCommand::read_stdout("/usr/bin/true").kuc_unwrap()
        );
        assert!(ProcessCommand::read_stdout("/usr/bin/false").is_err());
        assert!(ProcessCommand::read_stdout("/missing/kuc-command").is_err());
    }
}
