use super::NotificationToast;
use crate::render_model::{UiDismissAction, UiTone, UiVariant};

impl NotificationToast {
    #[must_use]
    pub fn severity(mut self, value: UiTone) -> Self {
        self.state.status.severity = value;
        self
    }

    #[must_use]
    pub fn variant(mut self, value: UiVariant) -> Self {
        self.state.status.variant = value;
        self
    }

    #[must_use]
    pub fn dismiss_action(mut self, value: UiDismissAction) -> Self {
        self.state.status.dismiss_action = value;
        self
    }
}
