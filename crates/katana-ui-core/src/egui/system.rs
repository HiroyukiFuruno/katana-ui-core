use std::ffi::OsStr;
use std::process::{Command as StdCommand, Output};

#[derive(Debug, Default)]
pub(crate) struct ProcessService;

impl ProcessService {
    pub(crate) fn create_command<P>(path: P) -> StdCommand
    where
        P: AsRef<OsStr>,
    {
        let mut command = StdCommand::new(path);
        apply_silent_policy(&mut command);
        command
    }

    pub(crate) fn run_output<P, I, A>(&self, path: P, args: I) -> std::io::Result<Output>
    where
        P: AsRef<OsStr>,
        I: IntoIterator<Item = A>,
        A: AsRef<OsStr>,
    {
        Self::create_command(path).args(args).output()
    }
}

#[cfg(not(windows))]
fn apply_silent_policy(_command: &mut StdCommand) {}

#[cfg(windows)]
fn apply_silent_policy(command: &mut StdCommand) {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}
