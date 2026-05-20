use super::banner_types::{
    BannerAction, BannerCommand, BannerDensity, BannerEvent, BannerPlacementHint, BannerSeverity,
    BannerState, BannerVisualContract,
};
use crate::render_model::{UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Banner {
    pub(super) state_id: UiStateId,
    pub(super) severity: BannerSeverity,
    pub(super) title: Option<String>,
    pub(super) message: String,
    pub(super) leading_icon: Option<String>,
    pub(super) actions: Vec<BannerAction>,
    pub(super) dismissible: bool,
    pub(super) expanded_details: Option<String>,
    pub(super) density: BannerDensity,
    pub(super) placement_hint: BannerPlacementHint,
    pub(super) state: BannerState,
    callback_log: Vec<BannerEvent>,
}

impl Banner {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::Banner),
            severity: BannerSeverity::Info,
            title: None,
            message: message.into(),
            leading_icon: None,
            actions: Vec::new(),
            dismissible: false,
            expanded_details: None,
            density: BannerDensity::Default,
            placement_hint: BannerPlacementHint::Inline,
            state: BannerState::default(),
            callback_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn severity(mut self, value: BannerSeverity) -> Self {
        self.severity = value;
        self
    }

    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    #[must_use]
    pub fn leading_icon(mut self, value: impl Into<String>) -> Self {
        self.leading_icon = Some(value.into());
        self
    }

    #[must_use]
    pub fn action(mut self, value: BannerAction) -> Self {
        self.actions.push(value);
        self
    }

    #[must_use]
    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = value;
        self
    }

    #[must_use]
    pub fn expanded_details(mut self, value: impl Into<String>) -> Self {
        self.expanded_details = Some(value.into());
        self
    }

    #[must_use]
    pub fn density(mut self, value: BannerDensity) -> Self {
        self.density = value;
        self
    }

    #[must_use]
    pub fn placement_hint(mut self, value: BannerPlacementHint) -> Self {
        self.placement_hint = value;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub const fn state(&self) -> BannerState {
        self.state
    }

    #[must_use]
    pub fn callback_log(&self) -> &[BannerEvent] {
        &self.callback_log
    }

    #[must_use]
    pub fn visual_contract(&self) -> BannerVisualContract {
        BannerVisualContract {
            icon: self
                .leading_icon
                .clone()
                .or_else(|| self.severity.default_icon()),
            tone: self.severity.tone(),
            role: self.severity.role(),
            live_region: self.severity.live_region(),
            density: self.density,
            placement_hint: self.placement_hint,
            action_count: self.actions.len(),
            dismissible: self.dismissible,
            details_available: self.expanded_details.is_some(),
        }
    }

    #[must_use]
    pub fn apply_action(&mut self, action: BannerCommand) -> Vec<BannerEvent> {
        match action {
            BannerCommand::Dismiss => self.dismiss(),
            BannerCommand::ToggleDetails => self.toggle_details(),
            BannerCommand::PressAction(action_id) => self.press_action(action_id),
        }
    }

    fn dismiss(&mut self) -> Vec<BannerEvent> {
        if !self.dismissible || !self.state.visible {
            return Vec::new();
        }
        self.state.visible = false;
        self.record(BannerEvent::BannerDismissed {
            id: self.state_id.clone(),
        })
    }

    fn toggle_details(&mut self) -> Vec<BannerEvent> {
        if self.expanded_details.is_none() {
            self.state.details_open = false;
            return Vec::new();
        }
        self.state.details_open = !self.state.details_open;
        self.record(BannerEvent::BannerDetailsToggled {
            id: self.state_id.clone(),
            open: self.state.details_open,
        })
    }

    fn press_action(&mut self, action_id: String) -> Vec<BannerEvent> {
        let Some(action) = self.actions.iter().find(|it| it.id == action_id) else {
            return Vec::new();
        };
        if action.disabled {
            return Vec::new();
        }
        self.record(BannerEvent::BannerActioned {
            id: self.state_id.clone(),
            action_id,
            kind: action.kind,
        })
    }

    fn record(&mut self, event: BannerEvent) -> Vec<BannerEvent> {
        self.callback_log.push(event.clone());
        vec![event]
    }
}
