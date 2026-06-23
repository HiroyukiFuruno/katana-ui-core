use super::{
    SettingsControl, SettingsDirtyVisualization, SettingsField, SettingsList, SettingsListDensity,
    SettingsSection, SettingsValue, state,
};
use crate::atom::{Button, Chip, Icon, Input, Radio, Text, TextArea, Toggle};
use crate::molecule::{
    ChipGroup, ColorPicker, ComboBox, EmptyState, FormField, RgbaColor, SearchBox, SelectBox,
};
use crate::render_model::{UiCommonProps, UiHostActionSpec, UiInteractivePreset};
use crate::render_model::{UiNode, UiNodeId, UiNodeKind, UiSize, UiStateId, UiVariant};

const DEFAULT_CHIP_WIDTH: u16 = 64;

pub(super) fn render(value: SettingsList) -> UiNode {
    let mut node = UiNode::from_state(
        UiNodeKind::SettingsList,
        value.label.clone(),
        value.state_id.clone(),
    )
    .interaction(state::interaction_state(&value))
    .size(size(value.density))
    .variant(variant(value.dirty_visualization))
    .style_class(density_class(value.density))
    .child(
        SearchBox::new("Settings query")
            .placeholder("Search settings")
            .value(value.query.clone().unwrap_or_default()),
    );
    let visible_sections = value.visible_sections();
    if visible_sections
        .iter()
        .all(|section| section.fields.is_empty())
    {
        return node.child(EmptyState::new("No matching settings").body("No settings match query"));
    }
    for visible in visible_sections {
        node = node.child(render_section_header(visible.section));
        for field in visible.fields {
            node = node.child(render_field(
                &value,
                field,
                value.dirty_field_ids.contains(&field.id),
            ));
        }
        if let Some(footer) = &visible.section.footer {
            node = node.child(Text::new(footer.clone()));
        }
    }
    node
}

fn render_section_header(section: &SettingsSection) -> UiNode {
    let section_id = SettingsList::section_interaction_id(&section.id);
    let mut header = UiNode::new(UiNodeKind::Panel, section.label.clone())
        .common(interactive_common())
        .stable_node_id(UiNodeId::new(section_id.clone()))
        .stable_state_id(UiStateId::new(section_id))
        .host_action(UiHostActionSpec::settings_section_toggle(
            section.label.clone(),
            section.id.clone(),
        ));
    if let Some(icon) = &section.icon {
        header = header.child(Icon::new(icon.clone()));
    }
    header.child(Text::new(section.label.clone()))
}

fn render_field(list: &SettingsList, field: &SettingsField, dirty: bool) -> UiNode {
    let field_id = SettingsList::field_interaction_id(&field.id);
    let mut form = FormField::new(field.label.clone())
        .common(interactive_common())
        .stable_node_id(UiNodeId::new(field_id.clone()))
        .stable_state_id(UiStateId::new(field_id))
        .child(render_control(list, field));
    if list.activation_action_for_field(&field.id).is_some() {
        form = form.host_action(UiHostActionSpec::settings_field_control(
            field.label.clone(),
            field.id.clone(),
        ));
    }
    if let Some(description) = &field.description {
        form = form.child(Text::new(description.clone()));
    }
    if dirty {
        form = form
            .child(Icon::new("dirty-marker"))
            .child(Button::new("Reset"));
    }
    form.into()
}

fn render_control(list: &SettingsList, field: &SettingsField) -> UiNode {
    let control_id = SettingsList::control_interaction_id(&field.id);
    let mut node: UiNode = match &field.control {
        SettingsControl::Toggle { checked } => Toggle::new(field.label.clone())
            .stable_state_id(control_id.clone())
            .checked(*checked)
            .into(),
        SettingsControl::Select { options, selected } => {
            let mut select = SelectBox::new(field.label.clone()).value(selected.clone());
            for option in options {
                select = select.child(Text::new(option.label.clone()));
            }
            select.into()
        }
        SettingsControl::Combo {
            options,
            query,
            selected,
        } => {
            let mut combo = ComboBox::new(field.label.clone()).input_value(query.clone());
            if let Some(value) = selected {
                combo = combo.value(value.clone());
            }
            for option in options {
                combo = combo.child(Text::new(option.label.clone()));
            }
            combo.into()
        }
        SettingsControl::Input { value } => Input::new(field.label.clone())
            .stable_state_id(control_id.clone())
            .value(value.clone())
            .into(),
        SettingsControl::TextArea { value } => TextArea::new(field.label.clone())
            .stable_state_id(control_id.clone())
            .value(value.clone())
            .into(),
        SettingsControl::Number { value, .. } => Input::new(field.label.clone())
            .stable_state_id(control_id.clone())
            .value(value.to_string())
            .into(),
        SettingsControl::Chips { values } => render_chips(field, values),
        SettingsControl::Radio { options, selected } => {
            let label = options
                .iter()
                .find(|option| option.value == *selected)
                .map_or_else(|| field.label.clone(), |option| option.label.clone());
            Radio::new(label).selected(true).into()
        }
        SettingsControl::ColorPicker { color } => render_color(field, color),
        SettingsControl::Custom(node) => node.as_ref().clone(),
    };
    node = node.stable_node_id(SettingsList::control_node_id(&field.id));
    if list.activation_action_for_field(&field.id).is_some() {
        node = node.host_action(UiHostActionSpec::settings_field_control(
            field.label.clone(),
            field.id.clone(),
        ));
    }
    node
}

fn render_chips(field: &SettingsField, values: &[String]) -> UiNode {
    let mut group = ChipGroup::new(field.label.clone());
    for value in values {
        group = group.chip(Chip::new(value.clone()), DEFAULT_CHIP_WIDTH);
    }
    group.into()
}

fn render_color(field: &SettingsField, value: &SettingsValue) -> UiNode {
    let color = match value {
        SettingsValue::Color {
            red,
            green,
            blue,
            alpha,
        } => RgbaColor::new(*red, *green, *blue, *alpha),
        _ => RgbaColor::new(0, 0, 0, 0),
    };
    ColorPicker::new(field.label.clone())
        .rgba(color)
        .child(Text::new(color.css_rgba()))
        .into()
}

fn variant(value: SettingsDirtyVisualization) -> UiVariant {
    match value {
        SettingsDirtyVisualization::None => UiVariant::Plain,
        SettingsDirtyVisualization::Marker => UiVariant::Outline,
        SettingsDirtyVisualization::Highlight => UiVariant::Filled,
    }
}

fn size(value: SettingsListDensity) -> UiSize {
    match value {
        SettingsListDensity::Compact => UiSize::Small,
        SettingsListDensity::Default => UiSize::Medium,
        SettingsListDensity::Spacious => UiSize::Large,
    }
}

fn density_class(value: SettingsListDensity) -> &'static str {
    match value {
        SettingsListDensity::Compact => "settings-density-compact",
        SettingsListDensity::Default => "settings-density-default",
        SettingsListDensity::Spacious => "settings-density-spacious",
    }
}

fn interactive_common() -> UiCommonProps {
    UiInteractivePreset::control().apply_to_common_defaults(UiCommonProps::default())
}
