use std::{
    io::{self, Read},
    process::{Child, Command, Output, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const MAX_STDOUT_BYTES: usize = 4096;
const READER_BUFFER_BYTES: usize = 512;
const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn output_with_timeout(mut command: Command, timeout: Duration) -> io::Result<Output> {
    if timeout.is_zero() {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "input method lookup timed out before spawn",
        ));
    }
    let deadline = Instant::now().checked_add(timeout).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "input method lookup timeout overflows the monotonic clock",
        )
    })?;
    command.stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = command.spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("input method stdout was not piped"))?;
    let (sender, receiver) = mpsc::channel();
    let reader = thread::Builder::new()
        .name("native-ime-output-reader".into())
        .spawn(move || {
            let mut stdout = stdout;
            let result = read_limited(&mut stdout);
            let _ = sender.send(result);
        });
    if let Err(error) = reader {
        terminate(&mut child);
        return Err(error);
    }
    loop {
        let status = match child.try_wait() {
            Ok(status) => status,
            Err(error) => {
                terminate(&mut child);
                return Err(error);
            }
        };
        if let Some(status) = status {
            let remaining = deadline.saturating_duration_since(Instant::now());
            let stdout = receiver.recv_timeout(remaining).map_err(|error| {
                io::Error::new(
                    if error == mpsc::RecvTimeoutError::Timeout {
                        io::ErrorKind::TimedOut
                    } else {
                        io::ErrorKind::BrokenPipe
                    },
                    "input method stdout reader did not finish",
                )
            })??;
            return Ok(Output {
                status,
                stdout,
                stderr: Vec::new(),
            });
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            terminate(&mut child);
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "input method lookup timed out",
            ));
        }
        thread::sleep(remaining.min(POLL_INTERVAL));
    }
}

fn read_limited(reader: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer = [0_u8; READER_BUFFER_BYTES];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            return Ok(output);
        }
        if output.len() + read > MAX_STDOUT_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "input method output exceeds the bounded capture size",
            ));
        }
        output.extend_from_slice(&buffer[..read]);
    }
}

fn terminate(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(test)]
mod tests {
    use super::output_with_timeout;
    use std::{
        process::Command,
        time::{Duration, Instant},
    };

    #[test]
    fn captures_successful_process_output() {
        let output =
            output_with_timeout(shell_command("printf native-ime"), Duration::from_secs(1))
                .expect("process should finish");
        assert!(output.status.success());
        assert_eq!(output.stdout, b"native-ime");
    }

    #[test]
    fn preserves_nonzero_process_status() {
        let output = output_with_timeout(shell_command("exit 7"), Duration::from_secs(1))
            .expect("process should finish");
        assert_eq!(output.status.code(), Some(7));
    }

    #[test]
    fn kills_and_reaps_sleeping_process_on_timeout() {
        let started = Instant::now();
        let error = output_with_timeout(shell_command("sleep 5"), Duration::from_millis(40))
            .expect_err("sleep must time out");
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(started.elapsed() < Duration::from_secs(1));
    }

    fn shell_command(script: &str) -> Command {
        let mut command = Command::new("sh");
        command.args(["-c", script]);
        command
    }
}
