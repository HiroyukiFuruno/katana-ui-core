#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a Linux Xvfb display"]
fn native_xvfb_integration_covers_storybook_dependency_adapters()
-> Result<(), Box<dyn std::error::Error>> {
    let executable = env!("CARGO_BIN_EXE_katana-ui-core-storybook");
    for args in [
        Vec::<&str>::new(),
        vec!["--unknown"],
        vec!["--runtime-regression"],
        vec!["--headless-scenario"],
        vec!["--headless-interaction-audit"],
        vec!["--open-window", "1", "button", "--preset", "1"],
        vec!["--open-modal-window", "1"],
    ] {
        let status = std::process::Command::new(executable).args(args).status()?;
        assert!(status.success(), "native Storybook command failed");
    }
    for command in [
        "--text-surface-artifact",
        "--command-chrome-artifact",
        "--text-command-root-artifact",
    ] {
        let missing_output = std::process::Command::new(executable)
            .arg(command)
            .status()?;
        assert_eq!(Some(2), missing_output.code());
    }
    for page in ["button", "text-area", "command-chrome", "text-command-root"] {
        let status = std::process::Command::new(executable)
            .args(["--open-window", "1", page])
            .status()?;
        assert!(status.success(), "native Storybook page failed: {page}");
    }
    let output_root =
        std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("native-window-contract-artifacts");
    std::fs::create_dir_all(&output_root)?;
    let blocking_file = output_root.join("blocking-file");
    std::fs::write(&blocking_file, b"not a directory")?;
    for command in [
        "--text-surface-artifact",
        "--command-chrome-artifact",
        "--text-command-root-artifact",
        "--visual-snapshot",
    ] {
        let failed = std::process::Command::new(executable)
            .arg(command)
            .arg(&blocking_file)
            .status()?;
        assert_eq!(Some(2), failed.code(), "expected failure for {command}");
    }
    for (flag, directory) in [
        ("--text-surface-artifact", "text-surface"),
        ("--command-chrome-artifact", "command-chrome"),
        ("--text-command-root-artifact", "text-command-root"),
    ] {
        let output_dir = output_root.join(directory);
        let status = std::process::Command::new(executable)
            .arg(flag)
            .arg(&output_dir)
            .status()?;
        assert!(status.success(), "native Storybook artifact failed: {flag}");
        assert!(output_dir.is_dir(), "artifact directory missing: {flag}");
    }
    Ok(())
}
