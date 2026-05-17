use katana_ui_core_storybook::{StorybookSummary, StorybookVisual};
use std::path::Path;
use std::{env, process};

const DEFAULT_WINDOW_FRAMES: usize = 120;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1).map(String::as_str) {
        match command {
            "--visual-snapshot" => save_snapshot(&args),
            "--open-window" => open_window(&args),
            "--open-modal-window" => open_modal_window(&args),
            "--runtime-regression" => print_runtime_regression(),
            _ => print_summary(),
        }
        return;
    }
    print_summary();
}

fn print_summary() {
    println!("katana-ui-core-storybook: {}", StorybookSummary.render());
}

fn save_snapshot(args: &[String]) {
    let output = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("target/storybook-panel.png");
    if let Err(error) = StorybookVisual.save_png(Path::new(output)) {
        eprintln!("failed to write visual snapshot: {error}");
        process::exit(2);
    }
    println!("katana-ui-core-storybook-snapshot: {output}");
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
