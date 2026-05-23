use super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::layout::SplitPaneResizeSource;
use katana_ui_core::render_model::{UiRect, UiScrollbarVisibility, UiStateId};
use katana_ui_core::{atom, layout};
const SPLIT_PANE_MIN_PERCENT: u8 = 20;
const SPLIT_PANE_MAX_PERCENT: u8 = 80;
const SPLIT_PANE_RESET_PERCENT: u8 = 50;
const SPLIT_PANE_RESIZE_PERCENT: u8 = 64;
const SPLIT_PANE_KEYBOARD_PERCENT: u8 = 56;
const SPLIT_PANE_CLAMP_INPUT_PERCENT: u8 = 8;
const SPLIT_PANE_HANDLE_WIDTH_PX: u8 = 8;
const SPLIT_PANE_KEYBOARD_DELTA: i8 = 4;
const SPLIT_PANE_DISABLED_POINTER_DELTA: i8 = 8;
const SCROLL_VIEWPORT_PX: (u32, u32) = (320, 220);
const SCROLL_CONTENT_EXTENT_PX: (u32, u32) = (860, 1400);
const SCROLL_OFFSET_PX: (u32, u32) = (40, 180);
const SCROLL_EDGE_THRESHOLD_PX: u32 = 24;
const SCROLL_BY_DELTA_PX: (i32, i32) = (0, 220);
const SCROLL_INTO_VIEW_RECT: UiRect = UiRect::new(0, 980, 120, 80);
const NESTED_SPLIT_CONFIG: (u8, u8, u8, u8) = (58, 30, 70, 6);

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
        scroll_area_story(),
        split_pane_story(),
        StoryCatalog::story(
            "align-center",
            layout::AlignCenter::new().child(atom::Text::new("Centered")),
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
        .handle_width_px(SPLIT_PANE_HANDLE_WIDTH_PX)
        .first(atom::Text::new(split_pane_preset_label(
            "horizontal",
            "axis=Horizontal ratio=50 min=20 max=80 handle=8 resize_mode=Drag",
        )))
        .second(atom::Text::new(split_pane_preset_label(
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
        .child(split_pane_nested_preview())
        .child(atom::Text::new(
            "settings: axis ratio min max reset handle resize_mode",
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

fn scroll_area_story() -> StoryExample {
    let area = layout::ScrollArea::new()
        .axis(layout::ScrollAxis::Both)
        .viewport(SCROLL_VIEWPORT_PX.0, SCROLL_VIEWPORT_PX.1)
        .content_extent(SCROLL_CONTENT_EXTENT_PX.0, SCROLL_CONTENT_EXTENT_PX.1)
        .offset(SCROLL_OFFSET_PX.0, SCROLL_OFFSET_PX.1)
        .scrollbar_visibility(layout::ScrollbarVisibility::Always)
        .scrollbar_placement(layout::ScrollbarPlacement::Reserved)
        .edge_threshold(SCROLL_EDGE_THRESHOLD_PX)
        .child(atom::Text::new(
            "settings: axis offset viewport content scrollbar visibility placement edge_threshold",
        ))
        .child(atom::Text::new(
            "state: offset=40,180 viewport=320x220 content=860x1400 edge=none",
        ))
        .child(atom::Text::new(
            "event: Scrolled ScrollEdgeReached ScrollCommandRejected",
        ))
        .child(atom::Text::new(
            "action: scroll_to scroll_by scroll_into_view scrollbar_visibility",
        ))
        .child(atom::Text::new(
            "preset: vertical horizontal both nested theme scroll",
        ))
        .child(atom::Text::new(
            "quality: nested_state_identity clamp edge_event axis_rejection",
        ));
    let target = area.state_id().clone();
    let mut probe = area.clone();
    let logs = scroll_area_logs(&mut probe, target);
    StoryCatalog::interactive_story("scroll-area", area, logs)
}

fn scroll_area_logs(area: &mut layout::ScrollArea, target: UiStateId) -> Vec<UiCallbackLog> {
    let mut logs = Vec::new();
    logs.extend(
        area.apply_action(&UiAction::scroll_to(
            target.clone(),
            SCROLL_OFFSET_PX.0,
            SCROLL_OFFSET_PX.1,
        ))
        .callback_log,
    );
    logs.extend(
        area.apply_action(&UiAction::scroll_by(
            target.clone(),
            SCROLL_BY_DELTA_PX.0,
            SCROLL_BY_DELTA_PX.1,
        ))
        .callback_log,
    );
    logs.extend(
        area.apply_action(&UiAction::scroll_into_view(
            target.clone(),
            SCROLL_INTO_VIEW_RECT,
        ))
        .callback_log,
    );
    logs.extend(
        area.apply_action(&UiAction::scrollbar_visibility(
            target.clone(),
            UiScrollbarVisibility::Auto,
        ))
        .callback_log,
    );
    logs.push(UiCallbackLog::new(
        target,
        "scroll_axis_rejected",
        "axis=Vertical dx=24",
        "ScrollCommandRejected(AxisMismatch)",
    ));
    logs
}

fn split_pane_nested_preview() -> layout::SplitPane {
    layout::SplitPane::new()
        .axis(layout::SplitPaneAxis::Vertical)
        .ratio_percent(NESTED_SPLIT_CONFIG.0)
        .min_percent(NESTED_SPLIT_CONFIG.1)
        .max_percent(NESTED_SPLIT_CONFIG.2)
        .handle_width_px(NESTED_SPLIT_CONFIG.3)
        .first(atom::Text::new(split_pane_preset_label(
            "nested",
            "children=2 nested=true axis=Vertical ratio=58 handle=6",
        )))
        .second(atom::Text::new("nested detail"))
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
                SPLIT_PANE_RESIZE_PERCENT,
            ))
            .callback_log,
    );
    logs.extend(
        split
            .apply_action(&UiAction::split_pane_resize_by(
                target.clone(),
                SPLIT_PANE_KEYBOARD_DELTA,
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
    let mut disabled = layout::SplitPane::new().resize_mode(layout::SplitPaneResizeMode::Disabled);
    logs.extend(
        disabled
            .apply_action(&UiAction::split_pane_resize_by(
                disabled.state_id().clone(),
                SPLIT_PANE_DISABLED_POINTER_DELTA,
                SplitPaneResizeSource::Pointer,
            ))
            .callback_log,
    );
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
