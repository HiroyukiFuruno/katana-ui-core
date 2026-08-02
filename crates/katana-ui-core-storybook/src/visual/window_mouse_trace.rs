use minifb::{MouseButton, MouseMode, Window};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::dedicated_dod_atom_progress_motion::{
    ProgressSegmentMotionSnapshot, progress_segment_motion_snapshot,
};
use super::render_context::ScenarioContext;
use super::runtime::StorybookMouseTraceRuntimeReport;
use super::window_interaction::StorybookWindowState;

const TRACE_PATH_ENV: &str = "KUC_STORYBOOK_MOUSE_TRACE";
const REPORT_POINTER_X: f32 = 12.25;
const REPORT_POINTER_Y: f32 = 34.75;
const REPORT_OPTIONAL_INDEX: usize = 3;
const REPORT_SEGMENT_X: usize = 4;
const REPORT_SEGMENT_WIDTH: usize = 8;
const REPORT_TRACK_WIDTH: usize = 12;

pub(super) trait MouseTraceWindow {
    fn mouse_pos(&self) -> Option<(f32, f32)>;
    fn size(&self) -> (usize, usize);
    fn left_down(&self) -> bool;
}

impl MouseTraceWindow for Window {
    fn mouse_pos(&self) -> Option<(f32, f32)> {
        self.get_unscaled_mouse_pos(MouseMode::Discard)
    }

    fn size(&self) -> (usize, usize) {
        self.get_size()
    }

    fn left_down(&self) -> bool {
        self.get_mouse_down(MouseButton::Left)
    }
}

pub(super) fn record(
    window: &dyn MouseTraceWindow,
    state: &StorybookWindowState,
    frame_index: usize,
) {
    let path = env::var(TRACE_PATH_ENV).ok();
    record_to_optional_path(window, state, frame_index, path.as_deref());
}

fn record_to_optional_path(
    window: &dyn MouseTraceWindow,
    state: &StorybookWindowState,
    frame_index: usize,
    path: Option<&str>,
) {
    let Some(path) = path else {
        return;
    };
    let mouse_pos = window.mouse_pos();
    let (width, height) = window.size();
    let left_down = window.left_down();
    let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(&path))
    else {
        return;
    };
    let (x, y) = format_mouse_position(mouse_pos);
    let screen_state = &state.screen_state;
    let text_selection_active =
        state.text_selection_start.is_some() && state.text_selection_end.is_some();
    let checkbox_hovered_index = format_optional_index(screen_state.checkbox_hovered_index());
    let progress_segment = progress_segment_motion_snapshot(ScenarioContext {
        selected_page: state.selected_page,
        selected_instance_id: state.selected_instance_id,
        preset_index: state.preset_index,
        preset_tab_scroll_x: state.preset_tab_scroll_x,
        tree_expansion: state.tree_expansion,
        scrollbar_visible: state.scrollbar_visible,
        panel_scroll: state.panel_scroll,
        screen_state,
        show_navigation_lines: state.show_navigation_lines,
        show_navigation_text_connectors: state.show_navigation_text_connectors,
    });
    let (progress_segment_x, progress_segment_width) = format_progress_segment(progress_segment);
    let _ = writeln!(
        file,
        "{{\"frame\":{frame_index},\"page\":\"{}\",\"preset_index\":{},\"x\":{},\"y\":{},\"left_down\":{left_down},\"width\":{width},\"height\":{height},\"action_count\":{},\"settings_revision\":{},\"last_action\":\"{}\",\"last_event\":\"{}\",\"state_label\":\"{}\",\"last_setting\":\"{}\",\"last_setting_value\":\"{}\",\"text_selection_active\":{},\"clipboard_text_len\":{},\"progress_percent\":{},\"progress_segment_x\":{},\"progress_segment_width\":{},\"checkbox_0_checked\":{},\"checkbox_1_checked\":{},\"checkbox_focused_index\":{},\"checkbox_hovered_index\":{}}}",
        escape_json(state.selected_page),
        state.preset_index,
        x,
        y,
        screen_state.action_count,
        screen_state.settings_revision,
        escape_json(screen_state.last_action),
        escape_json(screen_state.last_event),
        escape_json(screen_state.state_label),
        escape_json(screen_state.last_setting),
        escape_json(screen_state.last_setting_value),
        text_selection_active,
        state.clipboard_text.chars().count(),
        screen_state.progress_percent(),
        progress_segment_x,
        progress_segment_width,
        screen_state.is_checkbox_checked_at(0),
        screen_state.is_checkbox_checked_at(1),
        screen_state.checkbox_focused_index(),
        checkbox_hovered_index,
    );
}

pub(super) fn runtime_report() -> StorybookMouseTraceRuntimeReport {
    let pointer_values_formatted =
        format_mouse_position(Some((REPORT_POINTER_X, REPORT_POINTER_Y)))
            == ("12.2".to_string(), "34.8".to_string())
            && format_mouse_position(None) == ("null".to_string(), "null".to_string());
    let optional_index_formatted = format_optional_index(Some(REPORT_OPTIONAL_INDEX)) == "3"
        && format_optional_index(None) == "null";
    let progress_segment_formatted = format_progress_segment(Some(ProgressSegmentMotionSnapshot {
        x: REPORT_SEGMENT_X,
        width: REPORT_SEGMENT_WIDTH,
        track_x: 0,
        track_width: REPORT_TRACK_WIDTH,
    })) == ("4".to_string(), "8".to_string())
        && format_progress_segment(None) == ("null".to_string(), "null".to_string());

    StorybookMouseTraceRuntimeReport {
        pointer_values_formatted,
        optional_index_formatted,
        progress_segment_formatted,
    }
}

fn format_mouse_position(mouse_pos: Option<(f32, f32)>) -> (String, String) {
    match mouse_pos {
        Some((x, y)) => (format!("{x:.1}"), format!("{y:.1}")),
        None => ("null".to_string(), "null".to_string()),
    }
}

fn format_optional_index(index: Option<usize>) -> String {
    match index {
        Some(index) => index.to_string(),
        None => "null".to_string(),
    }
}

fn format_progress_segment(segment: Option<ProgressSegmentMotionSnapshot>) -> (String, String) {
    match segment {
        Some(segment) => (segment.x.to_string(), segment.width.to_string()),
        None => ("null".to_string(), "null".to_string()),
    }
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::screen_state::StorybookScreenState;
    use std::fs;

    struct FakeWindow {
        mouse_pos: Option<(f32, f32)>,
        size: (usize, usize),
        left_down: bool,
    }

    impl MouseTraceWindow for FakeWindow {
        fn mouse_pos(&self) -> Option<(f32, f32)> {
            self.mouse_pos
        }

        fn size(&self) -> (usize, usize) {
            self.size
        }

        fn left_down(&self) -> bool {
            self.left_down
        }
    }

    #[test]
    fn mouse_trace_writes_structured_json_for_present_and_missing_pointer() -> std::io::Result<()> {
        let output = std::env::temp_dir().join(format!(
            "katana-ui-core-mouse-trace-{}.jsonl",
            std::process::id()
        ));
        let output_string = output.to_string_lossy().into_owned();
        let state = StorybookWindowState {
            selected_page: "progress-bar",
            preset_index: 1,
            text_selection_start: Some((1, 2)),
            text_selection_end: Some((3, 4)),
            clipboard_text: "a\"b\\c\n".to_string(),
            screen_state: StorybookScreenState {
                last_action: "action\"",
                last_event: "event\\",
                state_label: "line\nnext",
                last_setting: "tab\tvalue",
                last_setting_value: "return\rvalue",
                ..StorybookScreenState::default()
            },
            ..StorybookWindowState::default()
        };
        let present = FakeWindow {
            mouse_pos: Some((12.25, 34.75)),
            size: (800, 600),
            left_down: true,
        };
        let missing = FakeWindow {
            mouse_pos: None,
            size: (320, 240),
            left_down: false,
        };

        record_to_optional_path(&present, &state, 7, Some(&output_string));
        record_to_optional_path(&missing, &state, 8, Some(&output_string));
        record_to_optional_path(&present, &state, 9, None);
        record(&present, &state, 10);

        let lines = fs::read_to_string(&output)?;
        let records = lines
            .lines()
            .map(serde_json::from_str::<serde_json::Value>)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["frame"], 7);
        assert_eq!(records[0]["x"], 12.2);
        assert_eq!(records[0]["left_down"], true);
        assert_eq!(records[0]["text_selection_active"], true);
        assert_eq!(records[1]["x"], serde_json::Value::Null);
        assert_eq!(records[1]["y"], serde_json::Value::Null);
        fs::remove_file(output)?;
        Ok(())
    }

    #[test]
    fn headless_mouse_trace_runtime_report_passes() {
        assert!(runtime_report().passed());
    }

    #[test]
    fn mouse_trace_ignores_unwritable_target_and_escapes_control_characters() {
        let window = FakeWindow {
            mouse_pos: None,
            size: (1, 1),
            left_down: false,
        };
        record_to_optional_path(
            &window,
            &StorybookWindowState::default(),
            0,
            Some(std::env::temp_dir().to_string_lossy().as_ref()),
        );
        assert_eq!(escape_json("\"\\\n\r\tplain"), "\\\"\\\\\\n\\r\\tplain");
    }
}
