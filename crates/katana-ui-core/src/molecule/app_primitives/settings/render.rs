use super::{
    SettingsControl, SettingsDirtyVisualization, SettingsField, SettingsList, SettingsValue, state,
};
use crate::atom::{Button, Chip, Icon, Input, Radio, Text, TextArea, Toggle};
use crate::molecule::{
    ChipGroup, ColorPicker, ComboBox, EmptyState, FormField, RgbaColor, SearchBox, SelectBox,
};
use crate::render_model::{UiNode, UiNodeKind, UiVariant};

const DEFAULT_CHIP_WIDTH: u16 = 64;

pub(super) fn render(value: SettingsList) -> UiNode {
    let mut node = UiNode::from_state(
        UiNodeKind::SettingsList,
        value.label.clone(),
        value.state_id.clone(),
    )
    .interaction(state::interaction_state(&value))
    .variant(variant(value.dirty_visualization))
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
        node = node.child(Text::new(visible.section.label.clone()));
        for field in visible.fields {
            node = node.child(render_field(
                field,
                value.dirty_field_ids.contains(&field.id),
            ));
        }
    }
    node
}

fn render_field(field: &SettingsField, dirty: bool) -> UiNode {
    let mut form = FormField::new(field.label.clone()).child(render_control(field));
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

fn render_control(field: &SettingsField) -> UiNode {
    match &field.control {
        SettingsControl::Toggle { checked } => {
            Toggle::new(field.label.clone()).checked(*checked).into()
        }
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
        SettingsControl::Input { value } => {
            Input::new(field.label.clone()).value(value.clone()).into()
        }
        SettingsControl::TextArea { value } => TextArea::new(field.label.clone())
            .value(value.clone())
            .into(),
        SettingsControl::Number { value, .. } => Input::new(field.label.clone())
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
    }
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
