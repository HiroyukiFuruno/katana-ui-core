use super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, layout, molecule};

const SPLIT_PANE_MIN_PERCENT: u8 = 20;
const SPLIT_PANE_MAX_PERCENT: u8 = 80;
const SPLIT_PANE_RESET_PERCENT: u8 = 50;
const SPLIT_PANE_RESIZE_PERCENT: u8 = 64;
const SPLIT_PANE_KEYBOARD_PERCENT: u8 = 56;
const SPLIT_PANE_CLAMP_INPUT_PERCENT: u8 = 8;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        StoryCatalog::story("row", layout::Row::new().child(atom::Text::new("Row item"))),
        StoryCatalog::story(
            "column",
            layout::Column::new().child(atom::Text::new("Column item")),
        ),
        StoryCatalog::story(
            "stack",
            layout::Stack::new().child(atom::Text::new("Stack item")),
        ),
        StoryCatalog::story(
            "grid",
            layout::Grid::new()
                .child(atom::Text::new("Grid item"))
                .child(atom::Text::new("Grid item 2")),
        ),
        StoryCatalog::story(
            "scroll-area",
            layout::ScrollArea::new().child(atom::Text::new("Scroll item")),
        ),
        split_pane_story(),
        StoryCatalog::story(
            "align-center",
            layout::AlignCenter::new().child(atom::Text::new("Centered")),
        ),
        StoryCatalog::story(
            "theme-tokens",
            molecule::Card::new("Theme tokens")
                .child(atom::Badge::new("Light/Dark"))
                .child(atom::ColorSwatch::new("Accent")),
        ),
    ]
}

fn split_pane_story() -> StoryExample {
    let mut split = layout::SplitPane::new()
        .axis(layout::SplitPaneAxis::Horizontal)
        .ratio_percent(SPLIT_PANE_RESET_PERCENT)
        .min_percent(SPLIT_PANE_MIN_PERCENT)
        .max_percent(SPLIT_PANE_MAX_PERCENT)
        .reset_percent(SPLIT_PANE_RESET_PERCENT)
        .handle_width_px(8)
        .child(atom::Text::new(split_pane_preset_label(
            "horizontal",
            "axis=Horizontal ratio=50 min=20 max=80 handle=8 resize_mode=Drag",
        )))
        .child(atom::Text::new(split_pane_preset_label(
            "vertical",
            "axis=Vertical ratio=42 min=24 max=76 handle=10 resize_mode=Keyboard",
        )));
    let target = split.state_id().clone();
    let logs = split_pane_logs(&mut split, target);
    let preview = layout::Column::new()
        .child(split)
        .child(atom::Text::new(split_pane_preset_label(
            "min clamp",
            "axis=Horizontal ratio=8->20 min=20 max=80 handle=8 resize_mode=Clamp",
        )))
        .child(atom::Text::new(split_pane_preset_label(
            "reset",
            "axis=Horizontal ratio=64->50 min=20 max=80 reset=50",
        )))
        .child(atom::Text::new(split_pane_preset_label(
            "keyboard resize",
            "axis=Horizontal ratio=50->56 min=20 max=80 resize_mode=Keyboard",
        )))
        .child(split_pane_nested_preview());
    StoryCatalog::interactive_story("split-pane", preview, logs)
}

fn split_pane_nested_preview() -> layout::SplitPane {
    layout::SplitPane::new()
        .axis(layout::SplitPaneAxis::Vertical)
        .ratio_percent(58)
        .min_percent(30)
        .max_percent(70)
        .handle_width_px(6)
        .child(atom::Text::new(split_pane_preset_label(
            "nested",
            "children=2 nested=true axis=Vertical ratio=58 handle=6",
        )))
        .child(atom::Text::new("nested detail"))
}

fn split_pane_logs(split: &mut layout::SplitPane, target: UiStateId) -> Vec<UiCallbackLog> {
    let mut logs = Vec::new();
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_resized(
                target.clone(),
                SPLIT_PANE_RESIZE_PERCENT,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_keyboard_resize(
                target.clone(),
                SPLIT_PANE_KEYBOARD_PERCENT,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_reset(target.clone()))
            .callback_log,
    );
    logs.push(split_pane_log(
        target.clone(),
        "split_pane_drag_start",
        "handle=idle ratio=50",
        "handle=dragging ratio=50",
    ));
    logs.push(split_pane_log(
        target.clone(),
        "split_pane_drag_end",
        "handle=dragging ratio=64",
        "handle=idle ratio=64",
    ));
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_resized(
                target.clone(),
                SPLIT_PANE_CLAMP_INPUT_PERCENT,
            ))
            .callback_log,
    );
    logs.push(split_pane_log(
        target,
        "split_pane_clamped",
        "requested=8 min=20 max=80",
        "ratio=20 clamped=true",
    ));
    logs
}

fn split_pane_log(
    target: UiStateId,
    action: &'static str,
    before: &'static str,
    after: &'static str,
) -> UiCallbackLog {
    UiCallbackLog::new(target, action, before, after)
}

fn split_pane_preset_label(preset: &str, settings: &str) -> String {
    format!("{preset}: {settings}")
}
