use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::layout::SplitPaneResizeSource;
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, layout};

const MIN_PERCENT: u8 = 20;
const MAX_PERCENT: u8 = 80;
const RESET_PERCENT: u8 = 50;
const RESIZE_PERCENT: u8 = 64;
const KEYBOARD_PERCENT: u8 = 56;
const CLAMP_INPUT_PERCENT: u8 = 8;
const HANDLE_WIDTH_PX: u8 = 8;
const KEYBOARD_DELTA: i8 = 4;
const DISABLED_POINTER_DELTA: i8 = 8;

pub(super) fn story() -> StoryExample {
    let mut split = layout::SplitPane::new()
        .axis(layout::SplitPaneAxis::Horizontal)
        .ratio_percent(RESET_PERCENT)
        .min_percent(MIN_PERCENT)
        .max_percent(MAX_PERCENT)
        .reset_percent(RESET_PERCENT)
        .handle_width_px(HANDLE_WIDTH_PX)
        .first(atom::Text::new(preset_label(
            "axis vertical",
            "axis=Vertical ratio_percent=50 min_percent=20 max_percent=80",
        )))
        .second(atom::Text::new(preset_label(
            "wide gap",
            "gap=12 handle_width_px=8 resize_mode=Drag",
        )));
    let target = split.state_id().clone();
    let logs = split_pane_logs(&mut split, target);
    let preview = layout::Column::new()
        .child(split)
        .child(atom::Text::new(preset_label(
            "center alignment",
            "alignment=Center children=2 nested=true",
        )))
        .child(atom::Text::new(preset_label(
            "overflow scroll",
            "overflow=Scroll children=2 nested=true",
        )))
        .child(atom::Text::new(preset_label(
            "ratio percent",
            "ratio_percent=64 min_percent=20 max_percent=80",
        )))
        .child(atom::Text::new(preset_label(
            "min percent clamp",
            "ratio_percent=8->20 min_percent=20 max_percent=80",
        )))
        .child(atom::Text::new(preset_label(
            "max percent clamp",
            "ratio_percent=92->80 min_percent=20 max_percent=80",
        )))
        .child(atom::Text::new(preset_label(
            "reset percent",
            "ratio_percent=64->50 reset_percent=50",
        )))
        .child(atom::Text::new(preset_label(
            "wide handle",
            "handle_width_px=10 hit_target=24",
        )))
        .child(atom::Text::new(preset_label(
            "keyboard resize mode",
            "resize_mode=KeyboardOnly ratio_percent=50->56",
        )))
        .child(atom::Text::new(
            "settings: axis gap alignment overflow ratio_percent min_percent max_percent reset_percent handle_width_px resize_mode",
        ))
        .child(atom::Text::new(
            "state: ratio=50 dragging=false focused_handle=false last_event=RatioChanged",
        ))
        .child(atom::Text::new(
            "event: ResizeStarted RatioChanged ResizeEnded ResizeRejected",
        ))
        .child(atom::Text::new(
            "action: split_pane_set_ratio split_pane_resize_by split_pane_reset_ratio",
        ))
        .child(atom::Text::new(
            "quality: clamp event_order public_api_guard",
        ));
    StoryCatalog::interactive_story("split-pane", preview, logs)
}

fn split_pane_logs(split: &mut layout::SplitPane, target: UiStateId) -> Vec<UiCallbackLog> {
    let mut logs = Vec::new();
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_start_resize(target.clone()))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_set_ratio(
                target.clone(),
                RESIZE_PERCENT,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_resize_by(
                target.clone(),
                KEYBOARD_DELTA,
                SplitPaneResizeSource::Keyboard,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_reset_ratio(target.clone()))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_end_resize(target.clone()))
            .callback_log,
    );
    logs.extend(resize_action_logs(split, target.clone()));
    logs.push(log(
        target.clone(),
        "split_pane_drag_start",
        "handle=idle ratio=50",
        "handle=dragging ratio=50",
    ));
    logs.push(log(
        target.clone(),
        "split_pane_drag_end",
        "handle=dragging ratio=64",
        "handle=idle ratio=64",
    ));
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_resized(
                target.clone(),
                CLAMP_INPUT_PERCENT,
            ))
            .callback_log,
    );
    logs.push(log(
        target,
        "split_pane_clamped",
        "requested=8 min=20 max=80",
        "ratio=20 clamped=true",
    ));
    logs.extend(disabled_resize_logs());
    logs
}

fn resize_action_logs(split: &mut layout::SplitPane, target: UiStateId) -> Vec<UiCallbackLog> {
    let mut logs = Vec::new();
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_resized(
                target.clone(),
                RESIZE_PERCENT,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_keyboard_resize(
                target.clone(),
                KEYBOARD_PERCENT,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_reset(target))
            .callback_log,
    );
    logs
}

fn disabled_resize_logs() -> Vec<UiCallbackLog> {
    let mut disabled = layout::SplitPane::new().resize_mode(layout::SplitPaneResizeMode::Disabled);
    disabled
        .apply_action(&UiAction::split_pane_resize_by(
            disabled.state_id().clone(),
            DISABLED_POINTER_DELTA,
            SplitPaneResizeSource::Pointer,
        ))
        .callback_log
}

fn log(
    target: UiStateId,
    action: &'static str,
    before: &'static str,
    after: &'static str,
) -> UiCallbackLog {
    UiCallbackLog::new(target, action, before, after)
}

fn preset_label(preset: &str, settings: &str) -> String {
    format!("{preset}: {settings}")
}
