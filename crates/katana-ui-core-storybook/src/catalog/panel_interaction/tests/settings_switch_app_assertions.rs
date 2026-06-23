use super::*;

pub(super) fn assert_banner_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "banner.severity",
        "banner.density",
        "banner.actions",
        "banner.details",
        "banner.dismissible",
        "banner.title",
        "banner.leading_icon",
        "banner.placement",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "banner"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "banner_settings_changed"
            }),
            "missing banner setting mutation for {option}"
        );
    }
}

pub(super) fn assert_card_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in ["card.label", "card.header", "card.footer", "card.padding"] {
        assert!(
            settings.iter().any(|it| {
                it.page == "card"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "card_settings_changed"
            }),
            "missing card setting mutation for {option}"
        );
    }
}

pub(super) fn assert_toast_stack_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "toast_stack.position",
        "toast_stack.max_visible",
        "toast_stack.dedup",
        "toast_stack.duration",
        "toast_stack.pause_on_hover",
        "toast_stack.stack_gap",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "toast-stack-manager"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "toast_stack_settings_changed"
            }),
            "missing toast stack setting mutation for {option}"
        );
    }
}

pub(super) fn assert_status_bar_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "status_bar.mode",
        "status_bar.segments",
        "status_bar.density",
        "status_bar.message",
        "status_bar.severity",
        "status_bar.dismiss",
        "status_bar.segment_a11y",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "status-bar"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "status_bar_settings_changed"
            }),
            "missing status bar setting mutation for {option}"
        );
    }
}

pub(super) fn assert_shortcut_combo_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "shortcut_combo.platform_display",
        "shortcut_combo.separator",
        "shortcut_combo.size",
        "shortcut_combo.tone",
        "shortcut_combo.a11y_label",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "shortcut-combo"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "shortcut_combo_settings_changed"
            }),
            "missing shortcut combo setting mutation for {option}"
        );
    }
}

pub(super) fn assert_shortcut_cheatsheet_settings_are_switchable(
    settings: &[SettingsMutationReport],
) {
    for option in [
        "shortcut_cheatsheet.label",
        "shortcut_cheatsheet.groups",
        "shortcut_cheatsheet.group_title",
        "shortcut_cheatsheet.items",
        "shortcut_cheatsheet.item_combo",
        "shortcut_cheatsheet.group_layout",
        "shortcut_cheatsheet.query",
        "shortcut_cheatsheet.selected",
        "shortcut_cheatsheet.result_count",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "shortcut-cheatsheet"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "shortcut_cheatsheet_settings_changed"
            }),
            "missing shortcut cheatsheet setting mutation for {option}"
        );
    }
}

pub(super) fn assert_empty_state_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "empty_state.heading",
        "empty_state.body",
        "empty_state.icon",
        "empty_state.illustration",
        "empty_state.tone",
        "empty_state.size",
        "empty_state.alignment",
        "empty_state.actions",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "empty-state"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "empty_state_settings_changed"
            }),
            "missing empty-state setting mutation for {option}"
        );
    }
}

pub(super) fn assert_color_picker_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "color_picker.rgba",
        "color_picker.value",
        "color_picker.open",
        "color_picker.hue",
        "color_picker.alpha",
        "color_picker.blending",
        "color_picker.color_area",
        "color_picker.trigger_size",
        "color_picker.title",
        "color_picker.rgba_mode",
        "color_picker.panel_scale_percent",
        "color_picker.trigger_border",
        "color_picker.eyedropper_callback",
        "color_picker.readonly",
        "color_picker.disabled",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "color-picker-rgba"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "color_picker_settings_changed"
            }),
            "missing color picker setting mutation for {option}"
        );
    }
}

pub(super) fn assert_diagnostics_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "diagnostics.group_by",
        "diagnostics.sort_by",
        "diagnostics.severity_filter",
        "diagnostics.bulk_action",
        "diagnostics.fix_preview",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "diagnostics-list"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "diagnostics_list_settings_changed"
            }),
            "missing diagnostics-list setting mutation for {option}"
        );
    }
}

pub(super) fn assert_chip_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for (page, option, event) in [
        ("chip", "chip.label", "chip_settings_changed"),
        ("chip", "chip.leading_icon", "chip_settings_changed"),
        ("chip", "chip.trailing_icon", "chip_settings_changed"),
        ("chip", "chip.variant", "chip_settings_changed"),
        ("chip", "chip.tone", "chip_settings_changed"),
        ("chip", "chip.size", "chip_settings_changed"),
        ("chip", "chip.interactive", "chip_settings_changed"),
        ("chip", "chip.selected", "chip_settings_changed"),
        ("chip", "chip.disabled", "chip_settings_changed"),
        ("chip", "chip.dismissible", "chip_settings_changed"),
        ("chip", "chip.a11y_label", "chip_settings_changed"),
        ("chip", "chip.focused", "chip_settings_changed"),
        (
            "attachment-chip",
            "attachment.name",
            "attachment_chip_settings_changed",
        ),
        (
            "attachment-chip",
            "attachment.meta",
            "attachment_chip_settings_changed",
        ),
        (
            "attachment-chip",
            "attachment.thumbnail",
            "attachment_chip_settings_changed",
        ),
        (
            "attachment-chip",
            "attachment.status",
            "attachment_chip_settings_changed",
        ),
        (
            "attachment-chip",
            "attachment.progress",
            "attachment_chip_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.label",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.chip_count",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.wrap",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.overflow",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.reorder",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.gap",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.available_width",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.overflow_trigger_width",
            "chip_group_settings_changed",
        ),
        (
            "chip-group",
            "chip_group.hidden_count",
            "chip_group_settings_changed",
        ),
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == page
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == event
            }),
            "missing chip setting mutation for {page} {option}"
        );
    }
}

pub(super) fn assert_toolbar_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "toolbar.action_count",
        "toolbar.priority",
        "toolbar.overflow_strategy",
        "toolbar.display_mode",
        "toolbar.density",
        "toolbar.action_priority",
        "toolbar.action_accelerator",
        "toolbar.action_split",
        "toolbar.action_group",
        "toolbar.action_tooltip",
        "toolbar.action_a11y",
        "toolbar.action_disabled",
        "toolbar.group_label",
        "toolbar.group_divider",
        "toolbar.split_disabled",
        "toolbar.split_tooltip",
        "toolbar.split_a11y",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "toolbar"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "toolbar_settings_changed"
            }),
            "missing toolbar setting mutation for {option}"
        );
    }
}

pub(super) fn assert_text_area_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "text_area.submit_key",
        "text_area.newline_key",
        "text_area.tab_behavior",
        "text_area.auto_grow",
        "text_area.wrap_policy",
        "text_area.resize_enabled",
        "text_area.vertical_scroll_enabled",
        "text_area.horizontal_scroll_enabled",
        "text_area.vertical_scrollbar_visible",
        "text_area.horizontal_scrollbar_visible",
    ] {
        assert!(
            settings.iter().any(|it| {
                it.page == "text-area"
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event == "text_area_settings_changed"
            }),
            "missing text-area setting mutation for {option}"
        );
    }
}

pub(super) fn assert_overlay_settings_are_switchable(settings: &[SettingsMutationReport]) {
    for option in [
        "popover.placement",
        "popover.arrow",
        "popover.focus_management",
        "popover.slot",
        "hover_card.delay",
        "hover_card.placement",
        "hover_card.arrow",
        "hover_card.focus",
        "hover_card.slot",
    ] {
        assert!(
            settings.iter().any(|it| {
                (it.page == "popover" || it.page == "hover-card")
                    && it.option.name == option
                    && it.action == format!("set_{option}")
                    && it.event.ends_with("_settings_changed")
            }),
            "missing overlay setting mutation for {option}"
        );
    }
}
