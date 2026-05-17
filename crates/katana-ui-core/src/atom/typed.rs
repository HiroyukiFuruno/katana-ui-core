use super::{Badge, Input, LoadingDots, ProgressBar, Spinner};
use crate::render_model::{
    UiAnimationState, UiClearActionSpec, UiDismissAction, UiSlotPlacement, UiSlotSpec,
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
