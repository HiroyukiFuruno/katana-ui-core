use katana_ui_core::render_model::{
    UiContextMenuAnchor, UiContextMenuItemKind, UiContextMenuPlacement, UiProps,
};

const POINTER_VALUE: &str = "Pointer(192,128)";
const POINTER_X: i32 = 192;
const POINTER_Y: i32 = 128;

pub(super) fn anchor(value: &str) -> UiContextMenuAnchor {
    if value == POINTER_VALUE {
        return UiContextMenuAnchor::Pointer {
            x: POINTER_X,
            y: POINTER_Y,
        };
    }
    UiContextMenuAnchor::Pointer { x: 0, y: 0 }
}

pub(super) fn anchor_value(anchor: &UiContextMenuAnchor) -> String {
    match anchor {
        UiContextMenuAnchor::Pointer { x, y } => format!("Pointer({x},{y})"),
        UiContextMenuAnchor::VirtualRect(rect) => {
            format!(
                "VirtualRect({},{},{},{})",
                rect.x, rect.y, rect.width, rect.height
            )
        }
        UiContextMenuAnchor::NodeId(id) => format!("NodeId({id})"),
    }
}

pub(super) fn placement(value: &str) -> UiContextMenuPlacement {
    match value {
        "BelowEnd" => UiContextMenuPlacement::BelowEnd,
        "AboveStart" => UiContextMenuPlacement::AboveStart,
        "AboveEnd" => UiContextMenuPlacement::AboveEnd,
        "RightStart" => UiContextMenuPlacement::RightStart,
        "LeftStart" => UiContextMenuPlacement::LeftStart,
        _ => UiContextMenuPlacement::BelowStart,
    }
}

pub(super) fn placement_priority(value: &str) -> Vec<UiContextMenuPlacement> {
    value.split('>').map(placement).collect()
}

pub(super) fn placement_priority_value(priority: &[UiContextMenuPlacement]) -> String {
    priority
        .iter()
        .map(|it| format!("{it:?}"))
        .collect::<Vec<_>>()
        .join(">")
}

pub(super) fn item_kind_value(props: &UiProps) -> String {
    props
        .context_menu
        .items
        .iter()
        .find(|it| !matches!(it.kind, UiContextMenuItemKind::Divider))
        .map_or_else(|| "Action".to_string(), |it| format!("{:?}", it.kind))
}

pub(super) fn set_item_kind(props: &mut UiProps, value: &str) {
    if let Some(item) = props.context_menu.items.first_mut() {
        item.kind = item_kind(value);
    }
}

fn item_kind(value: &str) -> UiContextMenuItemKind {
    match value {
        "Toggle" => UiContextMenuItemKind::Toggle,
        "Radio" => UiContextMenuItemKind::Radio,
        "Submenu" => UiContextMenuItemKind::Submenu,
        "Section" => UiContextMenuItemKind::Section,
        "Divider" => UiContextMenuItemKind::Divider,
        _ => UiContextMenuItemKind::Action,
    }
}
