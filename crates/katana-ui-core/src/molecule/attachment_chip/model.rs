use super::{
    AttachmentChipAction, AttachmentChipEvent, AttachmentKind, AttachmentMeta, AttachmentProgress,
    AttachmentStatus, AttachmentThumbnail,
};
use crate::atom::{Button, Chip, ChipAction, ChipEvent, ChipTone, ChipVariant};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachmentChip {
    chip: Chip,
    kind: AttachmentKind,
    name: String,
    meta: AttachmentMeta,
    icon: String,
    thumbnail: Option<AttachmentThumbnail>,
    progress: Option<AttachmentProgress>,
    status: AttachmentStatus,
    retry_action_label: String,
    callback_log: Vec<AttachmentChipEvent>,
}

impl AttachmentChip {
    #[must_use]
    pub fn new(kind: AttachmentKind, name: impl Into<String>) -> Self {
        let name = name.into();
        let icon = kind.default_icon().to_string();
        Self {
            chip: Chip::new(name.clone())
                .leading_icon(icon.clone())
                .interactive(true)
                .dismissible(true),
            kind,
            name,
            meta: AttachmentMeta::default(),
            icon,
            thumbnail: None,
            progress: None,
            status: AttachmentStatus::Pending,
            retry_action_label: "Retry".to_string(),
            callback_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn meta(mut self, value: AttachmentMeta) -> Self {
        self.meta = value;
        self
    }

    #[must_use]
    pub fn thumbnail(mut self, value: AttachmentThumbnail) -> Self {
        self.thumbnail = Some(value);
        self
    }

    #[must_use]
    pub fn progress(mut self, value: AttachmentProgress) -> Self {
        self.progress = Some(value);
        self
    }

    #[must_use]
    pub fn status(mut self, value: AttachmentStatus) -> Self {
        self.status = value;
        self
    }

    #[must_use]
    pub fn retry_action_label(mut self, value: impl Into<String>) -> Self {
        self.retry_action_label = value.into();
        self
    }

    #[must_use]
    pub const fn chip(&self) -> &Chip {
        &self.chip
    }

    #[must_use]
    pub const fn kind(&self) -> AttachmentKind {
        self.kind
    }

    #[must_use]
    pub const fn status_value(&self) -> AttachmentStatus {
        self.status
    }

    #[must_use]
    pub const fn progress_value(&self) -> Option<AttachmentProgress> {
        self.progress
    }

    #[must_use]
    pub fn callback_log(&self) -> &[AttachmentChipEvent] {
        &self.callback_log
    }

    #[must_use]
    pub fn retry_button(&self) -> Option<Button> {
        (self.status == AttachmentStatus::Error)
            .then(|| Button::new(self.retry_action_label.clone()))
    }

    #[must_use]
    pub fn retry_action_visible(&self) -> bool {
        self.status == AttachmentStatus::Error
    }

    #[must_use]
    pub fn effective_chip(&self) -> Chip {
        let tone = if self.status == AttachmentStatus::Error {
            ChipTone::Danger
        } else {
            ChipTone::Neutral
        };
        self.chip.clone().tone(tone).variant(ChipVariant::Soft)
    }

    #[must_use]
    pub fn apply_action(&mut self, action: AttachmentChipAction) -> Vec<AttachmentChipEvent> {
        match action {
            AttachmentChipAction::OpenPreview => self.open_preview(),
            AttachmentChipAction::Dismiss => self.dismiss(),
            AttachmentChipAction::Retry => self.retry(),
            AttachmentChipAction::TransitionStatus(status) => self.transition_status(status),
        }
    }

    fn open_preview(&mut self) -> Vec<AttachmentChipEvent> {
        let _ = self.chip.apply_action(ChipAction::Press);
        self.record_event(AttachmentChipEvent::Opened {
            id: self.chip.state_id().clone(),
        })
    }

    fn dismiss(&mut self) -> Vec<AttachmentChipEvent> {
        let chip_events = self.chip.apply_action(ChipAction::Dismiss);
        if !contains_chip_dismiss(&chip_events) {
            return Vec::new();
        }
        self.record_event(AttachmentChipEvent::Dismissed {
            id: self.chip.state_id().clone(),
        })
    }

    fn retry(&mut self) -> Vec<AttachmentChipEvent> {
        if self.status != AttachmentStatus::Error {
            return Vec::new();
        }
        let id = self.chip.state_id().clone();
        let mut events = self.record_event(AttachmentChipEvent::Retry { id: id.clone() });
        events.extend(self.transition_status_from(AttachmentStatus::Pending, id));
        events
    }

    fn transition_status(&mut self, current: AttachmentStatus) -> Vec<AttachmentChipEvent> {
        self.transition_status_from(current, self.chip.state_id().clone())
    }

    fn transition_status_from(
        &mut self,
        current: AttachmentStatus,
        id: UiStateId,
    ) -> Vec<AttachmentChipEvent> {
        if self.status == current {
            return Vec::new();
        }
        let previous = self.status;
        self.status = current;
        if current == AttachmentStatus::Pending {
            self.progress = None;
        }
        self.record_event(AttachmentChipEvent::StatusChanged {
            id,
            previous,
            current,
        })
    }

    fn record_event(&mut self, event: AttachmentChipEvent) -> Vec<AttachmentChipEvent> {
        self.callback_log.push(event.clone());
        vec![event]
    }
}

impl From<AttachmentChip> for UiNode {
    fn from(value: AttachmentChip) -> Self {
        let chip = value.effective_chip();
        let progress = value
            .progress
            .map(AttachmentProgress::percent)
            .unwrap_or_default();
        let retry_visible = value.retry_action_visible();
        let mut node = UiNode::new(UiNodeKind::AttachmentChip, value.name)
            .interaction(UiInteractionState {
                value: format!("{:?}", value.status),
                item_count: usize::from(value.progress.is_some()),
                ..UiInteractionState::default()
            })
            .progress(value.progress.is_some(), progress)
            .child(chip);
        if retry_visible {
            node = node.child(Button::new(value.retry_action_label));
        }
        node
    }
}

fn contains_chip_dismiss(events: &[ChipEvent]) -> bool {
    events
        .iter()
        .any(|event| matches!(event, ChipEvent::ChipDismissed { .. }))
}
