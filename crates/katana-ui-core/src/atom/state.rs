use super::defaults;
use crate::facade::DEFAULT_FONT_ROLE;
use crate::render_model::{
    UiButtonProps, UiColorSwatchProps, UiCommonProps, UiIconProps, UiInteractionState,
    UiInteractivePreset, UiLoadingProps, UiNode, UiNodeKind, UiShortcutProps, UiSize, UiStateId,
    UiStatusProps, UiTextEntryProps, UiTextProps, UiTone, UiVariant, UiVisualRole,
};
use crate::state::UiComponentState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct AtomState {
    pub state_id: UiStateId,
    pub common: UiCommonProps,
    pub disabled: bool,
    pub focusable: bool,
    pub accessibility_label: String,
    pub interaction: UiInteractionState,
    pub font_role: String,
    pub visual_role: UiVisualRole,
    pub variant: UiVariant,
    pub tone: UiTone,
    pub size: UiSize,
    pub loading: bool,
    pub readonly: bool,
    pub invalid: bool,
    pub placeholder: String,
    pub checked: bool,
    pub determinate: bool,
    pub progress_percent: u8,
    pub severity: UiTone,
    pub text: UiTextProps,
    pub button: UiButtonProps,
    pub color_swatch: UiColorSwatchProps,
    pub shortcut: UiShortcutProps,
    pub text_entry: UiTextEntryProps,
    pub status: UiStatusProps,
    pub loading_indicator: UiLoadingProps,
    pub icon: UiIconProps,
}

impl AtomState {
    pub(super) fn enabled(kind: UiNodeKind) -> Self {
        Self {
            state_id: UiStateId::next_for(kind),
            common: default_common_props(kind),
            disabled: false,
            focusable: false,
            accessibility_label: String::new(),
            interaction: UiInteractionState::default(),
            font_role: DEFAULT_FONT_ROLE.to_string(),
            visual_role: defaults::visual_role(kind),
            variant: defaults::variant(kind),
            tone: UiTone::Neutral,
            size: UiSize::Medium,
            loading: false,
            readonly: false,
            invalid: false,
            placeholder: String::new(),
            checked: false,
            determinate: false,
            progress_percent: 0,
            severity: UiTone::Neutral,
            text: UiTextProps::default(),
            button: UiButtonProps::default(),
            color_swatch: UiColorSwatchProps::default(),
            shortcut: UiShortcutProps::default(),
            text_entry: UiTextEntryProps::default(),
            status: UiStatusProps::default(),
            loading_indicator: defaults::loading_props(kind),
            icon: UiIconProps::default(),
        }
    }

    pub(super) fn node(self, kind: UiNodeKind, label: impl Into<String>) -> UiNode {
        UiNode::from_state(kind, label, self.state_id)
            .common(self.common)
            .disabled(self.disabled)
            .focusable(self.focusable)
            .accessibility_label(self.accessibility_label)
            .interaction(self.interaction)
            .font_role(self.font_role)
            .visual_role(self.visual_role)
            .variant(self.variant)
            .tone(self.tone)
            .size(self.size)
            .loading(self.loading)
            .readonly(self.readonly)
            .invalid(self.invalid)
            .placeholder(self.placeholder)
            .checked(self.checked)
            .progress(self.determinate, self.progress_percent)
            .severity(self.severity)
            .text(self.text)
            .button(self.button)
            .color_swatch(self.color_swatch)
            .shortcut(self.shortcut)
            .text_entry(self.text_entry)
            .status(self.status)
            .loading_indicator(self.loading_indicator)
            .icon(self.icon)
    }

    pub(super) fn component_state(&self) -> UiComponentState {
        UiComponentState {
            state_id: self.state_id.clone(),
            common: self.common.clone(),
            disabled: self.disabled,
            focusable: self.focusable,
            loading: self.loading,
            readonly: self.readonly,
            invalid: self.invalid,
            checked: self.checked,
            interaction: self.interaction.clone(),
        }
    }

    pub(super) fn sync_component_state(&mut self, state: UiComponentState) {
        self.state_id = state.state_id;
        self.common = state.common;
        self.disabled = state.disabled;
        self.focusable = state.focusable;
        self.loading = state.loading;
        self.readonly = state.readonly;
        self.invalid = state.invalid;
        self.checked = state.checked;
        self.interaction = state.interaction;
        self.common.disabled = self.disabled;
        self.common.focusable = self.focusable;
    }
}

fn default_common_props(kind: UiNodeKind) -> UiCommonProps {
    if is_pointer_control_kind(kind) {
        return UiInteractivePreset::control().apply_to_common(UiCommonProps::default());
    }
    UiCommonProps::default()
}

const fn is_pointer_control_kind(kind: UiNodeKind) -> bool {
    matches!(
        kind,
        UiNodeKind::Button
            | UiNodeKind::Checkbox
            | UiNodeKind::Radio
            | UiNodeKind::Toggle
            | UiNodeKind::ColorSwatch
            | UiNodeKind::SlideControl
            | UiNodeKind::SvgButton
            | UiNodeKind::TextButton
            | UiNodeKind::IconTextButton
    )
}
