use katana_ui_core_native_ime_evidence::{NativeRunner, RunnerOptions};
use std::{
    env,
    path::PathBuf,
    process::ExitCode,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let value = |name: &str| -> Result<String, String> {
        args.iter()
            .find_map(|arg| arg.strip_prefix(&format!("--{name}=")).map(str::to_owned))
            .ok_or_else(|| format!("missing --{name}=VALUE"))
    };
    let timeout_ms = || -> Result<u64, String> {
        value("timeout-ms")
            .unwrap_or_else(|_| "600000".into())
            .parse()
            .map_err(|_| "--timeout-ms must be an integer".into())
    };
    let result = (|| {
        let options = RunnerOptions {
            revision: value("revision")?,
            challenge: value("challenge")?,
            producer_id: value("producer-id")?,
            runner_id: value("runner-id")?,
            run_id: format!(
                "native-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            ),
            output: PathBuf::from(value("output")?),
            timeout: Duration::from_millis(timeout_ms()?),
        };
        NativeRunner::run(options)
            .map_err(|e| e.to_string())
            .map(|_| ())
    })();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("native-ime-evidence: {error}");
            ExitCode::FAILURE
        }
    }
}
