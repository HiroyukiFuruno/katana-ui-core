use super::{CHAT_SECTION_ID, DEFAULT_CONTROL_OPTION_COUNT, DEFAULT_FIELD_COUNT, FONT_FIELD_ID};
use katana_ui_core::molecule::{
    SettingsControl, SettingsField, SettingsList, SettingsSection, SettingsValue,
};

const DEFAULT_FONT_SIZE: i64 = 14;
const MIN_FONT_SIZE: i64 = 10;
const MAX_FONT_SIZE: i64 = 24;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::visual) struct SettingsListScreenState {
    pub(super) settings: SettingsList,
    pub(super) query_filter: bool,
    pub(super) callback_action: &'static str,
    pub(super) option_state: SettingsListOptionState,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
    pub(in crate::visual) scroll_offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct SettingsListOptionState {
    pub(in crate::visual) label_workspace: bool,
    pub(in crate::visual) density_compact: bool,
    pub(in crate::visual) dirty_highlight: bool,
    pub(in crate::visual) sections_app_lint: bool,
    pub(in crate::visual) section_label_editor: bool,
    pub(in crate::visual) section_description_visible: bool,
    pub(in crate::visual) section_icon_gear: bool,
    pub(in crate::visual) field_count: u8,
    pub(in crate::visual) section_footer_policy: bool,
    pub(in crate::visual) section_collapsible: bool,
    pub(in crate::visual) default_collapsed: bool,
    pub(in crate::visual) field_label_font_size: bool,
    pub(in crate::visual) field_description_visible: bool,
    pub(in crate::visual) control_kind_number: bool,
    pub(in crate::visual) control_option_count: u8,
    pub(in crate::visual) custom_control_button: bool,
    pub(in crate::visual) value_changed: bool,
    pub(in crate::visual) reset_default: bool,
}

impl Default for SettingsListOptionState {
    fn default() -> Self {
        Self {
            label_workspace: false,
            density_compact: false,
            dirty_highlight: false,
            sections_app_lint: false,
            section_label_editor: false,
            section_description_visible: false,
            section_icon_gear: false,
            field_count: DEFAULT_FIELD_COUNT,
            section_footer_policy: false,
            section_collapsible: false,
            default_collapsed: false,
            field_label_font_size: false,
            field_description_visible: false,
            control_kind_number: false,
            control_option_count: DEFAULT_CONTROL_OPTION_COUNT,
            custom_control_button: false,
            value_changed: false,
            reset_default: false,
        }
    }
}

impl Default for SettingsListScreenState {
    fn default() -> Self {
        Self {
            settings: settings_list(),
            query_filter: false,
            callback_action: "none",
            option_state: SettingsListOptionState::default(),
            focused: false,
            hovered: false,
            scroll_offset: 0,
        }
    }
}

fn settings_list() -> SettingsList {
    SettingsList::new("Settings list")
        .section(
            SettingsSection::new("app", "App settings").field(
                SettingsField::new(
                    FONT_FIELD_ID,
                    "Font size",
                    SettingsControl::Number {
                        value: DEFAULT_FONT_SIZE,
                        min: MIN_FONT_SIZE,
                        max: MAX_FONT_SIZE,
                    },
                )
                .reset_to_default(SettingsValue::Number(DEFAULT_FONT_SIZE)),
            ),
        )
        .section(SettingsSection::new(CHAT_SECTION_ID, "Chat settings").collapsible(true))
}
