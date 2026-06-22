use minifb::{MouseButton, MouseMode, Window};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::dedicated_dod_atom_progress_motion::progress_segment_motion_snapshot;
use super::render_context::ScenarioContext;
use super::window_interaction::StorybookWindowState;

const TRACE_PATH_ENV: &str = "KUC_STORYBOOK_MOUSE_TRACE";

pub(super) fn record(window: &Window, state: &StorybookWindowState, frame_index: usize) {
    let Ok(path) = env::var(TRACE_PATH_ENV) else {
        return;
    };
    let mouse_pos = window.get_unscaled_mouse_pos(MouseMode::Discard);
    let (width, height) = window.get_size();
    let left_down = window.get_mouse_down(MouseButton::Left);
    let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(&path))
    else {
        return;
    };
    let (x, y) = mouse_pos
        .map(|(x, y)| (format!("{x:.1}"), format!("{y:.1}")))
        .unwrap_or_else(|| ("null".to_string(), "null".to_string()));
    let screen_state = &state.screen_state;
    let text_selection_active =
        state.text_selection_start.is_some() && state.text_selection_end.is_some();
    let checkbox_hovered_index = screen_state
        .checkbox_hovered_index()
        .map(|index| index.to_string())
        .unwrap_or_else(|| "null".to_string());
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
    let progress_segment_x = progress_segment
        .map(|segment| segment.x.to_string())
        .unwrap_or_else(|| "null".to_string());
    let progress_segment_width = progress_segment
        .map(|segment| segment.width.to_string())
        .unwrap_or_else(|| "null".to_string());
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
