mod state;
#[cfg(test)]
mod tests;
mod typed;

use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiNode, UiNodeKind, UiProgressMode, UiSize, UiStateId, UiTone, UiVariant, UiVisualRole,
};
use serde::{Deserialize, Serialize};
use state::AtomState;

macro_rules! atom_model {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            state: AtomState,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: AtomState::enabled($kind),
                }
            }

            #[must_use]
            pub fn disabled(mut self, value: bool) -> Self {
                self.state.disabled = value;
                self
            }

            #[must_use]
            pub fn focusable(mut self, value: bool) -> Self {
                self.state.focusable = value;
                self
            }

            #[must_use]
            pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
                self.state.accessibility_label = value.into();
                self
            }

            #[must_use]
            pub fn selected(mut self, value: bool) -> Self {
                self.state.interaction.has_selection = value;
                self.state.interaction.selected_index = usize::from(value);
                self.state.checked = value;
                self
            }

            #[must_use]
            pub fn value(mut self, value: impl Into<String>) -> Self {
                self.state.interaction.value = value.into();
                self
            }

            #[must_use]
            pub fn font_role(mut self, value: impl Into<String>) -> Self {
                self.state.font_role = value.into();
                self
            }

            #[must_use]
            pub fn visual_role(mut self, value: UiVisualRole) -> Self {
                self.state.visual_role = value;
                self
            }

            #[must_use]
            pub fn variant(mut self, value: UiVariant) -> Self {
                self.state.variant = value;
                self.state.status.variant = value;
                self
            }

            #[must_use]
            pub fn tone(mut self, value: UiTone) -> Self {
                self.state.tone = value;
                self
            }

            #[must_use]
            pub fn size(mut self, value: UiSize) -> Self {
                self.state.size = value;
                self
            }

            #[must_use]
            pub fn loading(mut self, value: bool) -> Self {
                self.state.loading = value;
                self
            }

            #[must_use]
            pub fn readonly(mut self, value: bool) -> Self {
                self.state.readonly = value;
                self
            }

            #[must_use]
            pub fn invalid(mut self, value: bool) -> Self {
                self.state.invalid = value;
                self
            }

            #[must_use]
            pub fn placeholder(mut self, value: impl Into<String>) -> Self {
                self.state.placeholder = value.into();
                self
            }

            #[must_use]
            pub fn checked(mut self, value: bool) -> Self {
                self.state.checked = value;
                self.state.interaction.has_selection = value;
                self.state.interaction.selected_index = usize::from(value);
                self
            }

            #[must_use]
            pub fn progress(mut self, determinate: bool, percent: u8) -> Self {
                self.state.determinate = determinate;
                self.state.progress_percent = percent;
                self.state.loading_indicator.mode = if determinate {
                    UiProgressMode::Determinate
                } else {
                    UiProgressMode::Indeterminate
                };
                self
            }

            #[must_use]
            pub fn severity(mut self, value: UiTone) -> Self {
                self.state.severity = value;
                self.state.status.severity = value;
                self
            }

            #[must_use]
            pub fn state_id(&self) -> &UiStateId {
                &self.state.state_id
            }
        }

        impl crate::component::ComponentAction for $name {
            fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
                self.state.apply_action(action)
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                value.state.node($kind, value.label)
            }
        }
    };
}

atom_model!(Text, UiNodeKind::Text);
atom_model!(Icon, UiNodeKind::Icon);
atom_model!(Button, UiNodeKind::Button);
atom_model!(Input, UiNodeKind::Input);
atom_model!(Checkbox, UiNodeKind::Checkbox);
atom_model!(Radio, UiNodeKind::Radio);
atom_model!(Badge, UiNodeKind::Badge);
atom_model!(Divider, UiNodeKind::Divider);
atom_model!(Spacer, UiNodeKind::Spacer);
atom_model!(KeyCap, UiNodeKind::KeyCap);
atom_model!(LoadingDots, UiNodeKind::LoadingDots);
atom_model!(Spinner, UiNodeKind::Spinner);
atom_model!(ProgressBar, UiNodeKind::ProgressBar);
atom_model!(ColorSwatch, UiNodeKind::ColorSwatch);
atom_model!(Toggle, UiNodeKind::Toggle);
atom_model!(SlideControl, UiNodeKind::SlideControl);
atom_model!(SvgButton, UiNodeKind::SvgButton);
atom_model!(TextButton, UiNodeKind::TextButton);
atom_model!(IconTextButton, UiNodeKind::IconTextButton);
