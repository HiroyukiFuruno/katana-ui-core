use super::{Button, ColorSwatch, IconTextButton, KeyCap, SvgButton, Text, TextButton};
use crate::render_model::{
    UiButtonLayoutDto, UiButtonLayoutPatchDto, UiButtonLayoutPreset, UiButtonLayoutSpec,
};

impl Text {
    #[must_use]
    pub fn text_role(mut self, value: impl Into<String>) -> Self {
        self.state.text.role = value.into();
        self
    }

    #[must_use]
    pub fn text_color_token(mut self, value: impl Into<String>) -> Self {
        self.state.text.color_token = value.into();
        self
    }

    #[must_use]
    pub fn line_metrics(mut self, line_height_px: u16, baseline_offset_px: i16) -> Self {
        self.state.text.line_height_px = line_height_px;
        self.state.text.baseline_offset_px = baseline_offset_px;
        self
    }

    #[must_use]
    pub fn vertical_centered(mut self, value: bool) -> Self {
        self.state.text.vertical_centered = value;
        self
    }
}

impl ColorSwatch {
    #[must_use]
    pub fn palette_color(mut self, value: impl Into<String>) -> Self {
        self.state.color_swatch.palette.push(value.into());
        self
    }

    #[must_use]
    pub fn selected_color(mut self, value: impl Into<String>) -> Self {
        let value = value.into();
        self.state.color_swatch.selected_color = value.clone();
        self.state.interaction.value = value;
        self
    }
}

impl KeyCap {
    #[must_use]
    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.state.shortcut.platform = value.into();
        self
    }

    #[must_use]
    pub fn combo(mut self, value: impl Into<String>) -> Self {
        self.state.shortcut.combo = value.into();
        self
    }
}

macro_rules! button_atom {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn command(mut self, value: impl Into<String>) -> Self {
                self.state.button.command = value.into();
                self
            }

            #[must_use]
            pub fn keyboard_activation(mut self, value: bool) -> Self {
                self.state.button.keyboard_activation = value;
                self
            }

            #[must_use]
            pub fn icon_position(mut self, value: impl Into<String>) -> Self {
                self.state.button.icon_position = value.into();
                self
            }

            #[must_use]
            pub fn layout_preset(mut self, value: UiButtonLayoutPreset) -> Self {
                self.state.button.layout = value.to_dto();
                self
            }

            #[must_use]
            pub fn layout(mut self, value: UiButtonLayoutDto) -> Self {
                self.state.button.layout = value;
                self
            }

            #[must_use]
            pub fn layout_spec(mut self, value: impl Into<UiButtonLayoutSpec>) -> Self {
                self.state.button.layout = value.into().resolve();
                self
            }

            #[must_use]
            pub fn layout_patch(
                mut self,
                preset: UiButtonLayoutPreset,
                patch: UiButtonLayoutPatchDto,
            ) -> Self {
                self.state.button.layout =
                    UiButtonLayoutSpec::preset_patch(preset, patch).resolve();
                self
            }

            #[must_use]
            pub fn layout_from_preset(
                mut self,
                value: UiButtonLayoutPreset,
                customize: impl FnOnce(UiButtonLayoutDto) -> UiButtonLayoutDto,
            ) -> Self {
                self.state.button.layout = customize(value.to_dto());
                self
            }
        }
    };
}

button_atom!(Button);
button_atom!(SvgButton);
button_atom!(TextButton);
button_atom!(IconTextButton);
