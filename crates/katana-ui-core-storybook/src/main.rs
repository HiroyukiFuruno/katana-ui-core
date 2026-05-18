mod snapshot_output;

use katana_ui_core_storybook::{StoryCatalog, StorybookPanel, StorybookSummary, StorybookVisual};
use snapshot_output::SnapshotOutput;
use std::path::Path;
use std::{env, fs, process};

const DEFAULT_WINDOW_FRAMES: usize = 0;
const SNAPSHOT_PAGE_ARG: usize = 3;
const SNAPSHOT_THEME_ARG: usize = 4;
const SNAPSHOT_OPERATION_ARG: usize = 5;
const SNAPSHOT_SCROLL_ARG: usize = 6;
const SNAPSHOT_SCROLLBAR_ARG: usize = 7;
const DEFAULT_PRESET_INDEX: usize = 0;
const DEFAULT_SCROLL_Y: usize = 0;
const INTERACTIVE_PRESET_INDEX: usize = 1;
const EDGE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1).map(String::as_str) {
        match command {
            "--visual-snapshot" => save_snapshot(&args),
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

fn snapshot_preset_index(value: &str) -> usize {
    match value {
        "operation" | "interactive" | "preset-1" => INTERACTIVE_PRESET_INDEX,
        "edge" | "preset-2" => EDGE_PRESET_INDEX,
        "theme" | "preset-3" => THEME_PRESET_INDEX,
        _ => DEFAULT_PRESET_INDEX,
    }
}

fn snapshot_scroll_y(value: &str) -> usize {
    value.parse::<usize>().ok().unwrap_or(DEFAULT_SCROLL_Y)
}

fn snapshot_scrollbar_visible(value: &str) -> bool {
    !matches!(
        value,
        "false" | "hidden" | "hide-scrollbar" | "scrollbar-off" | "off"
    )
}

fn save_snapshot(args: &[String]) {
    let output = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("target/storybook-panel.png");
    let selected_page = args
        .get(SNAPSHOT_PAGE_ARG)
        .map(String::as_str)
        .unwrap_or("button");
    let theme_id = args
        .get(SNAPSHOT_THEME_ARG)
        .map(String::as_str)
        .unwrap_or("dark");
    let preset_index = args
        .get(SNAPSHOT_OPERATION_ARG)
        .map(String::as_str)
        .map(snapshot_preset_index)
        .unwrap_or(DEFAULT_PRESET_INDEX);
    let scroll_y = args
        .get(SNAPSHOT_SCROLL_ARG)
        .map(String::as_str)
        .map(snapshot_scroll_y)
        .unwrap_or(DEFAULT_SCROLL_Y);
    let scrollbar_visible = args
        .get(SNAPSHOT_SCROLLBAR_ARG)
        .map(String::as_str)
        .map(snapshot_scrollbar_visible)
        .unwrap_or(true);
    let output_path = Path::new(output);
    prepare_or_exit(output_path, "failed to prepare visual snapshot");
    if let Err(error) = StorybookVisual.save_preset_scrolled_png_with_scrollbar(
        output_path,
        theme_id,
        selected_page,
        preset_index,
        scroll_y,
        scrollbar_visible,
    ) {
        eprintln!("failed to write visual snapshot: {error}");
        process::exit(2);
    }
    let evidence = snapshot_evidence_or_exit(output_path, "failed to inspect visual snapshot");
    println!("katana-ui-core-storybook-snapshot: {output} {evidence}");
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
    save_scenario_png("target/storybook-panel-light.png", "light", "button", false);
    save_scenario_png("target/storybook-panel-dark.png", "dark", "button", false);
    save_scenario_png(
        "target/storybook-panel-after-operation.png",
        "dark",
        "button",
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
    prepare_or_exit(output_path, "failed to prepare modal snapshot");
    if let Err(error) = StorybookVisual.save_modal_png(output_path) {
        eprintln!("failed to write modal snapshot: {error}");
        process::exit(2);
    }
}

fn save_scenario_png(path: &str, theme_id: &str, selected_page: &str, operation: bool) {
    let output_path = Path::new(path);
    prepare_or_exit(output_path, "failed to prepare scenario snapshot");
    if let Err(error) =
        StorybookVisual.save_scenario_png(output_path, theme_id, selected_page, operation)
    {
        eprintln!("failed to write scenario snapshot: {error}");
        process::exit(2);
    }
}

fn prepare_or_exit(path: &Path, failure: &str) {
    if let Err(error) = SnapshotOutput::prepare(path) {
        eprintln!("{failure}: {error}");
        process::exit(2);
    }
}

fn snapshot_evidence_or_exit(path: &Path, failure: &str) -> String {
    match SnapshotOutput::evidence(path) {
        Ok(evidence) => evidence,
        Err(error) => {
            eprintln!("{failure}: {error}");
            process::exit(2);
        }
    }
}
