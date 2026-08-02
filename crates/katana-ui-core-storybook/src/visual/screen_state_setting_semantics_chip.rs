pub(in crate::visual) fn chip_state(setting: &'static str) -> &'static str {
    match setting {
        "chip.label" => "chip.label=filter: rust",
        "chip.leading_icon" => "chip.leading_icon=tag",
        "chip.trailing_icon" => "chip.trailing_icon=close",
        "chip.variant" => "chip.variant=Filled",
        "chip.tone" => "chip.tone=Danger",
        "chip.size" => "chip.size=Large",
        "chip.interactive" => "chip.interactive=true",
        "chip.selected" => "chip.selected=true",
        "chip.disabled" => "chip.disabled=true",
        "chip.dismissible" => "chip.dismissible=true",
        "chip.a11y_label" => "chip.a11y_label=Filter chip",
        "chip.focused" => "chip.focused=true",
        _ => setting,
    }
}

pub(in crate::visual) fn attachment_chip_state(setting: &'static str) -> &'static str {
    match setting {
        "attachment.kind" => "attachment.kind=Image",
        "attachment.name" => "attachment.name=proposal.pdf",
        "attachment.meta" => "attachment.meta=size+mime",
        "attachment.thumbnail" => "attachment.thumbnail=preview",
        "attachment.status" => "attachment.status=Error",
        "attachment.progress" => "attachment.progress=100",
        "attachment.retry" => "attachment.retry=visible",
        _ => setting,
    }
}

pub(in crate::visual) fn chip_group_state(setting: &'static str) -> &'static str {
    match setting {
        "chip_group.label" => "chip_group.label=Active filters",
        "chip_group.chip_count" => "chip_group.chip_count=5",
        "chip_group.wrap" => "chip_group.wrap=true",
        "chip_group.overflow" => "chip_group.overflow=Menu",
        "chip_group.reorder" => "chip_group.reorder=true",
        "chip_group.gap" => "chip_group.gap=8",
        "chip_group.available_width" => "chip_group.available_width=132",
        "chip_group.overflow_trigger_width" => "chip_group.overflow_trigger_width=32",
        "chip_group.hidden_count" => "chip_group.hidden_count=2",
        _ => setting,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chip_semantics_preserve_unknown_settings() {
        assert_eq!("unknown.chip", chip_state("unknown.chip"));
        assert_eq!(
            "unknown.attachment",
            attachment_chip_state("unknown.attachment")
        );
        assert_eq!("unknown.chip_group", chip_group_state("unknown.chip_group"));
    }
}
