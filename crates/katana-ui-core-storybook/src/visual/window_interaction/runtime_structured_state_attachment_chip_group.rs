use super::{
    AttachmentChipRuntimeState, CHIP_GROUP_AVAILABLE_WIDTH, CHIP_GROUP_CHIP_WIDTH, CHIP_GROUP_GAP,
    CHIP_GROUP_OVERFLOW_TRIGGER_WIDTH, ChipGroupRuntimeState, RuntimeStructuredUpdate,
};

impl AttachmentChipRuntimeState {
    pub(in crate::visual) fn preview_error(&mut self) -> RuntimeStructuredUpdate {
        self.status_error = attachment_chip_status_error_event();
        RuntimeStructuredUpdate::new(
            "attachment_status",
            "attachment_status_changed",
            attachment_status_label(self.status_error),
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        RuntimeStructuredUpdate::new("attachment_focus", "focus", "focus=attachment")
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hovered = true;
        RuntimeStructuredUpdate::new("attachment_hover", "hover_start", "hover=attachment")
    }

    pub(in crate::visual) fn keyboard_retry(&mut self) -> RuntimeStructuredUpdate {
        self.retried = attachment_chip_retry_event();
        RuntimeStructuredUpdate::new(
            "attachment_keyboard_retry",
            "attachment_retry",
            attachment_retry_label(self.retried),
        )
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "attachment.kind" => self.kind_image = true,
            "attachment.name" => self.name_changed = true,
            "attachment.meta" => self.meta_visible = true,
            "attachment.thumbnail" => self.thumbnail_visible = true,
            "attachment.status" => self.status_error = true,
            "attachment.progress" => self.progress_uploading = true,
            "attachment.retry" => self.retry_visible = true,
            _ => {}
        }
    }
}

fn attachment_chip_status_error_event() -> bool {
    use katana_ui_core::molecule::{
        AttachmentChip, AttachmentChipAction, AttachmentChipEvent, AttachmentKind, AttachmentStatus,
    };

    let mut attachment = AttachmentChip::new(AttachmentKind::File, "design.md");
    let events = attachment.apply_action(AttachmentChipAction::TransitionStatus(
        AttachmentStatus::Error,
    ));
    matches!(
        events.as_slice(),
        [AttachmentChipEvent::StatusChanged {
            previous: AttachmentStatus::Pending,
            current: AttachmentStatus::Error,
            ..
        }]
    )
}

fn attachment_chip_retry_event() -> bool {
    use katana_ui_core::molecule::{
        AttachmentChip, AttachmentChipAction, AttachmentChipEvent, AttachmentKind, AttachmentStatus,
    };

    let mut attachment =
        AttachmentChip::new(AttachmentKind::File, "design.md").status(AttachmentStatus::Error);
    let events = attachment.apply_action(AttachmentChipAction::Retry);
    matches!(
        events.as_slice(),
        [
            AttachmentChipEvent::Retry { .. },
            AttachmentChipEvent::StatusChanged {
                previous: AttachmentStatus::Error,
                current: AttachmentStatus::Pending,
                ..
            }
        ]
    )
}

impl ChipGroupRuntimeState {
    pub(in crate::visual) fn preview_overflow(&mut self) -> RuntimeStructuredUpdate {
        self.overflow_open = chip_group_overflow_open_event();
        RuntimeStructuredUpdate::new(
            "chip_group_overflow",
            "chip_group_overflow_opened",
            chip_group_overflow_label(self.overflow_open),
        )
    }

    pub(in crate::visual) fn focus(&mut self) -> RuntimeStructuredUpdate {
        self.focused = true;
        RuntimeStructuredUpdate::new("chip_group_focus", "focus", "focus=chip")
    }

    pub(in crate::visual) fn hover(&mut self) -> RuntimeStructuredUpdate {
        self.hovered = true;
        RuntimeStructuredUpdate::new("chip_group_hover", "hover_start", "hover=chip")
    }

    pub(in crate::visual) fn keyboard_dismiss(&mut self) -> RuntimeStructuredUpdate {
        self.keyboard_dismissed = chip_group_keyboard_dismiss_event();
        RuntimeStructuredUpdate::new(
            "chip_group_keyboard_dismiss",
            "chip_group_chip_dismissed",
            chip_group_dismiss_label(self.keyboard_dismissed),
        )
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "chip_group.label" => self.label_changed = true,
            "chip_group.chip_count" => self.chip_count_five = true,
            "chip_group.wrap" => self.wrap_enabled = true,
            "chip_group.overflow" => self.overflow_menu = true,
            "chip_group.reorder" => self.reorder_enabled = true,
            "chip_group.gap" => self.gap_eight = true,
            "chip_group.available_width" => self.width_expanded = true,
            "chip_group.overflow_trigger_width" => self.trigger_wide = true,
            "chip_group.hidden_count" => self.hidden_count_two = true,
            _ => {}
        }
    }
}

const fn attachment_status_label(status_error: bool) -> &'static str {
    if status_error {
        "status=error"
    } else {
        "status=unknown"
    }
}

const fn attachment_retry_label(retried: bool) -> &'static str {
    if retried {
        "retry=requested"
    } else {
        "retry=ignored"
    }
}

const fn chip_group_overflow_label(open: bool) -> &'static str {
    if open {
        "overflow=open"
    } else {
        "overflow=closed"
    }
}

const fn chip_group_dismiss_label(dismissed: bool) -> &'static str {
    if dismissed {
        "dismissed=focused"
    } else {
        "dismissed=ignored"
    }
}

fn chip_group_overflow_open_event() -> bool {
    use katana_ui_core::atom::Chip;
    use katana_ui_core::molecule::{ChipGroup, ChipGroupAction, ChipGroupEvent, ChipGroupOverflow};

    let second = Chip::new("second");
    let third = Chip::new("third");
    let second_id = second.state_id().clone();
    let third_id = third.state_id().clone();
    let mut group = ChipGroup::new("filters")
        .chip(Chip::new("first"), CHIP_GROUP_CHIP_WIDTH)
        .chip(second, CHIP_GROUP_CHIP_WIDTH)
        .chip(third, CHIP_GROUP_CHIP_WIDTH)
        .gap(CHIP_GROUP_GAP)
        .available_width(CHIP_GROUP_AVAILABLE_WIDTH)
        .overflow_trigger_width(CHIP_GROUP_OVERFLOW_TRIGGER_WIDTH)
        .wrap(false)
        .overflow(ChipGroupOverflow::Menu);
    let events = group.apply_action(ChipGroupAction::OpenOverflow);
    group.overflow_open()
        && events
            == [ChipGroupEvent::OverflowOpened {
                hidden_chip_ids: vec![second_id, third_id],
            }]
}

fn chip_group_keyboard_dismiss_event() -> bool {
    use katana_ui_core::atom::{Chip, ChipKeyboardInput};
    use katana_ui_core::molecule::{ChipGroup, ChipGroupEvent, ChipGroupFocusTarget};

    let first = Chip::new("first");
    let first_id = first.state_id().clone();
    let second = Chip::new("second").dismissible(true).focused(true);
    let second_id = second.state_id().clone();
    let mut group = ChipGroup::new("filters")
        .chip(first, CHIP_GROUP_CHIP_WIDTH)
        .chip(second, CHIP_GROUP_CHIP_WIDTH);
    let events = group.dismiss_focused_with_keyboard(ChipKeyboardInput::Backspace);
    events
        == [ChipGroupEvent::ChipDismissed {
            chip_id: second_id,
            focus_target: ChipGroupFocusTarget::Chip(first_id),
        }]
}

#[cfg(test)]
mod tests {
    use super::{
        AttachmentChipRuntimeState, ChipGroupRuntimeState, attachment_retry_label,
        attachment_status_label, chip_group_dismiss_label, chip_group_overflow_label,
    };

    #[test]
    fn runtime_labels_and_unknown_options_cover_success_failure_and_noop_contracts() {
        assert_eq!("status=error", attachment_status_label(true));
        assert_eq!("status=unknown", attachment_status_label(false));
        assert_eq!("retry=requested", attachment_retry_label(true));
        assert_eq!("retry=ignored", attachment_retry_label(false));
        assert_eq!("overflow=open", chip_group_overflow_label(true));
        assert_eq!("overflow=closed", chip_group_overflow_label(false));
        assert_eq!("dismissed=focused", chip_group_dismiss_label(true));
        assert_eq!("dismissed=ignored", chip_group_dismiss_label(false));

        let mut attachment = AttachmentChipRuntimeState::default();
        attachment.apply_option("unknown.setting");
        assert_eq!(AttachmentChipRuntimeState::default(), attachment);

        let mut group = ChipGroupRuntimeState::default();
        group.apply_option("unknown.setting");
        assert_eq!(ChipGroupRuntimeState::default(), group);
    }
}
