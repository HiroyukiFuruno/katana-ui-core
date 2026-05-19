use super::CONTROL_COUNT;
#[path = "button_options_values.rs"]
mod values;
pub(in crate::visual) use values::{
    StorybookButtonHeightMode, StorybookButtonTabIndex, StorybookButtonWidthMode,
    StorybookButtonZIndex,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct StorybookButtonOptions {
    pub(in crate::visual) visible: bool,
    pub(in crate::visual) disabled: bool,
    pub(in crate::visual) focusable: bool,
    pub(in crate::visual) border: bool,
    pub(in crate::visual) japanese_label: bool,
    pub(in crate::visual) width_mode: StorybookButtonWidthMode,
    pub(in crate::visual) height_mode: StorybookButtonHeightMode,
    pub(in crate::visual) tab_index: StorybookButtonTabIndex,
    pub(in crate::visual) z_index: StorybookButtonZIndex,
}

impl Default for StorybookButtonOptions {
    fn default() -> Self {
        Self {
            visible: true,
            disabled: false,
            focusable: true,
            border: true,
            japanese_label: false,
            width_mode: StorybookButtonWidthMode::Auto,
            height_mode: StorybookButtonHeightMode::Auto,
            tab_index: StorybookButtonTabIndex::Zero,
            z_index: StorybookButtonZIndex::Auto,
        }
    }
}

impl StorybookButtonOptions {
    pub(in crate::visual) fn toggle(&mut self, control: StorybookButtonOptionControl) {
        match control {
            StorybookButtonOptionControl::Visible => self.visible = !self.visible,
            StorybookButtonOptionControl::Disabled => self.disabled = !self.disabled,
            StorybookButtonOptionControl::Focusable => self.focusable = !self.focusable,
            StorybookButtonOptionControl::Border => self.border = !self.border,
            StorybookButtonOptionControl::Label => self.japanese_label = !self.japanese_label,
            StorybookButtonOptionControl::Width => self.width_mode = self.width_mode.next(),
            StorybookButtonOptionControl::Height => self.height_mode = self.height_mode.next(),
            StorybookButtonOptionControl::TabIndex => self.tab_index = self.tab_index.next(),
            StorybookButtonOptionControl::ZIndex => self.z_index = self.z_index.next(),
        }
    }

    pub(in crate::visual) fn label(self, fallback: &'static str) -> &'static str {
        if self.japanese_label {
            "保存する"
        } else {
            fallback
        }
    }

    pub(in crate::visual) fn compact_props_label(self) -> String {
        format!(
            "w:{} h:{}",
            self.width_mode.label(),
            self.height_mode.label()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum StorybookButtonOptionControl {
    Visible,
    Disabled,
    Focusable,
    Border,
    Label,
    Width,
    Height,
    TabIndex,
    ZIndex,
}

impl StorybookButtonOptionControl {
    pub(in crate::visual) const fn all() -> [Self; CONTROL_COUNT] {
        [
            Self::Visible,
            Self::Disabled,
            Self::Focusable,
            Self::Width,
            Self::Height,
            Self::Border,
            Self::Label,
            Self::TabIndex,
            Self::ZIndex,
        ]
    }

    pub(in crate::visual) const fn setting_name(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Disabled => "disabled",
            Self::Focusable => "focusable",
            Self::Border => "border",
            Self::Label => "label",
            Self::Width => "width",
            Self::Height => "height",
            Self::TabIndex => "tab-index",
            Self::ZIndex => "z-index",
        }
    }

    pub(in crate::visual) fn setting_value(self, options: StorybookButtonOptions) -> &'static str {
        match self {
            Self::Visible if options.visible => "true",
            Self::Visible => "false",
            Self::Disabled if options.disabled => "true",
            Self::Disabled => "false",
            Self::Focusable if options.focusable => "true",
            Self::Focusable => "false",
            Self::Border if options.border => "visible",
            Self::Border => "hidden",
            Self::Label if options.japanese_label => "保存する",
            Self::Label => "Save changes",
            Self::Width => options.width_mode.label(),
            Self::Height => options.height_mode.label(),
            Self::TabIndex => options.tab_index.label(),
            Self::ZIndex => options.z_index.label(),
        }
    }

    pub(in crate::visual) fn state_label(self, options: StorybookButtonOptions) -> &'static str {
        match self {
            Self::Visible if options.visible => "visible=true",
            Self::Visible => "visible=false",
            Self::Disabled if options.disabled => "disabled=true",
            Self::Disabled => "disabled=false",
            Self::Focusable if options.focusable => "focusable=true",
            Self::Focusable => "focusable=false",
            Self::Border if options.border => "border=true",
            Self::Border => "border=false",
            Self::Label if options.japanese_label => "label=ja",
            Self::Label => "label=en",
            Self::Width => width_state(options.width_mode),
            Self::Height => height_state(options.height_mode),
            Self::TabIndex => tab_index_state(options.tab_index),
            Self::ZIndex => z_index_state(options.z_index),
        }
    }
}

const fn width_state(width: StorybookButtonWidthMode) -> &'static str {
    match width {
        StorybookButtonWidthMode::Auto => "width=auto",
        StorybookButtonWidthMode::Px => "width=px",
        StorybookButtonWidthMode::Percent => "width=percent",
        StorybookButtonWidthMode::Fill => "width=fill",
    }
}

const fn height_state(height: StorybookButtonHeightMode) -> &'static str {
    match height {
        StorybookButtonHeightMode::Auto => "height=auto",
        StorybookButtonHeightMode::Compact => "height=compact",
        StorybookButtonHeightMode::Tall => "height=tall",
    }
}

const fn tab_index_state(tab_index: StorybookButtonTabIndex) -> &'static str {
    match tab_index {
        StorybookButtonTabIndex::Zero => "tab-index=0",
        StorybookButtonTabIndex::One => "tab-index=1",
        StorybookButtonTabIndex::Disabled => "tab-index=-1",
    }
}

const fn z_index_state(z_index: StorybookButtonZIndex) -> &'static str {
    match z_index {
        StorybookButtonZIndex::Auto => "z-index=auto",
        StorybookButtonZIndex::Raised => "z-index=10",
        StorybookButtonZIndex::Overlay => "z-index=100",
    }
}
