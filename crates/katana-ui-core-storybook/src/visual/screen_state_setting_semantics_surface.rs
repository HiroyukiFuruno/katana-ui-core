pub(in crate::visual) fn badge_state(setting: &'static str) -> &'static str {
    match setting {
        "status.severity" => "badge.status.severity=Danger",
        "badge.passive" => "badge.passive=use-chip",
        "size" => "badge.size=small",
        "tone" => "badge.tone=accent",
        "badge.leading_icon" => "badge.leading_icon=dot",
        "variant" => "badge.variant=filled",
        _ => setting,
    }
}

pub(in crate::visual) fn banner_state(setting: &'static str) -> &'static str {
    match setting {
        "severity" => "banner.severity=warning",
        "density" => "banner.density=compact",
        "action" => "banner.action=visible",
        "dismiss" => "banner.dismiss=true",
        "banner.details" => "banner.details=expanded",
        "banner.title" => "banner.title=visible",
        "banner.leading_icon" => "banner.leading_icon=custom",
        "banner.placement" => "banner.placement=sticky",
        _ => setting,
    }
}

pub(in crate::visual) fn card_state(setting: &'static str) -> &'static str {
    match setting {
        "card.label" => "card.label=Project summary",
        "card.header" => "card.header=custom",
        "card.footer" => "card.footer=visible",
        "card.variant" => "card.variant=theme_border",
        "card.padding" => "card.padding=Large",
        "card.clickable" => "card.clickable=true",
        "card.nested_controls" => "card.nested_controls=interactive",
        "card.child_state" => "card.child_state=changed",
        _ => setting,
    }
}

pub(in crate::visual) fn empty_state_state(setting: &'static str) -> &'static str {
    match setting {
        "empty_state.heading" => "empty_state.heading=Empty project",
        "empty_state.body" => "empty_state.body=create a file",
        "empty_state.icon" => "empty_state.icon=search",
        "empty_state.illustration" => "empty_state.illustration=folder",
        "empty_state.tone" => "empty_state.tone=Danger",
        "empty_state.size" => "empty_state.size=Large",
        "empty_state.alignment" => "empty_state.alignment=Leading",
        "empty_state.actions" => "empty_state.actions=Primary+Secondary",
        _ => setting,
    }
}

pub(in crate::visual) fn feedback_state(
    page_state_prefix: &'static str,
    setting: &'static str,
) -> &'static str {
    match (page_state_prefix, setting) {
        ("toast_stack", "severity") => "toast_stack.severity=warning",
        ("toast_stack", "duration") => "toast_stack.duration=custom",
        ("toast_stack", "action") => "toast_stack.action=visible",
        ("toast_stack", "dismiss") => "toast_stack.dismiss=true",
        ("notification_toast", "severity") => "notification_toast.severity=warning",
        ("notification_toast", "duration") => "notification_toast.duration=custom",
        ("notification_toast", "action") => "notification_toast.action=visible",
        ("notification_toast", "dismiss") => "notification_toast.dismiss=true",
        _ => setting,
    }
}
