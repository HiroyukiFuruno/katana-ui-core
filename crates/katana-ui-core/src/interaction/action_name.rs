use crate::interaction::{ColorDragAction, ProgressAction, UiActionSource};

pub(super) fn value_name(
    source: UiActionSource,
    progress: &Option<ProgressAction>,
    color_drag: &Option<ColorDragAction>,
) -> &'static str {
    if progress.is_some() {
        return "progress_changed";
    }
    if color_drag.is_some() {
        return "color_drag";
    }
    match source {
        UiActionSource::Input => "input_value",
        UiActionSource::SlideControl => "slide_changed",
        UiActionSource::SplitPane => "split_pane_resized",
        UiActionSource::SplitPaneReset => "split_pane_reset",
        UiActionSource::SplitPaneKeyboard => "split_pane_keyboard_resize",
        UiActionSource::ColorPickerBlending => "color_blending_changed",
        UiActionSource::CodeDiffMode => "code_diff_mode_changed",
        UiActionSource::CodeDiffDirection => "code_diff_direction_changed",
        _ => "set_value",
    }
}
