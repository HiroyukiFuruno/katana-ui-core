use super::{
    UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID, UI_SETTINGS_SECTION_TOGGLE_ACTION_ID,
    UiHostActionPayload, UiHostActionPlan,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSettingsFieldControlTarget {
    pub field_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSettingsSectionToggleTarget {
    pub section_id: String,
}

impl UiHostActionPlan {
    #[must_use]
    pub fn settings_field_control_target(&self) -> Option<UiSettingsFieldControlTarget> {
        UiSettingsFieldControlTarget::from_plan(self)
    }

    #[must_use]
    pub fn settings_section_toggle_target(&self) -> Option<UiSettingsSectionToggleTarget> {
        UiSettingsSectionToggleTarget::from_plan(self)
    }
}

impl UiSettingsFieldControlTarget {
    fn from_plan(plan: &UiHostActionPlan) -> Option<Self> {
        if plan.action_id != UI_SETTINGS_FIELD_ACTIVATE_ACTION_ID {
            return None;
        }
        let UiHostActionPayload::SettingsFieldControl(payload) = &plan.typed_payload else {
            return None;
        };
        Some(Self {
            field_id: payload.field_id.clone(),
        })
    }
}

impl UiSettingsSectionToggleTarget {
    fn from_plan(plan: &UiHostActionPlan) -> Option<Self> {
        if plan.action_id != UI_SETTINGS_SECTION_TOGGLE_ACTION_ID {
            return None;
        }
        let UiHostActionPayload::SettingsSectionToggle(payload) = &plan.typed_payload else {
            return None;
        };
        Some(Self {
            section_id: payload.section_id.clone(),
        })
    }
}
