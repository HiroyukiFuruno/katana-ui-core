#[path = "legacy_dod_context_menu_options.rs"]
mod legacy_dod_context_menu_options;

use katana_ui_core::render_model::{UiAnimationState, UiProps, UiSize, UiTone, UiVariant};

const INVALID_USIZE_SETTING: usize = 0;
const INVALID_U32_SETTING: u32 = 0;
const INVALID_U8_SETTING: u8 = 0;

pub(super) fn option_value(option: &str, props: &UiProps) -> String {
    match option {
        "theme_id" => props.theme_id.clone(),
        "text.role" => props.text.role.clone(),
        "icon.svg_source" => props.icon.svg_source.clone(),
        "loading.reduced_motion" => props.loading_indicator.reduced_motion.to_string(),
        "loading.animation_state" => format!("{:?}", props.loading_indicator.animation_state),
        "progress.percent" => props.progress_percent.to_string(),
        "variant" | "chip.variant" => format!("{:?}", props.variant),
        "tone" | "chip.tone" => format!("{:?}", props.tone),
        "chip.size" => format!("{:?}", props.size),
        "button.icon_position" => props.button.icon_position.clone(),
        "checked" => props.checked.to_string(),
        "interaction.selected_index" => props.interaction.selected_index.to_string(),
        "interaction.open" => props.interaction.open.to_string(),
        "color_swatch.selected_color" => props.color_swatch.selected_color.clone(),
        "color_picker.trigger_size" => format!("{:?}", props.size),
        "interaction.value" => props.interaction.value.clone(),
        "text_entry.submit_on_enter" => props.text_entry.submit_on_enter.to_string(),
        "interaction.hovered" => props.interaction.hovered.to_string(),
        "status.severity" => format!("{:?}", props.status.severity),
        "shortcut.platform" => props.shortcut.platform.clone(),
        "interaction.active" => props.interaction.active.to_string(),
        "context_menu.anchor" => {
            legacy_dod_context_menu_options::anchor_value(&props.context_menu.anchor)
        }
        "context_menu.placement" => format!("{:?}", props.context_menu.placement_used),
        "context_menu.placement_priority" => {
            legacy_dod_context_menu_options::placement_priority_value(
                &props.context_menu.placement_priority,
            )
        }
        "context_menu.placement_used" => format!("{:?}", props.context_menu.placement_used),
        "context_menu.min_width" => props.context_menu.min_width.to_string(),
        "context_menu.max_height" => props.context_menu.max_height.to_string(),
        "context_menu.item_kind" => legacy_dod_context_menu_options::item_kind_value(props),
        _ => props.theme_id.clone(),
    }
}

pub(super) fn option_state_summary(option: &str, props: &UiProps) -> String {
    format!(
        "{} option:{}={}",
        props.interaction.summary(),
        option,
        option_value(option, props)
    )
}

pub(super) fn resolved_after_value(
    _option: &str,
    value_type: &str,
    configured: &str,
    before: &str,
) -> String {
    if configured != before {
        return configured.to_string();
    }
    match value_type {
        "bool" => toggled_bool(before),
        "usize" => changed_usize(before),
        "u8" => changed_u8(before),
        "ThemeId" => alternate_text(before, "dark", "light"),
        "UiVariant" => alternate_text(before, "Icon", "Outline"),
        "UiTone" => alternate_text(before, "Danger", "Success"),
        "AnimationState" => alternate_text(before, "Paused", "Running"),
        "TriggerSize" => alternate_text(before, "large", "small"),
        "Ratio" => alternate_text(before, "0.64", "0.42"),
        "DiffMode" => alternate_text(before, "Split", "Inline"),
        _ => format!("{configured}-override"),
    }
}

pub(super) fn props_with_option(props: &UiProps, option: &str, value: &str) -> UiProps {
    let mut next = props.clone();
    match option {
        "theme_id" => next.theme_id = value.to_string(),
        "text.role" => next.text.role = value.to_string(),
        "icon.svg_source" => next.icon.svg_source = value.to_string(),
        "loading.reduced_motion" => {
            next.loading_indicator.reduced_motion = value == "true";
            next.interaction.reduced_motion = value == "true";
        }
        "loading.animation_state" => {
            next.loading_indicator.animation_state = animation_state(value);
        }
        "progress.percent" => next.progress_percent = parse_u8(value),
        "variant" | "chip.variant" => next.variant = variant(value),
        "tone" | "chip.tone" => next.tone = tone(value),
        "chip.size" => next.size = size(value),
        "button.icon_position" => next.button.icon_position = value.to_string(),
        "checked" => next.checked = value == "true",
        "interaction.selected_index" => {
            next.interaction.selected_index = parse_usize(value);
            next.interaction.has_selection = true;
        }
        "interaction.open" => next.interaction.open = value == "true",
        "color_swatch.selected_color" => next.color_swatch.selected_color = value.to_string(),
        "color_picker.trigger_size" => next.size = size(value),
        "interaction.value" => next.interaction.value = value.to_string(),
        "text_entry.submit_on_enter" => next.text_entry.submit_on_enter = value == "true",
        "interaction.hovered" => next.interaction.hovered = value == "true",
        "status.severity" => next.status.severity = tone(value),
        "shortcut.platform" => next.shortcut.platform = value.to_string(),
        "interaction.active" => next.interaction.active = value == "true",
        "context_menu.anchor" => {
            next.context_menu.anchor = legacy_dod_context_menu_options::anchor(value);
        }
        "context_menu.placement" => {
            next.context_menu.placement_used = legacy_dod_context_menu_options::placement(value);
        }
        "context_menu.placement_priority" => {
            next.context_menu.placement_priority =
                legacy_dod_context_menu_options::placement_priority(value);
        }
        "context_menu.placement_used" => {
            next.context_menu.placement_used = legacy_dod_context_menu_options::placement(value);
        }
        "context_menu.min_width" => next.context_menu.min_width = parse_u32(value),
        "context_menu.max_height" => next.context_menu.max_height = parse_u32(value),
        "context_menu.item_kind" => {
            legacy_dod_context_menu_options::set_item_kind(&mut next, value)
        }
        _ => next.theme_id = value.to_string(),
    }
    next
}

fn toggled_bool(before: &str) -> String {
    if before == "true" {
        "false".to_string()
    } else {
        "true".to_string()
    }
}

fn changed_usize(before: &str) -> String {
    if before == "0" {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

fn changed_u8(before: &str) -> String {
    if before == "64" {
        "80".to_string()
    } else {
        "64".to_string()
    }
}

fn parse_usize(value: &str) -> usize {
    value.parse().map_or(INVALID_USIZE_SETTING, |it| it)
}

fn parse_u32(value: &str) -> u32 {
    value.parse().map_or(INVALID_U32_SETTING, |it| it)
}

fn parse_u8(value: &str) -> u8 {
    value.parse().map_or(INVALID_U8_SETTING, |it| it)
}

fn alternate_text(before: &str, configured: &str, replacement: &str) -> String {
    if before == configured {
        replacement.to_string()
    } else {
        configured.to_string()
    }
}

fn animation_state(value: &str) -> UiAnimationState {
    match value {
        "Running" => UiAnimationState::Running,
        "Paused" => UiAnimationState::Paused,
        _ => UiAnimationState::Idle,
    }
}

fn variant(value: &str) -> UiVariant {
    match value {
        "Filled" => UiVariant::Filled,
        "Text" => UiVariant::Text,
        "Icon" => UiVariant::Icon,
        "IconText" => UiVariant::IconText,
        "Outline" => UiVariant::Outline,
        _ => UiVariant::Plain,
    }
}

fn tone(value: &str) -> UiTone {
    match value {
        "Accent" => UiTone::Accent,
        "Success" => UiTone::Success,
        "Warning" => UiTone::Warning,
        "Danger" => UiTone::Danger,
        _ => UiTone::Neutral,
    }
}

fn size(value: &str) -> UiSize {
    match value {
        "x-small" => UiSize::XSmall,
        "small" | "Small" => UiSize::Small,
        "large" | "Large" => UiSize::Large,
        "x-large" | "XLarge" => UiSize::XLarge,
        "medium" | "Medium" => UiSize::Medium,
        _ => UiSize::Medium,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_option_fallbacks_and_remaining_variants_are_total() {
        let mut props = UiProps::new("legacy", "legacy-options".into());
        props.theme_id = "theme".to_string();
        assert_eq!("theme", option_value("unknown", &props));
        assert_eq!(
            "changed",
            props_with_option(&props, "unknown", "changed").theme_id
        );
        assert_eq!(
            UiAnimationState::Idle,
            props_with_option(&props, "loading.animation_state", "unknown")
                .loading_indicator
                .animation_state
        );
        assert_eq!(
            UiVariant::Plain,
            props_with_option(&props, "variant", "unknown").variant
        );
        assert_eq!(
            UiTone::Neutral,
            props_with_option(&props, "tone", "unknown").tone
        );

        assert_eq!("0", changed_usize("1"));
        assert_eq!("64", changed_u8("1"));
        assert_eq!(
            "configured",
            alternate_text("other", "configured", "replacement")
        );
        assert_eq!(UiSize::Large, size("large"));
        assert_eq!(UiSize::XLarge, size("x-large"));
        assert_eq!(UiSize::Medium, size("medium"));
        assert_eq!(UiSize::Medium, size("unknown"));

        let placement = props_with_option(&props, "context_menu.placement", "AboveEnd");
        assert_eq!(
            katana_ui_core::render_model::UiContextMenuPlacement::AboveEnd,
            placement.context_menu.placement_used
        );
    }
}
