use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsListAction {
    SetQuery(Option<String>),
    ToggleSection {
        section_id: String,
    },
    KeyboardSection {
        section_id: String,
        input: SettingsKeyboardInput,
    },
    UpdateField {
        field_id: String,
        value: super::SettingsValue,
    },
    ResetField {
        field_id: String,
    },
    RouteChildEvent {
        field_id: String,
        event: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsKeyboardInput {
    Enter,
    Space,
    Tab,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsListEvent {
    QueryChanged(Option<String>),
    FieldChanged { field_id: String },
    FieldReset { field_id: String },
    SectionCollapsed { section_id: String, collapsed: bool },
    ChildEventRouted { field_id: String, event: String },
}
