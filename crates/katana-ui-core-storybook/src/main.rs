mod headless_command;
mod snapshot_command;
mod snapshot_output;

#[cfg(test)]
use headless_command::{modal_snapshot_error, run_steps, scenario_snapshot_error, write_json};
use headless_command::{run_headless_interaction_audit, run_headless_scenario};
use katana_ui_core_storybook::{
    DEFAULT_STORYBOOK_PAGE, StorybookRoutes, StorybookSummary, StorybookVisual,
    StorybookVisualError,
};
use snapshot_command::SnapshotCommand;
use std::env;
use std::process::ExitCode;

const DEFAULT_WINDOW_FRAMES: usize = 0;
const WINDOW_FIRST_ARG_INDEX: usize = 2;
const PRESET_ARG: &str = "--preset";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpenWindowRequest {
    frames: usize,
    page: Option<&'static str>,
    preset_index: Option<usize>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    exit_code(run(&args))
}

fn exit_code(result: Result<(), String>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    run_with(args, open_window, open_modal_window)
}

fn run_with(
    args: &[String],
    open_window_command: impl FnOnce(&[String]) -> Result<(), String>,
    open_modal_command: impl FnOnce(&[String]) -> Result<(), String>,
) -> Result<(), String> {
    if let Some(command) = args.get(1).map(String::as_str) {
        match command {
            "--visual-snapshot" => SnapshotCommand::save_snapshot(args)?,
            "--open-window" => open_window_command(args)?,
            "--open-modal-window" => open_modal_command(args)?,
            "--runtime-regression" => print_runtime_regression(),
            "--headless-scenario" => run_headless_scenario()?,
            "--headless-interaction-audit" => run_headless_interaction_audit()?,
            _ => print_summary(),
        }
        return Ok(());
    }
    print_summary();
    Ok(())
}

fn print_summary() {
    println!("katana-ui-core-storybook: {}", StorybookSummary.render());
}

fn open_window(args: &[String]) -> Result<(), String> {
    open_window_with(args, |frames, page, preset_index| {
        StorybookVisual
            .open_window_for_page_and_preset(frames, page, preset_index)
            .map_err(minifb_error)
    })
}

fn open_window_with(
    args: &[String],
    opener: impl FnOnce(usize, &'static str, Option<usize>) -> Result<(), String>,
) -> Result<(), String> {
    let request = open_window_request(args)?;
    let page = request.page.unwrap_or(DEFAULT_STORYBOOK_PAGE);
    opener(request.frames, page, request.preset_index)
        .map_err(|error| format!("failed to open storybook window: {error}"))?;
    println!(
        "katana-ui-core-storybook-window: frames={} page={}",
        request.frames, page
    );
    Ok(())
}

fn open_window_request(args: &[String]) -> Result<OpenWindowRequest, String> {
    let mut preset_index = None;
    let mut position = WINDOW_FIRST_ARG_INDEX;
    let Some(first_arg) = args.get(WINDOW_FIRST_ARG_INDEX) else {
        return Ok(OpenWindowRequest {
            frames: DEFAULT_WINDOW_FRAMES,
            page: None,
            preset_index: None,
        });
    };
    let (frames, mut page) = match first_arg.parse::<usize>() {
        Ok(frames) => {
            position += 1;
            (frames, None)
        }
        Err(_) if first_arg == PRESET_ARG => (DEFAULT_WINDOW_FRAMES, None),
        Err(_) => {
            position += 1;
            (
                DEFAULT_WINDOW_FRAMES,
                Some(resolve_storybook_page(first_arg).ok_or(unknown_storybook_page(first_arg))?),
            )
        }
    };
    while let Some(arg) = args.get(position) {
        if arg == PRESET_ARG {
            let Some(value) = args.get(position + 1) else {
                return Err("missing value for --preset".to_string());
            };
            preset_index = Some(
                value
                    .parse::<usize>()
                    .map_err(|_| format!("invalid --preset value: {value}"))?,
            );
            position += 2;
            continue;
        }
        if page.is_none() {
            page = Some(resolve_storybook_page(arg).ok_or(unknown_storybook_page(arg))?);
            position += 1;
            continue;
        }
        return Err(format!("unexpected --open-window argument: {arg}"));
    }
    Ok(OpenWindowRequest {
        frames,
        page,
        preset_index,
    })
}

fn unknown_storybook_page(value: &str) -> String {
    format!("unknown Storybook page for --open-window: {value}")
}

fn storybook_visual_error(error: StorybookVisualError) -> String {
    error.to_string()
}

fn minifb_error(error: minifb::Error) -> String {
    error.to_string()
}

fn resolve_storybook_page(value: &str) -> Option<&'static str> {
    StorybookRoutes
        .default_routes()
        .into_iter()
        .find(|route| route.page == value)
        .map(|route| route.page)
}

fn open_modal_window(args: &[String]) -> Result<(), String> {
    open_modal_window_with(args, |frames| {
        StorybookVisual
            .open_modal_window(frames)
            .map_err(storybook_visual_error)
    })
}

fn open_modal_window_with(
    args: &[String],
    opener: impl FnOnce(usize) -> Result<katana_ui_core_storybook::StorybookWindowRun, String>,
) -> Result<(), String> {
    let frames = args
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_WINDOW_FRAMES);
    let report = opener(frames)
        .map_err(|error| format!("failed to open storybook modal window: {error}"))?;
    println!(
        "katana-ui-core-storybook-modal-window: {}",
        report.summary()
    );
    Ok(())
}

fn print_runtime_regression() {
    println!(
        "katana-ui-core-storybook-runtime: {}",
        StorybookVisual.runtime_report().summary()
    );
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_WINDOW_FRAMES, exit_code, main as cli_main, minifb_error, modal_snapshot_error,
        open_modal_window_with, open_window_request, open_window_with, resolve_storybook_page, run,
        run_steps, run_with, scenario_snapshot_error, storybook_visual_error, write_json,
    };
    #[cfg(target_os = "linux")]
    use super::{open_modal_window, open_window};
    use katana_ui_core_storybook::StorybookWindowRun;
    use serde::ser::{Error as _, Serialize, Serializer};
    use std::cell::Cell;
    use std::error::Error;
    use std::io;
    use std::process::ExitCode;
    use std::{env, fs, process};

    #[test]
    fn open_window_request_accepts_frame_count_and_page() {
        let args = args(&["bin", "--open-window", "12", "progress-bar"]);
        let request = open_window_request(&args);
        assert!(matches!(
            request,
            Ok(request)
                if request.frames == 12
                    && request.page == Some("progress-bar")
                    && request.preset_index.is_none()
        ));
    }

    #[test]
    fn open_window_request_accepts_page_without_frame_count() {
        let args = args(&["bin", "--open-window", "progress-bar"]);
        let request = open_window_request(&args);
        assert!(matches!(
            request,
            Ok(request)
                if request.frames == DEFAULT_WINDOW_FRAMES
                    && request.page == Some("progress-bar")
                    && request.preset_index.is_none()
        ));
    }

    #[test]
    fn open_window_request_accepts_page_frame_count_and_preset() {
        let args = args(&[
            "bin",
            "--open-window",
            "12",
            "progress-bar",
            "--preset",
            "4",
        ]);
        let request = open_window_request(&args);
        assert!(matches!(
            request,
            Ok(request)
                if request.frames == 12
                    && request.page == Some("progress-bar")
                    && request.preset_index == Some(4)
        ));
    }

    #[test]
    fn open_window_request_accepts_preset_without_frame_count() {
        let args = args(&["bin", "--open-window", "progress-bar", "--preset", "4"]);
        let request = open_window_request(&args);
        assert!(matches!(
            request,
            Ok(request)
                if request.frames == DEFAULT_WINDOW_FRAMES
                    && request.page == Some("progress-bar")
                    && request.preset_index == Some(4)
        ));
    }

    #[test]
    fn open_window_request_keeps_default_frames_without_page() {
        let args = args(&["bin", "--open-window"]);
        let request = open_window_request(&args);
        assert!(matches!(
            request,
            Ok(request)
                if request.frames == DEFAULT_WINDOW_FRAMES
                    && request.page.is_none()
                    && request.preset_index.is_none()
        ));
    }

    #[test]
    fn resolve_storybook_page_rejects_unknown_page_without_defaulting() {
        assert_eq!(Some("progress-bar"), resolve_storybook_page("progress-bar"));
        assert_eq!(None, resolve_storybook_page("progress"));
    }

    #[test]
    fn open_window_request_rejects_invalid_page_preset_and_extra_arguments() {
        for (values, expected) in [
            (
                vec!["bin", "--open-window", "unknown"],
                "unknown Storybook page",
            ),
            (
                vec!["bin", "--open-window", "1", "unknown"],
                "unknown Storybook page",
            ),
            (
                vec!["bin", "--open-window", "--preset"],
                "missing value for --preset",
            ),
            (
                vec!["bin", "--open-window", "--preset", "bad"],
                "invalid --preset value",
            ),
            (
                vec!["bin", "--open-window", "button", "extra"],
                "unexpected --open-window argument",
            ),
        ] {
            let result = open_window_request(&args(&values));
            assert!(matches!(result, Err(error) if error.contains(expected)));
        }
    }

    #[test]
    fn external_error_mappers_preserve_operation_context() {
        assert_eq!(
            "Failed to create window",
            storybook_visual_error(katana_ui_core_storybook::StorybookVisualError::from(
                minifb::Error::WindowCreate("test".to_string())
            ))
        );
        assert_eq!(
            "Failed to create window",
            minifb_error(minifb::Error::WindowCreate("test".to_string()))
        );
        assert!(
            modal_snapshot_error(image::ImageError::IoError(io::Error::other("modal failed")))
                .starts_with("failed to write modal snapshot:")
        );
        assert!(
            scenario_snapshot_error(image::ImageError::IoError(io::Error::other(
                "scenario failed"
            )))
            .starts_with("failed to write scenario snapshot:")
        );
    }

    #[test]
    fn open_window_and_modal_commands_are_headless_through_injected_openers() {
        let window_args = args(&["bin", "--open-window", "3", "button", "--preset", "2"]);
        assert!(
            open_window_with(&window_args, |frames, page, preset| {
                assert_eq!((3, "button", Some(2)), (frames, page, preset));
                Ok(())
            })
            .is_ok()
        );
        assert_eq!(
            Err("failed to open storybook window: unavailable".to_string()),
            open_window_with(&window_args, |_, _, _| Err("unavailable".to_string()))
        );

        let modal_args = args(&["bin", "--open-modal-window", "4"]);
        assert!(
            open_modal_window_with(&modal_args, |frames| {
                assert_eq!(4, frames);
                Ok(window_run(frames))
            })
            .is_ok()
        );
        assert_eq!(
            Err("failed to open storybook modal window: unavailable".to_string()),
            open_modal_window_with(&modal_args, |_| Err("unavailable".to_string()))
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[ignore = "requires an X11 display such as the strict coverage Xvfb session"]
    fn native_xvfb_cli_wrappers_open_main_and_modal_windows() -> Result<(), String> {
        open_window(&args(&["bin", "--open-window", "1", "button"]))?;
        open_modal_window(&args(&["bin", "--open-modal-window", "1"]))
    }

    #[test]
    fn command_dispatch_covers_window_modal_summary_and_errors() {
        for values in [
            vec!["bin"],
            vec!["bin", "--unknown"],
            vec!["bin", "--runtime-regression"],
        ] {
            assert!(run_with(&args(&values), ok_window_command, ok_modal_command).is_ok());
        }
        assert!(
            run_with(
                &args(&["bin", "--open-window"]),
                ok_window_command,
                ok_modal_command
            )
            .is_ok()
        );
        assert!(
            run_with(
                &args(&["bin", "--open-modal-window"]),
                ok_window_command,
                ok_modal_command
            )
            .is_ok()
        );
        assert_eq!(
            Err("window failed".to_string()),
            run_with(
                &args(&["bin", "--open-window"]),
                failing_window_command,
                ok_modal_command
            )
        );
        assert!(run(&args(&["bin"])).is_ok());
        assert_eq!(ExitCode::SUCCESS, cli_main());
        assert_eq!(ExitCode::SUCCESS, exit_code(Ok(())));
        assert_eq!(
            ExitCode::from(2),
            exit_code(Err("expected failure".to_string()))
        );
    }

    #[test]
    fn command_dispatch_runs_headless_scenario_and_interaction_audit() {
        for command in ["--headless-scenario", "--headless-interaction-audit"] {
            assert!(
                run_with(
                    &args(&["bin", command]),
                    ok_window_command,
                    ok_modal_command
                )
                .is_ok()
            );
        }
    }

    #[test]
    fn headless_step_runner_stops_at_first_error() {
        let success_count = Cell::new(0);
        let mut steps: [Box<dyn FnMut() -> Result<(), String> + '_>; 3] = [
            Box::new(|| Ok(())),
            Box::new(|| Err("step failed".to_string())),
            Box::new(|| {
                success_count.set(success_count.get() + 1);
                Ok(())
            }),
        ];

        assert_eq!(Err("step failed".to_string()), run_steps(&mut steps));
        assert_eq!(0, success_count.get());
        assert!(steps[2]().is_ok());
        assert_eq!(1, success_count.get());

        let mut success_steps: [Box<dyn FnMut() -> Result<(), String>>; 1] = [Box::new(|| Ok(()))];
        assert!(run_steps(&mut success_steps).is_ok());
    }

    #[test]
    fn write_json_covers_success_and_contextual_failures() -> Result<(), Box<dyn Error>> {
        let directory = env::temp_dir().join(format!("kuc-storybook-json-{}", process::id()));
        let path = directory.join("report.json");
        write_json(&path, &vec!["ok"], "write failed")?;
        assert!(fs::read_to_string(&path)?.contains("ok"));

        let serialization_error = write_json(&path, &FailingSerialize, "serialize failed");
        assert!(matches!(
            serialization_error,
            Err(error) if error.starts_with("serialize failed:")
        ));

        let missing_parent_error =
            write_json(std::path::Path::new("/"), &vec!["value"], "parent failed");
        assert_eq!(
            Err("parent failed: missing parent directory".to_string()),
            missing_parent_error
        );

        let blocking_file = directory.join("blocking");
        fs::write(&blocking_file, b"file")?;
        let create_error = write_json(
            &blocking_file.join("report.json"),
            &vec!["value"],
            "create failed",
        );
        assert!(matches!(
            create_error,
            Err(error) if error.starts_with("create failed:")
        ));

        let write_error = write_json(&directory, &vec!["value"], "write failed");
        assert!(matches!(
            write_error,
            Err(error) if error.starts_with("write failed:")
        ));
        fs::remove_dir_all(directory)?;
        Ok(())
    }

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(S::Error::custom("intentional serialization failure"))
        }
    }

    fn window_run(frames: usize) -> StorybookWindowRun {
        StorybookWindowRun {
            frames,
            modal_window_opened: true,
            same_display: true,
            frontmost: true,
            state_reflected: true,
            overlay_rendered: true,
        }
    }

    fn ok_window_command(_args: &[String]) -> Result<(), String> {
        Ok(())
    }

    fn failing_window_command(_args: &[String]) -> Result<(), String> {
        Err("window failed".to_string())
    }

    fn ok_modal_command(_args: &[String]) -> Result<(), String> {
        Ok(())
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }
}
