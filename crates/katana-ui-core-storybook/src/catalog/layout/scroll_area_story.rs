use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::{UiRect, UiScrollbarVisibility, UiStateId};
use katana_ui_core::{atom, layout};

const SCROLL_VIEWPORT_PX: (u32, u32) = (320, 220);
const SCROLL_CONTENT_EXTENT_PX: (u32, u32) = (860, 1400);
const SCROLL_OFFSET_PX: (u32, u32) = (40, 180);
const SCROLL_EDGE_THRESHOLD_PX: u32 = 24;
const SCROLL_BY_DELTA_PX: (i32, i32) = (0, 220);
const SCROLL_INTO_VIEW_RECT: UiRect = UiRect::new(0, 980, 120, 80);

pub(super) fn story() -> StoryExample {
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
