use super::{
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventForwardingReceipt, ROOT_EVENT_CONTEXT_MENU_INDEX,
    ROOT_EVENT_DIAGNOSTICS_INDEX, ROOT_EVENT_SEARCH_INDEX, ROOT_EVENT_STATUS_BAR_INDEX,
};

impl EguiTextCommandSurfaceRootEventDispatchReceipt {
    #[must_use]
    pub fn class_dispatches(&self) -> &[EguiTextCommandSurfaceRootEventClassDispatch] {
        &self.class_dispatches
    }

    #[must_use]
    pub const fn text_count(self) -> usize {
        self.class_dispatches[0].event_count
    }

    #[must_use]
    pub const fn toolbar_count(self) -> usize {
        self.class_dispatches[1].event_count
    }

    #[must_use]
    pub const fn floating_count(self) -> usize {
        self.class_dispatches[2].event_count
    }

    #[must_use]
    pub const fn search_count(self) -> usize {
        self.class_dispatches[ROOT_EVENT_SEARCH_INDEX].event_count
    }

    #[must_use]
    pub const fn context_menu_count(self) -> usize {
        self.class_dispatches[ROOT_EVENT_CONTEXT_MENU_INDEX].event_count
    }

    #[must_use]
    pub const fn status_bar_count(self) -> usize {
        self.class_dispatches[ROOT_EVENT_STATUS_BAR_INDEX].event_count
    }

    #[must_use]
    pub const fn diagnostics_list_count(self) -> usize {
        self.class_dispatches[ROOT_EVENT_DIAGNOSTICS_INDEX].event_count
    }
}
impl EguiTextCommandSurfaceRootEventForwardingReceipt {
    #[must_use]
    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub fn correlation_fingerprint(&self) -> &str {
        &self.correlation_fingerprint
    }

    #[must_use]
    pub fn event_batch_fingerprint(&self) -> &str {
        &self.event_batch_fingerprint
    }

    #[must_use]
    pub const fn consumed_once(&self) -> bool {
        self.consumed_once
    }

    #[must_use]
    pub const fn event_cardinality(&self) -> usize {
        self.event_cardinality
    }
}
