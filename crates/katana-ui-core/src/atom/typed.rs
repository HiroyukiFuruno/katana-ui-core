use super::{Badge, Icon, Input, LoadingDots, ProgressBar, Spinner, SvgButton};
use crate::render_model::{
    UiAnimationState, UiClearActionSpec, UiDismissAction, UiIconProps, UiSlotPlacement, UiSlotSpec,
    UiSvgPaintPolicy,
};

impl Input {
    #[must_use]
    pub fn leading_slot(mut self, label: impl Into<String>) -> Self {
        self.state.text_entry.leading_slot = Some(UiSlotSpec::new(UiSlotPlacement::Leading, label));
        self
    }

    #[must_use]
    pub fn trailing_slot(mut self, label: impl Into<String>) -> Self {
        self.state.text_entry.trailing_slot =
            Some(UiSlotSpec::new(UiSlotPlacement::Trailing, label));
        self
    }

    #[must_use]
    pub fn clear_action(mut self, label: impl Into<String>) -> Self {
        self.state.text_entry.clear_action = Some(UiClearActionSpec::new(label));
        self
    }

    #[must_use]
    pub fn submit_on_enter(mut self, value: bool) -> Self {
        self.state.text_entry.submit_on_enter = value;
        self
    }

    #[must_use]
    pub fn ime_enabled(mut self, value: bool) -> Self {
        self.state.text_entry.ime_enabled = value;
        self
    }

    #[must_use]
    pub fn emoji_enabled(mut self, value: bool) -> Self {
        self.state.text_entry.emoji_enabled = value;
        self
    }
}

impl Badge {
    #[must_use]
    pub fn dismiss_action(mut self, value: UiDismissAction) -> Self {
        self.state.status.dismiss_action = value;
        self
    }

    #[must_use]
    pub fn leading_icon(mut self, value: impl Into<String>) -> Self {
        self.state.status.leading_icon = value.into();
        self
    }
}

macro_rules! svg_icon_atom {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn svg_source(mut self, value: impl Into<String>) -> Self {
                self.state.icon.svg_source = value.into();
                self
            }

            #[must_use]
            pub fn svg_icon(mut self, value: UiIconProps) -> Self {
                self.state.icon = value;
                self
            }

            #[must_use]
            pub fn icon_view_box(mut self, value: impl Into<String>) -> Self {
                self.state.icon.view_box = value.into();
                self
            }

            #[must_use]
            pub fn icon_path_summary(mut self, value: impl Into<String>) -> Self {
                self.state.icon.path_summary = value.into();
                self
            }

            #[must_use]
            pub fn icon_paint_policy(mut self, value: UiSvgPaintPolicy) -> Self {
                self.state.icon.paint_policy = value;
                self
            }

            #[must_use]
            pub fn icon_role(mut self, value: impl Into<String>) -> Self {
                self.state.icon.role = value.into();
                self
            }

            #[must_use]
            pub fn icon_color_token(mut self, value: impl Into<String>) -> Self {
                self.state.icon.color_token = value.into();
                self
            }

            #[must_use]
            pub fn icon_theme_token(mut self, value: impl Into<String>) -> Self {
                self.state.icon.theme_token = value.into();
                self
            }
        }
    };
}

svg_icon_atom!(Icon);
svg_icon_atom!(SvgButton);

macro_rules! loading_atom {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn animation_state(mut self, value: UiAnimationState) -> Self {
                self.state.loading_indicator.animation_state = value;
                self
            }

            #[must_use]
            pub fn loading_label(mut self, value: impl Into<String>) -> Self {
                self.state.loading_indicator.label = value.into();
                self
            }

            #[must_use]
            pub fn speed_ms(mut self, value: u16) -> Self {
                self.state.loading_indicator.speed_ms = value;
                self
            }

            #[must_use]
            pub fn dot_count(mut self, value: u8) -> Self {
                self.state.loading_indicator.dot_count = value;
                self
            }

            #[must_use]
            pub fn reduced_motion(mut self, value: bool) -> Self {
                self.state.loading_indicator.reduced_motion = value;
                self.state.interaction.reduced_motion = value;
                self
            }
        }
    };
}

loading_atom!(LoadingDots);
loading_atom!(ProgressBar);
loading_atom!(Spinner);
