pub(in crate::visual) fn tooltip_state(setting: &'static str) -> &'static str {
    match setting {
        "open" => "tooltip.open=true",
        "placement" => "tooltip.placement=edge",
        "focus" => "tooltip.focus=first",
        "dismiss" => "tooltip.dismiss=outside",
        _ => setting,
    }
}

pub(in crate::visual) fn popover_state(setting: &'static str) -> &'static str {
    match setting {
        "open" => "popover.open=true",
        "placement" => "popover.placement=edge",
        "focus" => "popover.focus=first",
        "dismiss" => "popover.dismiss=outside",
        _ => setting,
    }
}

pub(in crate::visual) fn modal_state(setting: &'static str) -> &'static str {
    match setting {
        "open" => "modal.open=true",
        "placement" => "modal.placement=edge",
        "focus" => "modal.focus=first",
        "dismiss" => "modal.dismiss=outside",
        _ => setting,
    }
}

pub(in crate::visual) fn modal_overlay_state(setting: &'static str) -> &'static str {
    match setting {
        "open" => "modal_overlay.open=true",
        "placement" => "modal_overlay.placement=edge",
        "focus" => "modal_overlay.focus=first",
        "dismiss" => "modal_overlay.dismiss=outside",
        _ => setting,
    }
}
