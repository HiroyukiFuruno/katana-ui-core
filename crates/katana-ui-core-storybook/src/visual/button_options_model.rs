use super::CONTROL_COUNT;
#[path = "button_options_values.rs"]
mod values;
pub(in crate::visual) use values::{
    StorybookButtonCommandMode, StorybookButtonHeightMode, StorybookButtonIconPosition,
    StorybookButtonLayoutPreset, StorybookButtonTabIndex, StorybookButtonWidthMode,
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
    pub(in crate::visual) command_mode: StorybookButtonCommandMode,
    pub(in crate::visual) keyboard_activation: bool,
    pub(in crate::visual) icon_position: StorybookButtonIconPosition,
    pub(in crate::visual) layout_preset: StorybookButtonLayoutPreset,
    pub(in crate::visual) external_svg_source: bool,
    pub(in crate::visual) aria_label: bool,
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
            command_mode: StorybookButtonCommandMode::Save,
            keyboard_activation: true,
            icon_position: StorybookButtonIconPosition::Leading,
            layout_preset: StorybookButtonLayoutPreset::Page,
            external_svg_source: false,
            aria_label: false,
        }
    }
}

impl StorybookButtonOptions {
    pub(in crate::visual) fn apply_contract_after(
        &mut self,
        control: StorybookButtonOptionControl,
    ) {
        match control {
            StorybookButtonOptionControl::Visible => self.visible = false,
            StorybookButtonOptionControl::Disabled => self.disabled = true,
            StorybookButtonOptionControl::Focusable => self.focusable = false,
            StorybookButtonOptionControl::Border => self.border = false,
            StorybookButtonOptionControl::Label => self.japanese_label = true,
            StorybookButtonOptionControl::Width => {
                self.width_mode = StorybookButtonWidthMode::Auto.next();
            }
            StorybookButtonOptionControl::Height => {
                self.height_mode = StorybookButtonHeightMode::Auto.next();
            }
            StorybookButtonOptionControl::TabIndex => {
                self.tab_index = StorybookButtonTabIndex::Zero.next();
            }
            StorybookButtonOptionControl::ZIndex => {
                self.z_index = StorybookButtonZIndex::Auto.next();
            }
            StorybookButtonOptionControl::Command => {
                self.command_mode = StorybookButtonCommandMode::Save.next();
            }
            StorybookButtonOptionControl::KeyboardActivation => {
                self.keyboard_activation = false;
            }
            StorybookButtonOptionControl::IconPosition => {
                self.icon_position = StorybookButtonIconPosition::Leading.next();
            }
            StorybookButtonOptionControl::LayoutPreset => {
                self.layout_preset = StorybookButtonLayoutPreset::Page.next();
            }
            StorybookButtonOptionControl::SvgSource => {
                self.external_svg_source = true;
            }
            StorybookButtonOptionControl::AriaLabel => {
                self.aria_label = true;
            }
        }
    }

    pub(in crate::visual) fn label(self, fallback: &'static str) -> &'static str {
        if self.japanese_label {
            "保存する"
        } else if matches!(self.command_mode, StorybookButtonCommandMode::Open) {
            "Open command"
        } else {
            fallback
        }
    }

    pub(in crate::visual) const fn icon_trailing(self) -> bool {
        matches!(self.icon_position, StorybookButtonIconPosition::Trailing)
    }

    pub(in crate::visual) const fn effective_preset_index(self, fallback: usize) -> usize {
        self.layout_preset.preset_index(fallback)
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
    Command,
    KeyboardActivation,
    IconPosition,
    LayoutPreset,
    SvgSource,
    AriaLabel,
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
            Self::Command,
            Self::KeyboardActivation,
            Self::IconPosition,
            Self::LayoutPreset,
            Self::SvgSource,
            Self::AriaLabel,
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
            Self::Command => "button.command",
            Self::KeyboardActivation => "button.keyboard_activation",
            Self::IconPosition => "button.icon_position",
            Self::LayoutPreset => "button.layout_preset",
            Self::SvgSource => "button.svg_source",
            Self::AriaLabel => "button.aria_label",
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
            Self::Command => options.command_mode.label(),
            Self::KeyboardActivation if options.keyboard_activation => "true",
            Self::KeyboardActivation => "false",
            Self::IconPosition => options.icon_position.label(),
            Self::LayoutPreset => options.layout_preset.label(),
            Self::SvgSource if options.external_svg_source => "custom-svg",
            Self::SvgSource => "default-svg",
            Self::AriaLabel if options.aria_label => "Close panel",
            Self::AriaLabel => "Svg action",
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
            Self::Command => command_state(options.command_mode),
            Self::KeyboardActivation if options.keyboard_activation => "keyboard=true",
            Self::KeyboardActivation => "keyboard=false",
            Self::IconPosition => icon_position_state(options.icon_position),
            Self::LayoutPreset => layout_preset_state(options.layout_preset),
            Self::SvgSource if options.external_svg_source => "svg_source=custom",
            Self::SvgSource => "svg_source=default",
            Self::AriaLabel if options.aria_label => "aria_label=custom",
            Self::AriaLabel => "aria_label=default",
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

const fn command_state(command: StorybookButtonCommandMode) -> &'static str {
    match command {
        StorybookButtonCommandMode::Save => "command=save",
        StorybookButtonCommandMode::Open => "command=open",
    }
}

const fn icon_position_state(position: StorybookButtonIconPosition) -> &'static str {
    match position {
        StorybookButtonIconPosition::Leading => "icon=leading",
        StorybookButtonIconPosition::Trailing => "icon=trailing",
    }
}

const fn layout_preset_state(layout: StorybookButtonLayoutPreset) -> &'static str {
    match layout {
        StorybookButtonLayoutPreset::Page => "layout=page",
        StorybookButtonLayoutPreset::Dense => "layout=dense",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_button_option_reports_before_and_after_contract_values() {
        for control in StorybookButtonOptionControl::all() {
            let before = StorybookButtonOptions::default();
            assert!(!control.setting_name().is_empty());
            assert!(!control.setting_value(before).is_empty());
            assert!(!control.state_label(before).is_empty());

            let mut after = before;
            after.apply_contract_after(control);
            assert!(!control.setting_value(after).is_empty());
            assert!(!control.state_label(after).is_empty());
        }
    }

    #[test]
    fn button_option_value_cycles_cover_every_enum_variant() {
        for width in [
            StorybookButtonWidthMode::Auto,
            StorybookButtonWidthMode::Px,
            StorybookButtonWidthMode::Percent,
            StorybookButtonWidthMode::Fill,
        ] {
            let options = StorybookButtonOptions {
                width_mode: width,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::Width
                    .state_label(options)
                    .is_empty()
            );
            assert!(!width.label().is_empty());
            let _ = width.next();
        }
        for height in [
            StorybookButtonHeightMode::Auto,
            StorybookButtonHeightMode::Compact,
            StorybookButtonHeightMode::Tall,
        ] {
            let options = StorybookButtonOptions {
                height_mode: height,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::Height
                    .state_label(options)
                    .is_empty()
            );
            assert!(!height.label().is_empty());
            let _ = height.next();
        }
        for tab_index in [
            StorybookButtonTabIndex::Zero,
            StorybookButtonTabIndex::One,
            StorybookButtonTabIndex::Disabled,
        ] {
            let options = StorybookButtonOptions {
                tab_index,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::TabIndex
                    .state_label(options)
                    .is_empty()
            );
            assert!(!tab_index.label().is_empty());
            let _ = tab_index.next();
        }
        for z_index in [
            StorybookButtonZIndex::Auto,
            StorybookButtonZIndex::Raised,
            StorybookButtonZIndex::Overlay,
        ] {
            let options = StorybookButtonOptions {
                z_index,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::ZIndex
                    .state_label(options)
                    .is_empty()
            );
            assert!(!z_index.label().is_empty());
            let _ = z_index.next();
        }
        for command in [
            StorybookButtonCommandMode::Save,
            StorybookButtonCommandMode::Open,
        ] {
            let options = StorybookButtonOptions {
                command_mode: command,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::Command
                    .state_label(options)
                    .is_empty()
            );
            assert!(!command.label().is_empty());
            let _ = command.next();
        }
        for position in [
            StorybookButtonIconPosition::Leading,
            StorybookButtonIconPosition::Trailing,
        ] {
            let options = StorybookButtonOptions {
                icon_position: position,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::IconPosition
                    .state_label(options)
                    .is_empty()
            );
            assert!(!position.label().is_empty());
            let _ = position.next();
        }
        for layout in [
            StorybookButtonLayoutPreset::Page,
            StorybookButtonLayoutPreset::Dense,
        ] {
            let options = StorybookButtonOptions {
                layout_preset: layout,
                ..StorybookButtonOptions::default()
            };
            assert!(
                !StorybookButtonOptionControl::LayoutPreset
                    .state_label(options)
                    .is_empty()
            );
            assert!(!layout.label().is_empty());
            let _ = layout.next();
            let _ = layout.preset_index(2);
        }
    }

    #[test]
    fn button_option_helpers_cover_label_icon_and_preset_resolution() {
        let fallback = StorybookButtonOptions::default();
        assert_eq!("Fallback", fallback.label("Fallback"));
        assert!(!fallback.icon_trailing());
        assert_eq!(2, fallback.effective_preset_index(2));

        let open = StorybookButtonOptions {
            command_mode: StorybookButtonCommandMode::Open,
            ..fallback
        };
        assert_eq!("Open command", open.label("Fallback"));

        let japanese = StorybookButtonOptions {
            japanese_label: true,
            icon_position: StorybookButtonIconPosition::Trailing,
            layout_preset: StorybookButtonLayoutPreset::Dense,
            ..fallback
        };
        assert_eq!("保存する", japanese.label("Fallback"));
        assert!(japanese.icon_trailing());
        assert_eq!(3, japanese.effective_preset_index(2));
    }
}
