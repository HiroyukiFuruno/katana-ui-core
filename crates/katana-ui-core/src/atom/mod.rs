mod action_policy;
pub mod chip;
mod defaults;
mod drag_handle;
mod drop_indicator;
mod image_surface;
mod input_boundary;
mod options;
pub mod shortcut_combo;
pub mod skeleton;
mod state;
mod state_actions;
#[cfg(test)]
mod tests;
pub mod text_area;
mod typed;

use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiCommonProps, UiCursor, UiDimension, UiDisplay, UiNode, UiNodeKind, UiPointerEvents,
    UiPosition, UiProgressMode, UiSize, UiStateId, UiTone, UiVariant, UiVisualRole, UiZIndex,
};
use crate::state::{UiComponentState, UiStateHandle};
pub use chip::{Chip, ChipAction, ChipEvent, ChipKeyboardInput, ChipSize, ChipTone, ChipVariant};
pub use drag_handle::DragHandle;
pub use drop_indicator::DropIndicator;
pub use image_surface::ImageSurface;
pub use input_boundary::InputValidationError;
use serde::{Deserialize, Serialize};
pub use shortcut_combo::{
    KeyCombo, KeyKind, KeyModifiers, NamedKey, RuntimePlatform, ShortcutCombo, ShortcutPlatform,
    ShortcutPlatformProvider, ShortcutSeparator,
};
pub use skeleton::{Skeleton, SkeletonAnimation, SkeletonShape, SkeletonSize};
use state::AtomState;
pub use text_area::{
    TextArea, TextAreaAction, TextAreaActionOutcome, TextAreaCaretMove, TextAreaCompositionPhase,
    TextAreaCompositionState, TextAreaEvent, TextAreaKey, TextAreaKeyChord, TextAreaNewlineKey,
    TextAreaOptions, TextAreaResizeEvent, TextAreaSelection, TextAreaState, TextAreaSubmitKey,
    TextAreaTabBehavior, TextAreaValidationError, TextAreaWrapPolicy,
};

macro_rules! atom_model {
    ($(#[$meta:meta])* $name:ident, $kind:expr) => {
        $(#[$meta])*
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
                self.state.common.disabled = value;
                self
            }

            #[must_use]
            pub fn focusable(mut self, value: bool) -> Self {
                self.state.focusable = value;
                self.state.common.focusable = value;
                self
            }

            #[must_use]
            pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
                let label = value.into();
                self.state.accessibility_label = label.clone();
                self.state.common.accessibility_label = label;
                self
            }

            #[must_use]
            pub fn common(mut self, value: UiCommonProps) -> Self {
                self.state.disabled = value.disabled;
                self.state.focusable = value.focusable;
                self.state.accessibility_label = value.accessibility_label.clone();
                self.state.common = value;
                self
            }

            #[must_use]
            pub fn visible(mut self, value: bool) -> Self {
                self.state.common.visible = value;
                self
            }

            #[must_use]
            pub fn width(mut self, value: UiDimension) -> Self {
                self.state.common.width = value;
                self
            }

            #[must_use]
            pub fn height(mut self, value: UiDimension) -> Self {
                self.state.common.height = value;
                self
            }

            #[must_use]
            pub fn display(mut self, value: UiDisplay) -> Self {
                self.state.common.display = value;
                self
            }

            #[must_use]
            pub fn position(mut self, value: UiPosition) -> Self {
                self.state.common.position = value;
                self
            }

            #[must_use]
            pub fn tab_index(mut self, value: i16) -> Self {
                self.state.common.tab_index = Some(value);
                self
            }

            #[must_use]
            pub fn z_index(mut self, value: UiZIndex) -> Self {
                self.state.common.z_index = value;
                self
            }

            #[must_use]
            pub fn cursor(mut self, value: UiCursor) -> Self {
                self.state.common.cursor = value;
                self
            }

            #[must_use]
            pub fn pointer_events(mut self, value: UiPointerEvents) -> Self {
                self.state.common.pointer_events = value;
                self
            }

            #[must_use]
            pub fn selectable(mut self, value: bool) -> Self {
                self.state.common.selectable = value;
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

            #[must_use]
            pub fn state_snapshot(&self) -> UiComponentState {
                self.state.component_state()
            }

            #[must_use]
            pub fn state_handle(&self) -> UiStateHandle<UiComponentState> {
                UiStateHandle::new(self.state_snapshot())
            }

            #[must_use]
            pub fn set_state(mut self, state: UiComponentState) -> Self {
                self.state.sync_component_state(state);
                self
            }

            #[must_use]
            pub fn update_state(mut self, update_state: impl FnOnce(&mut UiComponentState)) -> Self {
                let mut state = self.state_snapshot();
                update_state(&mut state);
                self.state.sync_component_state(state);
                self
            }

            #[must_use]
            pub fn sync_state(mut self, state_handle: &UiStateHandle<UiComponentState>) -> Self {
                self.state.sync_component_state(state_handle.get());
                self
            }
        }

        impl crate::component::ComponentAction for $name {
            fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
                self.state.apply_action_for_kind($kind, action)
            }
        }

        impl crate::component::ComponentStateBinding for $name {
            fn state_snapshot(&self) -> UiComponentState {
                self.state.component_state()
            }

            fn set_state_snapshot(&mut self, state: UiComponentState) {
                self.state.sync_component_state(state);
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
atom_model!(
    #[doc = "単一行の入力 atom。複数行入力、IME の複数行 preedit、自動行数調整は `TextArea` を使う。"]
    Input,
    UiNodeKind::Input
);
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
