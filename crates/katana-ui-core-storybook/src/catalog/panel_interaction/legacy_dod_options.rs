use katana_ui_core::render_model::{
    UiAnimationState, UiContextMenuAnchor, UiProps, UiSize, UiTone, UiVariant,
};

const CONTEXT_MENU_POINTER_VALUE: &str = "Pointer(192,128)";
const CONTEXT_MENU_POINTER_X: i32 = 192;
const CONTEXT_MENU_POINTER_Y: i32 = 128;
const INVALID_USIZE_SETTING: usize = 0;
const INVALID_U8_SETTING: u8 = 0;

pub(super) fn option_value(option: &str, props: &UiProps) -> String {
    match option {
        "theme_id" => props.theme_id.clone(),
        "text.role" => props.text.role.clone(),
        "icon.svg_source" => props.icon.svg_source.clone(),
        "loading.reduced_motion" => props.loading_indicator.reduced_motion.to_string(),
        "loading.animation_state" => format!("{:?}", props.loading_indicator.animation_state),
        "progress.percent" => props.progress_percent.to_string(),
        "variant" => format!("{:?}", props.variant),
        "tone" => format!("{:?}", props.tone),
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
        "context_menu.anchor" => context_menu_anchor_value(&props.context_menu.anchor),
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
        "variant" => next.variant = variant(value),
        "tone" => next.tone = tone(value),
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
        "context_menu.anchor" => next.context_menu.anchor = context_menu_anchor(value),
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
        "small" => UiSize::Small,
        "large" => UiSize::Large,
        "x-large" => UiSize::XLarge,
        _ => UiSize::Medium,
    }
}

fn context_menu_anchor(value: &str) -> UiContextMenuAnchor {
    if value == CONTEXT_MENU_POINTER_VALUE {
        return UiContextMenuAnchor::Pointer {
            x: CONTEXT_MENU_POINTER_X,
            y: CONTEXT_MENU_POINTER_Y,
        };
    }
    UiContextMenuAnchor::Pointer { x: 0, y: 0 }
}

fn context_menu_anchor_value(anchor: &UiContextMenuAnchor) -> String {
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
