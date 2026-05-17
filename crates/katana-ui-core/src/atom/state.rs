use crate::facade::DEFAULT_FONT_ROLE;
use crate::interaction::{ProgressAction, UiAction, UiActionResult, UiActionSource};
use crate::render_model::{
    UiAnimationState, UiInteractionState, UiLoadingProps, UiNode, UiNodeKind, UiProgressMode,
    UiSize, UiStateId, UiStatusProps, UiTextEntryProps, UiTone, UiVariant, UiVisualRole,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct AtomState {
    pub state_id: UiStateId,
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
    pub text_entry: UiTextEntryProps,
    pub status: UiStatusProps,
    pub loading_indicator: UiLoadingProps,
}

impl AtomState {
    pub(super) fn enabled(kind: UiNodeKind) -> Self {
        Self {
            state_id: UiStateId::next_for(kind),
            disabled: false,
            focusable: false,
            accessibility_label: String::new(),
            interaction: UiInteractionState::default(),
            font_role: DEFAULT_FONT_ROLE.to_string(),
            visual_role: default_visual_role(kind),
            variant: default_variant(kind),
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
            text_entry: UiTextEntryProps::default(),
            status: UiStatusProps::default(),
            loading_indicator: default_loading_props(kind),
        }
    }

    pub(super) fn node(self, kind: UiNodeKind, label: impl Into<String>) -> UiNode {
        UiNode::from_state(kind, label, self.state_id)
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
            .text_entry(self.text_entry)
            .status(self.status)
            .loading_indicator(self.loading_indicator)
    }

    pub(super) fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.interaction.clone();
        let blocks_value = matches!(
            action,
            UiAction::SetValue { .. } | UiAction::ClearValue { .. }
        );
        if action.target() != &self.state_id || self.disabled || (self.readonly && blocks_value) {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        self.apply_interaction_action(action);
        UiActionResult::handled(
            self.state_id.clone(),
            action,
            before,
            self.interaction.clone(),
        )
    }

    fn apply_interaction_action(&mut self, action: &UiAction) {
        match action {
            UiAction::Press { .. } => {}
            UiAction::SetOpen { open, .. } => {
                self.interaction.open = *open;
            }
            UiAction::SetSelectedIndex {
                selected_index,
                selected,
                source,
                ..
            } => {
                self.interaction.has_selection = *selected;
                self.interaction.selected_index = *selected_index;
                self.apply_checked_selection(*source, *selected);
            }
            UiAction::SetValue {
                value, progress, ..
            } => {
                self.apply_value_action(value, progress.as_ref());
            }
            UiAction::ClearValue { .. } => {
                self.interaction.value.clear();
            }
            UiAction::Dismiss { .. } => {
                self.interaction.open = false;
            }
        }
    }

    fn apply_checked_selection(&mut self, source: UiActionSource, selected: bool) {
        match source {
            UiActionSource::Checkbox | UiActionSource::Radio | UiActionSource::Toggle => {
                self.checked = selected;
            }
            _ => {}
        }
    }

    fn apply_value_action(&mut self, value: &str, progress: Option<&ProgressAction>) {
        if let Some(progress) = progress {
            self.determinate = progress.determinate;
            self.progress_percent = progress.percent;
            self.interaction.value = progress.percent.to_string();
            self.loading_indicator.mode = if progress.determinate {
                UiProgressMode::Determinate
            } else {
                UiProgressMode::Indeterminate
            };
            return;
        }
        self.interaction.value = value.to_string();
    }
}

fn default_loading_props(kind: UiNodeKind) -> UiLoadingProps {
    let animation_state = match kind {
        UiNodeKind::LoadingDots | UiNodeKind::Spinner => UiAnimationState::Running,
        _ => UiAnimationState::Idle,
    };
    UiLoadingProps {
        mode: UiProgressMode::Indeterminate,
        label: String::new(),
        animation_state,
    }
}

fn default_visual_role(kind: UiNodeKind) -> UiVisualRole {
    match kind {
        UiNodeKind::Icon => UiVisualRole::Icon,
        UiNodeKind::Input => UiVisualRole::Input,
        UiNodeKind::Badge => UiVisualRole::Status,
        UiNodeKind::Divider => UiVisualRole::Separator,
        UiNodeKind::KeyCap => UiVisualRole::Shortcut,
        UiNodeKind::LoadingDots | UiNodeKind::Spinner => UiVisualRole::Loading,
        UiNodeKind::ProgressBar => UiVisualRole::Progress,
        UiNodeKind::Button
        | UiNodeKind::Checkbox
        | UiNodeKind::Radio
        | UiNodeKind::ColorSwatch
        | UiNodeKind::Toggle
        | UiNodeKind::SlideControl => UiVisualRole::Control,
        _ => UiVisualRole::Content,
    }
}

fn default_variant(kind: UiNodeKind) -> UiVariant {
    match kind {
        UiNodeKind::Button => UiVariant::Filled,
        UiNodeKind::Icon => UiVariant::Icon,
        _ => UiVariant::Plain,
    }
}
