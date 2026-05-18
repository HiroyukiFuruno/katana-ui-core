use super::{Badge, Icon, Input, LoadingDots, ProgressBar, Spinner, SvgButton};
use crate::render_model::{
    UiAnimationState, UiClearActionSpec, UiDismissAction, UiIconProps, UiSlotPlacement, UiSlotSpec,
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
}

impl Badge {
    #[must_use]
    pub fn dismiss_action(mut self, value: UiDismissAction) -> Self {
        self.state.status.dismiss_action = value;
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
            pub fn icon_role(mut self, value: impl Into<String>) -> Self {
                self.state.icon.role = value.into();
                self
            }

            #[must_use]
            pub fn icon_color_token(mut self, value: impl Into<String>) -> Self {
                self.state.icon.color_token = value.into();
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
        }
    };
}

loading_atom!(LoadingDots);
loading_atom!(ProgressBar);
loading_atom!(Spinner);
