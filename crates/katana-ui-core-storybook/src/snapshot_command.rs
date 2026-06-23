use super::snapshot_output::SnapshotOutput;
use katana_ui_core_storybook::{DEFAULT_STORYBOOK_PAGE, StorybookVisual};
use std::path::Path;
use std::process;

const SNAPSHOT_PAGE_ARG: usize = 3;
const SNAPSHOT_THEME_ARG: usize = 4;
const SNAPSHOT_OPERATION_ARG: usize = 5;
const SNAPSHOT_ACTION_ARG: usize = 6;
const SNAPSHOT_SCROLL_ARG: usize = 7;
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
            .unwrap_or(DEFAULT_STORYBOOK_PAGE);
        let theme_id = args
            .get(SNAPSHOT_THEME_ARG)
            .map(String::as_str)
            .unwrap_or("dark");
        let request = snapshot_request(args);
        Self::write_snapshot(
            output,
            theme_id,
            selected_page,
            request.preset_index,
            request.scroll_y,
            request.scrollbar_visible,
            request.clicked,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SnapshotRequest {
    preset_index: usize,
    clicked: bool,
    scroll_y: usize,
    scrollbar_visible: bool,
}

fn snapshot_request(args: &[String]) -> SnapshotRequest {
    let operation = args.get(SNAPSHOT_OPERATION_ARG).map(String::as_str);
    let action = args.get(SNAPSHOT_ACTION_ARG).map(String::as_str);
    let clicked = operation.is_some_and(snapshot_clicked) || action.is_some_and(snapshot_clicked);
    let preset_index = operation
        .filter(|value| !snapshot_clicked(value))
        .map(snapshot_preset_index)
        .unwrap_or(DEFAULT_PRESET_INDEX);
    let scroll_arg = if action.is_some_and(snapshot_clicked) {
        SNAPSHOT_SCROLL_ARG
    } else {
        SNAPSHOT_ACTION_ARG
    };
    let scrollbar_arg = scroll_arg + 1;
    let scroll_y = args
        .get(scroll_arg)
        .map(String::as_str)
        .filter(|value| !snapshot_clicked(value))
        .map(snapshot_scroll_y)
        .unwrap_or(DEFAULT_SCROLL_Y);
    let scrollbar_visible = args
        .get(scrollbar_arg)
        .map(String::as_str)
        .map(snapshot_scrollbar_visible)
        .unwrap_or(true);
    SnapshotRequest {
        preset_index,
        clicked,
        scroll_y,
        scrollbar_visible,
    }
}

fn snapshot_preset_index(value: &str) -> usize {
    if let Some(index) = snapshot_numeric_preset_index(value) {
        return index;
    }
    match value {
        "classic" | "operation" | "interactive" | "preset-1" => INTERACTIVE_PRESET_INDEX,
        "basic" | "edge" | "preset-2" => EDGE_PRESET_INDEX,
        "dense" | "theme" | "preset-3" => THEME_PRESET_INDEX,
        _ => DEFAULT_PRESET_INDEX,
    }
}

fn snapshot_numeric_preset_index(value: &str) -> Option<usize> {
    value
        .strip_prefix("preset-")
        .unwrap_or(value)
        .parse::<usize>()
        .ok()
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

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_PRESET_INDEX, EDGE_PRESET_INDEX, INTERACTIVE_PRESET_INDEX, SnapshotRequest,
        snapshot_preset_index, snapshot_request,
    };

    #[test]
    fn snapshot_preset_index_keeps_named_aliases() {
        assert_eq!(
            INTERACTIVE_PRESET_INDEX,
            snapshot_preset_index("interactive")
        );
        assert_eq!(EDGE_PRESET_INDEX, snapshot_preset_index("edge"));
    }

    #[test]
    fn snapshot_preset_index_accepts_numeric_presets() {
        assert_eq!(5, snapshot_preset_index("5"));
        assert_eq!(6, snapshot_preset_index("preset-6"));
    }

    #[test]
    fn snapshot_request_keeps_clicked_action_separate_from_preset() {
        let args = args(&[
            "storybook",
            "--visual-snapshot",
            "target/checkbox.png",
            "checkbox",
            "dark",
            "preset-1",
            "clicked",
        ]);

        assert_eq!(
            SnapshotRequest {
                preset_index: INTERACTIVE_PRESET_INDEX,
                clicked: true,
                scroll_y: 0,
                scrollbar_visible: true,
            },
            snapshot_request(&args)
        );
    }

    #[test]
    fn snapshot_request_keeps_legacy_clicked_operation() {
        let args = args(&[
            "storybook",
            "--visual-snapshot",
            "target/checkbox.png",
            "checkbox",
            "dark",
            "clicked",
        ]);

        assert_eq!(
            SnapshotRequest {
                preset_index: DEFAULT_PRESET_INDEX,
                clicked: true,
                scroll_y: 0,
                scrollbar_visible: true,
            },
            snapshot_request(&args)
        );
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }
}
