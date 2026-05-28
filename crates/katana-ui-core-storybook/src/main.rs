mod snapshot_command;
mod snapshot_output;

use katana_ui_core_storybook::{
    DEFAULT_STORYBOOK_PAGE, StoryCatalog, StorybookPanel, StorybookSummary, StorybookVisual,
};
use snapshot_command::SnapshotCommand;
use std::path::Path;
use std::{env, fs, process};

const DEFAULT_WINDOW_FRAMES: usize = 0;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1).map(String::as_str) {
        match command {
            "--visual-snapshot" => SnapshotCommand::save_snapshot(&args),
            "--open-window" => open_window(&args),
            "--open-modal-window" => open_modal_window(&args),
            "--runtime-regression" => print_runtime_regression(),
            "--headless-scenario" => run_headless_scenario(),
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
    let frames = args
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_WINDOW_FRAMES);
    if let Err(error) = StorybookVisual.open_window(frames) {
        eprintln!("failed to open storybook window: {error}");
        process::exit(2);
    }
    println!("katana-ui-core-storybook-window: frames={frames}");
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
