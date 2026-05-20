use super::rich_content::PopoverFocusManagement;
use super::types::DisclosureTypedModel;
use crate::interaction::placement::Placement;
use crate::render_model::{UiPopoverFocusManagement, UiPopoverPlacement, UiPopoverProps};

pub(super) fn popover_props(model: &DisclosureTypedModel) -> UiPopoverProps {
    UiPopoverProps {
        anchor: model.anchor_summary.clone(),
        placement: placement(model.placement.as_str()),
        offset_x: model.offset.0,
        offset_y: model.offset.1,
        width: model.width.clone(),
        focus_handling: model.focus_handling.clone(),
        dismiss_on_outside_click: model.outside_click_dismiss,
        dismiss_on_escape: model.escape_dismiss,
        arrow_visible: model.arrow.visible,
        arrow_size_px: model.arrow.size_px,
        arrow_tone: model.arrow.tone.clone(),
        heading: model.slots.heading.clone(),
        body: model.slots.body.clone(),
        footer: model.slots.footer.clone(),
        action_count: model.slots.actions.len(),
        focus_management: focus_management(&model.focus_management),
        auto_flip_priority: model
            .auto_flip_priority
            .iter()
            .copied()
            .map(placement_from_engine)
            .collect(),
    }
}

fn focus_management(value: &PopoverFocusManagement) -> UiPopoverFocusManagement {
    match value {
        PopoverFocusManagement::None => UiPopoverFocusManagement::None,
        PopoverFocusManagement::FirstInteractive => UiPopoverFocusManagement::FirstInteractive,
        PopoverFocusManagement::NodeId(_) => UiPopoverFocusManagement::NodeId,
    }
}

fn placement(value: &str) -> UiPopoverPlacement {
    match value {
        "top" => UiPopoverPlacement::Top,
        "top-start" => UiPopoverPlacement::TopStart,
        "top-end" => UiPopoverPlacement::TopEnd,
        "right" => UiPopoverPlacement::Right,
        "right-start" => UiPopoverPlacement::RightStart,
        "right-end" => UiPopoverPlacement::RightEnd,
        "bottom" => UiPopoverPlacement::Bottom,
        "bottom-end" => UiPopoverPlacement::BottomEnd,
        "left" => UiPopoverPlacement::Left,
        "left-start" => UiPopoverPlacement::LeftStart,
        "left-end" => UiPopoverPlacement::LeftEnd,
        _ => UiPopoverPlacement::BottomStart,
    }
}

fn placement_from_engine(value: Placement) -> UiPopoverPlacement {
    match value {
        Placement::Top => UiPopoverPlacement::Top,
        Placement::TopStart => UiPopoverPlacement::TopStart,
        Placement::TopEnd => UiPopoverPlacement::TopEnd,
        Placement::Right => UiPopoverPlacement::Right,
        Placement::RightStart => UiPopoverPlacement::RightStart,
        Placement::RightEnd => UiPopoverPlacement::RightEnd,
        Placement::Bottom => UiPopoverPlacement::Bottom,
        Placement::BottomStart => UiPopoverPlacement::BottomStart,
        Placement::BottomEnd => UiPopoverPlacement::BottomEnd,
        Placement::Left => UiPopoverPlacement::Left,
        Placement::LeftStart => UiPopoverPlacement::LeftStart,
        Placement::LeftEnd => UiPopoverPlacement::LeftEnd,
    }
}
