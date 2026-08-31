#[cfg(target_os = "linux")]
use std::ffi::OsString;
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "linux")]
use std::process::{Child, Command};

#[cfg(target_os = "linux")]
const MAX_PARALLEL_CASES: usize = 2;

#[cfg(target_os = "linux")]
struct ContractCase {
    label: &'static str,
    args: Vec<OsString>,
    expected_code: i32,
    artifact_dir: Option<PathBuf>,
}

#[cfg(target_os = "linux")]
impl ContractCase {
    fn new(label: &'static str, expected_code: i32, args: &[&str]) -> Self {
        Self {
            label,
            args: args.iter().map(OsString::from).collect(),
            expected_code,
            artifact_dir: None,
        }
    }

    fn with_path(
        label: &'static str,
        expected_code: i32,
        flag: &str,
        path: &Path,
        artifact_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            label,
            args: vec![OsString::from(flag), path.as_os_str().to_owned()],
            expected_code,
            artifact_dir,
        }
    }
}

#[cfg(target_os = "linux")]
struct RunningCase {
    case: ContractCase,
    child: Child,
}

#[cfg(target_os = "linux")]
fn wait_batch(mut batch: Vec<RunningCase>) -> Result<(), Box<dyn std::error::Error>> {
    let mut failures = Vec::new();
    for mut running in batch.drain(..) {
        match running.child.wait() {
            Ok(status) if status.code() == Some(running.case.expected_code) => {
                if running
                    .case
                    .artifact_dir
                    .as_ref()
                    .is_some_and(|path| !path.is_dir())
                {
                    failures.push(format!(
                        "{} did not create its artifact directory",
                        running.case.label
                    ));
                }
            }
            Ok(status) => failures.push(format!(
                "{} exited with {:?}, expected {}",
                running.case.label,
                status.code(),
                running.case.expected_code
            )),
            Err(error) => failures.push(format!("{} wait failed: {error}", running.case.label)),
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; ").into())
    }
}

#[cfg(target_os = "linux")]
fn run_cases(
    executable: &str,
    trace_root: &Path,
    cases: Vec<ContractCase>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut batch = Vec::with_capacity(MAX_PARALLEL_CASES);
    for (index, case) in cases.into_iter().enumerate() {
        let trace = trace_root.join(format!(
            "native-window-contract-{}-{index}.jsonl",
            std::process::id()
        ));
        let child = match Command::new(executable)
            .args(&case.args)
            .env("KUC_STORYBOOK_MOUSE_TRACE", trace)
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                let cleanup = wait_batch(std::mem::take(&mut batch));
                return Err(format!(
                    "{} spawn failed: {error}; prior batch cleanup: {cleanup:?}",
                    case.label
                )
                .into());
            }
        };
        batch.push(RunningCase { case, child });
        if batch.len() == MAX_PARALLEL_CASES {
            wait_batch(std::mem::take(&mut batch))?;
        }
    }
    wait_batch(batch)
}

#[cfg(target_os = "linux")]
fn basic_cases() -> Vec<ContractCase> {
    vec![
        ContractCase::new("default", 0, &[]),
        ContractCase::new("unknown option", 0, &["--unknown"]),
        ContractCase::new("runtime regression", 0, &["--runtime-regression"]),
        ContractCase::new("headless scenario", 0, &["--headless-scenario"]),
        ContractCase::new(
            "headless interaction audit",
            0,
            &["--headless-interaction-audit"],
        ),
        ContractCase::new(
            "preset window",
            0,
            &["--open-window", "1", "button", "--preset", "1"],
        ),
        ContractCase::new("modal window", 0, &["--open-modal-window", "1"]),
        ContractCase::new(
            "missing text artifact output",
            2,
            &["--text-surface-artifact"],
        ),
        ContractCase::new(
            "missing chrome artifact output",
            2,
            &["--command-chrome-artifact"],
        ),
        ContractCase::new(
            "missing root artifact output",
            2,
            &["--text-command-root-artifact"],
        ),
    ]
}

#[cfg(target_os = "linux")]
fn window_cases() -> Vec<ContractCase> {
    ["button", "text-area", "command-chrome", "text-command-root"]
        .into_iter()
        .map(|page| ContractCase::new(page, 0, &["--open-window", "1", page]))
        .collect()
}

#[cfg(target_os = "linux")]
fn blocking_cases(output_root: &Path) -> Result<Vec<ContractCase>, Box<dyn std::error::Error>> {
    [
        ("blocked text artifact", "--text-surface-artifact"),
        ("blocked chrome artifact", "--command-chrome-artifact"),
        ("blocked root artifact", "--text-command-root-artifact"),
        ("blocked visual snapshot", "--visual-snapshot"),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (label, flag))| {
        let blocking_file =
            output_root.join(format!("blocking-file-{}-{index}", std::process::id()));
        std::fs::write(&blocking_file, b"not a directory")?;
        let mut permissions = std::fs::metadata(&blocking_file)?.permissions();
        permissions.set_readonly(true);
        std::fs::set_permissions(&blocking_file, permissions)?;
        Ok(ContractCase::with_path(
            label,
            2,
            flag,
            &blocking_file,
            None,
        ))
    })
    .collect()
}

#[cfg(target_os = "linux")]
fn artifact_cases(output_root: &Path) -> Vec<ContractCase> {
    [
        ("text artifact", "--text-surface-artifact", "text-surface"),
        (
            "chrome artifact",
            "--command-chrome-artifact",
            "command-chrome",
        ),
        (
            "root artifact",
            "--text-command-root-artifact",
            "text-command-root",
        ),
    ]
    .into_iter()
    .map(|(label, flag, directory)| {
        let output = output_root.join(directory);
        ContractCase::with_path(label, 0, flag, &output, Some(output.clone()))
    })
    .collect()
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a Linux Xvfb display"]
fn native_xvfb_integration_covers_storybook_dependency_adapters()
-> Result<(), Box<dyn std::error::Error>> {
    let output_root =
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("native-window-contract-artifacts");
    std::fs::create_dir_all(&output_root)?;

    let mut cases = basic_cases();
    cases.extend(window_cases());
    cases.extend(blocking_cases(&output_root)?);
    cases.extend(artifact_cases(&output_root));
    run_cases(
        env!("CARGO_BIN_EXE_katana-ui-core-storybook"),
        &output_root,
        cases,
    )
}
