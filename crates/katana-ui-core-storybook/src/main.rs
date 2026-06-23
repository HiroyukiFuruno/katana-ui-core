mod snapshot_command;
mod snapshot_output;

use katana_ui_core_storybook::{
    DEFAULT_STORYBOOK_PAGE, StoryCatalog, StorybookPanel, StorybookRoutes, StorybookSummary,
    StorybookVisual,
};
use snapshot_command::SnapshotCommand;
use std::path::Path;
use std::{env, fs, process};

const DEFAULT_WINDOW_FRAMES: usize = 0;
const WINDOW_FIRST_ARG_INDEX: usize = 2;
const PRESET_ARG: &str = "--preset";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpenWindowRequest {
    frames: usize,
    page: Option<&'static str>,
    preset_index: Option<usize>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1).map(String::as_str) {
        match command {
            "--visual-snapshot" => SnapshotCommand::save_snapshot(&args),
            "--open-window" => open_window(&args),
            "--open-modal-window" => open_modal_window(&args),
            "--runtime-regression" => print_runtime_regression(),
            "--headless-scenario" => run_headless_scenario(),
            "--headless-interaction-audit" => run_headless_interaction_audit(),
            _ => print_summary(),
        }
        return;
    }
    print_summary();
}

fn print_summary() {
    println!("katana-ui-core-storybook: {}", StorybookSummary.render());
}

fn open_window(args: &[String]) {
    let request = open_window_request(args);
    let page = request.page.unwrap_or(DEFAULT_STORYBOOK_PAGE);
    let result =
        StorybookVisual.open_window_for_page_and_preset(request.frames, page, request.preset_index);
    if let Err(error) = result {
        eprintln!("failed to open storybook window: {error}");
        process::exit(2);
    }
    println!(
        "katana-ui-core-storybook-window: frames={} page={}",
        request.frames, page
    );
}

fn open_window_request(args: &[String]) -> OpenWindowRequest {
    let mut preset_index = None;
    let mut position = WINDOW_FIRST_ARG_INDEX;
    let Some(first_arg) = args.get(WINDOW_FIRST_ARG_INDEX) else {
        return OpenWindowRequest {
            frames: DEFAULT_WINDOW_FRAMES,
            page: None,
            preset_index: None,
        };
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
                Some(resolve_storybook_page_or_exit(first_arg)),
            )
        }
    };
    while let Some(arg) = args.get(position) {
        if arg == PRESET_ARG {
            let Some(value) = args.get(position + 1) else {
                eprintln!("missing value for --preset");
                process::exit(2);
            };
            preset_index = Some(parse_preset_index_or_exit(value));
            position += 2;
            continue;
        }
        if page.is_none() {
            page = Some(resolve_storybook_page_or_exit(arg));
            position += 1;
            continue;
        }
        eprintln!("unexpected --open-window argument: {arg}");
        process::exit(2);
    }
    OpenWindowRequest {
        frames,
        page,
        preset_index,
    }
}

fn resolve_storybook_page_or_exit(value: &str) -> &'static str {
    resolve_storybook_page(value).unwrap_or_else(|| {
        eprintln!("unknown Storybook page for --open-window: {value}");
        process::exit(2);
    })
}

fn resolve_storybook_page(value: &str) -> Option<&'static str> {
    StorybookRoutes
        .default_routes()
        .into_iter()
        .find(|route| route.page == value)
        .map(|route| route.page)
}

fn parse_preset_index_or_exit(value: &str) -> usize {
    value.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("invalid --preset value: {value}");
        process::exit(2);
    })
}

fn open_modal_window(args: &[String]) {
    let frames = args
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_WINDOW_FRAMES);
    match StorybookVisual.open_modal_window(frames) {
        Ok(report) => println!(
            "katana-ui-core-storybook-modal-window: {}",
            report.summary()
        ),
        Err(error) => {
            eprintln!("failed to open storybook modal window: {error}");
            process::exit(2);
        }
    }
}

fn print_runtime_regression() {
    println!(
        "katana-ui-core-storybook-runtime: {}",
        StorybookVisual.runtime_report().summary()
    );
}

fn run_headless_scenario() {
    let catalog = StoryCatalog;
    let examples = catalog.examples();
    let panel_report = StorybookPanel::interaction_report(&examples);
    let visual_report = StorybookVisual.coverage_report();
    write_json(
        Path::new("target/storybook-panel-interaction-report.json"),
        &panel_report,
        "failed to write panel interaction report",
    );
    write_json(
        Path::new("target/storybook-visual-coverage.json"),
        &visual_report,
        "failed to write visual coverage report",
    );
    save_scenario_png(
        "target/storybook-panel-light.png",
        "light",
        DEFAULT_STORYBOOK_PAGE,
        false,
    );
    save_scenario_png(
        "target/storybook-panel-dark.png",
        "dark",
        DEFAULT_STORYBOOK_PAGE,
        false,
    );
    save_scenario_png(
        "target/storybook-panel-after-operation.png",
        "dark",
        DEFAULT_STORYBOOK_PAGE,
        true,
    );
    save_modal_png("target/storybook-panel-modal-window.png");
    println!(
        "katana-ui-core-storybook-headless: {} {} {}",
        StorybookSummary.render(),
        panel_report.summary(),
        visual_report.summary()
    );
}

fn run_headless_interaction_audit() {
    let report = StorybookVisual.live_interaction_audit_report();
    write_json(
        Path::new("target/storybook-live-interaction-audit.json"),
        &report,
        "failed to write live interaction audit report",
    );
    println!(
        "katana-ui-core-storybook-live-interaction: {}",
        report.summary()
    );
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T, failure: &str) {
    let Some(parent) = path.parent() else {
        eprintln!("{failure}: missing parent directory");
        process::exit(2);
    };
    if let Err(error) = fs::create_dir_all(parent) {
        eprintln!("{failure}: {error}");
        process::exit(2);
    }
    let json = match serde_json::to_string_pretty(value) {
        Ok(it) => it,
        Err(error) => {
            eprintln!("{failure}: {error}");
            process::exit(2);
        }
    };
    if let Err(error) = fs::write(path, json) {
        eprintln!("{failure}: {error}");
        process::exit(2);
    }
}

fn save_modal_png(path: &str) {
    let output_path = Path::new(path);
    SnapshotCommand::prepare_or_exit(output_path, "failed to prepare modal snapshot");
    if let Err(error) = StorybookVisual.save_modal_png(output_path) {
        eprintln!("failed to write modal snapshot: {error}");
        process::exit(2);
    }
}

fn save_scenario_png(path: &str, theme_id: &str, selected_page: &str, operation: bool) {
    let output_path = Path::new(path);
    SnapshotCommand::prepare_or_exit(output_path, "failed to prepare scenario snapshot");
    if let Err(error) =
        StorybookVisual.save_scenario_png(output_path, theme_id, selected_page, operation)
    {
        eprintln!("failed to write scenario snapshot: {error}");
        process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_WINDOW_FRAMES, open_window_request, resolve_storybook_page};

    #[test]
    fn open_window_request_accepts_frame_count_and_page() {
        let args = args(&["bin", "--open-window", "12", "progress-bar"]);
        let request = open_window_request(&args);

        assert_eq!(12, request.frames);
        assert_eq!(Some("progress-bar"), request.page);
        assert_eq!(None, request.preset_index);
    }

    #[test]
    fn open_window_request_accepts_page_without_frame_count() {
        let args = args(&["bin", "--open-window", "progress-bar"]);
        let request = open_window_request(&args);

        assert_eq!(DEFAULT_WINDOW_FRAMES, request.frames);
        assert_eq!(Some("progress-bar"), request.page);
        assert_eq!(None, request.preset_index);
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

        assert_eq!(12, request.frames);
        assert_eq!(Some("progress-bar"), request.page);
        assert_eq!(Some(4), request.preset_index);
    }

    #[test]
    fn open_window_request_accepts_preset_without_frame_count() {
        let args = args(&["bin", "--open-window", "progress-bar", "--preset", "4"]);
        let request = open_window_request(&args);

        assert_eq!(DEFAULT_WINDOW_FRAMES, request.frames);
        assert_eq!(Some("progress-bar"), request.page);
        assert_eq!(Some(4), request.preset_index);
    }

    #[test]
    fn open_window_request_keeps_default_frames_without_page() {
        let args = args(&["bin", "--open-window"]);
        let request = open_window_request(&args);

        assert_eq!(DEFAULT_WINDOW_FRAMES, request.frames);
        assert_eq!(None, request.page);
        assert_eq!(None, request.preset_index);
    }

    #[test]
    fn resolve_storybook_page_rejects_unknown_page_without_defaulting() {
        assert_eq!(Some("progress-bar"), resolve_storybook_page("progress-bar"));
        assert_eq!(None, resolve_storybook_page("progress"));
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }
}
