use super::model::Popover;
use super::rich_content::{PopoverArrowSpec, PopoverFocusManagement, PopoverSlots};
use crate::interaction::placement::Placement;
use crate::render_model::UiNodeId;

impl Popover {
    #[must_use]
    pub fn arrow(mut self, value: PopoverArrowSpec) -> Self {
        self.model.arrow = value;
        self
    }

    #[must_use]
    pub fn slots(mut self, value: PopoverSlots) -> Self {
        self.model.slots = value;
        self
    }

    #[must_use]
    pub fn focus_management(mut self, value: PopoverFocusManagement) -> Self {
        self.model.focus_management = value;
        self
    }

    #[must_use]
    pub fn focus_return_target(mut self, value: UiNodeId) -> Self {
        self.model.focus_return_target = Some(value);
        self
    }

    #[must_use]
    pub fn keep_open_on_inner_focus(mut self, value: bool) -> Self {
        self.model.keep_open_on_inner_focus = value;
        self
    }

    #[must_use]
    pub fn auto_flip_priority(mut self, value: impl IntoIterator<Item = Placement>) -> Self {
        self.model.auto_flip_priority = value.into_iter().collect();
        self
    }

    #[must_use]
    pub fn open_focus_target(&self) -> Option<UiNodeId> {
        match &self.model.focus_management {
            PopoverFocusManagement::None => None,
            PopoverFocusManagement::FirstInteractive => self
                .model
                .slots
                .actions
                .first()
                .map(|action| action.node_id.clone()),
            PopoverFocusManagement::NodeId(node_id) => Some(node_id.clone()),
        }
    }

    #[must_use]
    pub fn close_focus_target(&self) -> Option<UiNodeId> {
        self.model.focus_return_target.clone()
    }

    #[must_use]
    pub fn arrow_model(&self) -> &PopoverArrowSpec {
        &self.model.arrow
    }

    #[must_use]
    pub fn slots_model(&self) -> &PopoverSlots {
        &self.model.slots
    }

    #[must_use]
    pub fn focus_management_model(&self) -> &PopoverFocusManagement {
        &self.model.focus_management
    }

    #[must_use]
    pub fn keeps_open_on_inner_focus(&self) -> bool {
        self.model.keep_open_on_inner_focus
    }

    #[must_use]
    pub fn auto_flip_priority_model(&self) -> &[Placement] {
        &self.model.auto_flip_priority
    }
}
