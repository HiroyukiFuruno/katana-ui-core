use super::{
    SettingsKeyboardInput, SettingsList, SettingsListAction, SettingsListEvent, SettingsValue,
};
use crate::render_model::UiInteractionState;

pub(super) fn apply(list: &mut SettingsList, action: SettingsListAction) -> Vec<SettingsListEvent> {
    let event = match action {
        SettingsListAction::SetQuery(value) => {
            list.query = value.clone().filter(|it| !it.is_empty());
            SettingsListEvent::QueryChanged(list.query.clone())
        }
        SettingsListAction::ToggleSection { section_id } => toggle_section(list, section_id),
        SettingsListAction::KeyboardSection { section_id, input } => {
            return keyboard_section(list, section_id, input);
        }
        SettingsListAction::KeyboardField { field_id, input } => {
            return keyboard_field(list, field_id, input);
        }
        SettingsListAction::FocusField { field_id } => focus_field(list, field_id),
        SettingsListAction::UpdateField { field_id, value } => update_field(list, field_id, value),
        SettingsListAction::ResetField { field_id } => reset_field(list, field_id),
        SettingsListAction::RouteChildEvent { field_id, event } => {
            SettingsListEvent::ChildEventRouted { field_id, event }
        }
    };
    record_event(list, event)
}

fn keyboard_section(
    list: &mut SettingsList,
    section_id: String,
    input: SettingsKeyboardInput,
) -> Vec<SettingsListEvent> {
    if matches!(
        input,
        SettingsKeyboardInput::Enter | SettingsKeyboardInput::Space
    ) {
        return apply(list, SettingsListAction::ToggleSection { section_id });
    }
    Vec::new()
}

fn keyboard_field(
    list: &mut SettingsList,
    field_id: String,
    input: SettingsKeyboardInput,
) -> Vec<SettingsListEvent> {
    if input == SettingsKeyboardInput::Tab {
        return apply(
            list,
            SettingsListAction::FocusField {
                field_id: next_visible_field_id(list, &field_id),
            },
        );
    }
    Vec::new()
}

fn focus_field(list: &mut SettingsList, field_id: Option<String>) -> SettingsListEvent {
    list.focused_field_id = field_id.filter(|id| field_exists(list, id));
    SettingsListEvent::FieldFocused {
        field_id: list.focused_field_id.clone(),
    }
}

fn toggle_section(list: &mut SettingsList, section_id: String) -> SettingsListEvent {
    let collapsible = list
        .sections
        .iter()
        .find(|section| section.id == section_id)
        .is_some_and(|section| section.collapsible);
    let collapsed = if collapsible && !list.collapsed_section_ids.remove(&section_id) {
        list.collapsed_section_ids.insert(section_id.clone());
        true
    } else {
        false
    };
    SettingsListEvent::SectionCollapsed {
        section_id,
        collapsed,
    }
}

fn update_field(
    list: &mut SettingsList,
    field_id: String,
    value: SettingsValue,
) -> SettingsListEvent {
    if let Some(field) = list
        .sections
        .iter_mut()
        .flat_map(|section| section.fields.iter_mut())
        .find(|field| field.id == field_id)
    {
        let _ = field.control.set_value(value);
        sync_dirty_field(&mut list.dirty_field_ids, field);
    }
    SettingsListEvent::FieldChanged { field_id }
}

fn reset_field(list: &mut SettingsList, field_id: String) -> SettingsListEvent {
    if let Some(field) = list
        .sections
        .iter_mut()
        .flat_map(|section| section.fields.iter_mut())
        .find(|field| field.id == field_id)
        && let Some(value) = field.reset_to_default.clone()
    {
        let _ = field.control.set_value(value);
        sync_dirty_field(&mut list.dirty_field_ids, field);
    }
    SettingsListEvent::FieldReset { field_id }
}

fn record_event(list: &mut SettingsList, event: SettingsListEvent) -> Vec<SettingsListEvent> {
    list.last_event = Some(event.clone());
    list.callback_log.push(event.clone());
    vec![event]
}

fn sync_dirty_field(
    dirty_field_ids: &mut std::collections::BTreeSet<String>,
    field: &super::SettingsField,
) {
    if field.is_dirty() {
        dirty_field_ids.insert(field.id.clone());
    } else {
        dirty_field_ids.remove(&field.id);
    }
}

pub(super) fn interaction_state(value: &SettingsList) -> UiInteractionState {
    let visible_field_ids = visible_field_ids(value);
    UiInteractionState {
        value: value.query.clone().unwrap_or_default(),
        item_count: visible_field_ids.len(),
        open: value.collapsed_section_ids.is_empty(),
        has_selection: !value.dirty_field_ids.is_empty(),
        focused: value.focused_field_id.is_some(),
        selected_index: value
            .focused_field_id
            .as_ref()
            .and_then(|id| visible_field_ids.iter().position(|it| it == id))
            .unwrap_or_default(),
        ..UiInteractionState::default()
    }
}

fn next_visible_field_id(list: &SettingsList, current_field_id: &str) -> Option<String> {
    let ids = visible_field_ids(list);
    if ids.is_empty() {
        return None;
    }
    let next_index = ids
        .iter()
        .position(|id| id == current_field_id)
        .map_or(0, |index| (index + 1) % ids.len());
    Some(ids[next_index].clone())
}

fn visible_field_ids(list: &SettingsList) -> Vec<String> {
    list.visible_fields()
        .into_iter()
        .map(|field| field.id.clone())
        .collect()
}

fn field_exists(list: &SettingsList, field_id: &str) -> bool {
    list.sections
        .iter()
        .flat_map(|section| section.fields.iter())
        .any(|field| field.id == field_id)
}
