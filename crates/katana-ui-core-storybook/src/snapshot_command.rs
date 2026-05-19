use super::snapshot_output::SnapshotOutput;
use katana_ui_core_storybook::StorybookVisual;
use std::path::Path;
use std::process;

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

pub(crate) struct SnapshotCommand;

impl SnapshotCommand {
    pub(crate) fn save_snapshot(args: &[String]) {
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
        let clicked = args
            .get(SNAPSHOT_OPERATION_ARG)
            .map(String::as_str)
            .is_some_and(snapshot_clicked);
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
        Self::write_snapshot(
            output,
            theme_id,
            selected_page,
            preset_index,
            scroll_y,
            scrollbar_visible,
            clicked,
        );
    }

    pub(crate) fn prepare_or_exit(path: &Path, failure: &str) {
        if let Err(error) = SnapshotOutput::prepare(path) {
            eprintln!("{failure}: {error}");
            process::exit(2);
        }
    }

    fn write_snapshot(
        output: &str,
        theme_id: &str,
        selected_page: &str,
        preset_index: usize,
        scroll_y: usize,
        scrollbar_visible: bool,
        clicked: bool,
    ) {
        let output_path = Path::new(output);
        Self::prepare_or_exit(output_path, "failed to prepare visual snapshot");
        let result = if clicked {
            StorybookVisual.save_clicked_preset_scrolled_png_with_scrollbar(
                output_path,
                theme_id,
                selected_page,
                preset_index,
                scroll_y,
                scrollbar_visible,
            )
        } else {
            StorybookVisual.save_preset_scrolled_png_with_scrollbar(
                output_path,
                theme_id,
                selected_page,
                preset_index,
                scroll_y,
                scrollbar_visible,
            )
        };
        if let Err(error) = result {
            eprintln!("failed to write visual snapshot: {error}");
            process::exit(2);
        }
        let evidence = snapshot_evidence_or_exit(output_path, "failed to inspect visual snapshot");
        println!("katana-ui-core-storybook-snapshot: {output} {evidence}");
    }
}

fn snapshot_preset_index(value: &str) -> usize {
    match value {
        "classic" | "operation" | "interactive" | "preset-1" => INTERACTIVE_PRESET_INDEX,
        "basic" | "edge" | "preset-2" => EDGE_PRESET_INDEX,
        "dense" | "theme" | "preset-3" => THEME_PRESET_INDEX,
        _ => DEFAULT_PRESET_INDEX,
    }
}

fn snapshot_clicked(value: &str) -> bool {
    matches!(value, "clicked" | "press" | "pressed")
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

fn snapshot_evidence_or_exit(path: &Path, failure: &str) -> String {
    match SnapshotOutput::evidence(path) {
        Ok(evidence) => evidence,
        Err(error) => {
            eprintln!("{failure}: {error}");
            process::exit(2);
        }
    }
}
