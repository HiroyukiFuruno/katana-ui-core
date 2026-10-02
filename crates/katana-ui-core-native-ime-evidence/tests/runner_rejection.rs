use katana_ui_core_native_ime_evidence::{NativeRunner, RunnerError, RunnerOptions};
use std::{path::PathBuf, time::Duration};

fn options(output: PathBuf) -> RunnerOptions {
    RunnerOptions {
        run_id: "fresh-run".into(),
        revision: "revision".into(),
        challenge: "challenge".into(),
        producer_id: "trusted-producer".into(),
        runner_id: "runner".into(),
        output,
        timeout: Duration::from_secs(1),
    }
}

#[test]
fn missing_binding_fails_before_creating_an_os_event_loop() {
    let directory = tempfile::tempdir().expect("directory");
    let path = directory.path().join("artifact.json");
    let mut configured = options(path.clone());
    configured.challenge.clear();
    assert!(matches!(
        NativeRunner::run(configured),
        Err(RunnerError::Core(_))
    ));
    assert!(!path.exists());
}

#[test]
fn existing_artifact_is_preserved_and_cannot_be_reused() {
    let directory = tempfile::tempdir().expect("directory");
    let path = directory.path().join("artifact.json");
    let previous_run = b"previous-run-evidence";
    std::fs::write(&path, previous_run).expect("previous artifact");
    assert!(matches!(
        NativeRunner::run(options(path.clone())),
        Err(RunnerError::Core(_))
    ));
    assert_eq!(
        std::fs::read(&path).expect("preserved artifact"),
        previous_run
    );
}

#[test]
fn zero_timeout_cannot_create_a_successful_evidence_file() {
    let directory = tempfile::tempdir().expect("directory");
    let path = directory.path().join("artifact.json");
    let mut configured = options(path.clone());
    configured.timeout = Duration::ZERO;
    assert!(matches!(
        NativeRunner::run(configured),
        Err(RunnerError::Core(_))
    ));
    assert!(!path.exists());
}
