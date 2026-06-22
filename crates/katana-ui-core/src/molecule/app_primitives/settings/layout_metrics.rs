use super::SettingsControl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsListLayoutMetrics {
    title_height: u32,
    search_box_height: u32,
    section_height: u32,
    field_height: u32,
    footer_height: u32,
    child_indent: u32,
    field_label_width: u32,
    toggle_width: u32,
    text_entry_width: u32,
    choice_control_width: u32,
}

impl SettingsListLayoutMetrics {
    pub const DEFAULT: Self = Self {
        title_height: 24,
        search_box_height: 20,
        section_height: 22,
        field_height: 22,
        footer_height: 22,
        child_indent: 8,
        field_label_width: 112,
        toggle_width: 48,
        text_entry_width: 132,
        choice_control_width: 132,
    };

    #[must_use]
    pub const fn title_height(self) -> u32 {
        self.title_height
    }

    #[must_use]
    pub const fn search_box_height(self) -> u32 {
        self.search_box_height
    }

    #[must_use]
    pub const fn section_height(self) -> u32 {
        self.section_height
    }

    #[must_use]
    pub const fn field_height(self) -> u32 {
        self.field_height
    }

    #[must_use]
    pub const fn footer_height(self) -> u32 {
        self.footer_height
    }

    #[must_use]
    pub const fn child_indent(self) -> u32 {
        self.child_indent
    }

    #[must_use]
    pub const fn field_label_width(self) -> u32 {
        self.field_label_width
    }

    #[must_use]
    pub const fn field_control_x(self) -> u32 {
        self.child_indent + self.field_label_width
    }

    #[must_use]
    pub const fn toggle_width(self) -> u32 {
        self.toggle_width
    }

    #[must_use]
    pub const fn text_entry_width(self) -> u32 {
        self.text_entry_width
    }

    #[must_use]
    pub const fn choice_control_width(self) -> u32 {
        self.choice_control_width
    }

    #[must_use]
    pub fn control_width(self, control: &SettingsControl) -> u32 {
        match control {
            SettingsControl::Toggle { .. } => self.toggle_width,
            SettingsControl::Radio { .. } => self.choice_control_width,
            SettingsControl::Select { .. }
            | SettingsControl::Combo { .. }
            | SettingsControl::Input { .. }
            | SettingsControl::TextArea { .. }
            | SettingsControl::Number { .. }
            | SettingsControl::Chips { .. }
            | SettingsControl::ColorPicker { .. }
            | SettingsControl::Custom(_) => self.text_entry_width,
        }
    }
}

impl Default for SettingsListLayoutMetrics {
    fn default() -> Self {
        Self::DEFAULT
    }
}
