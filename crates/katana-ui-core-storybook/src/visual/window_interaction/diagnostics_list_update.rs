#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct DiagnosticsListUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
    pub(in crate::visual) setting: &'static str,
}

impl DiagnosticsListUpdate {
    pub(in crate::visual) const fn new(
        action: &'static str,
        event: &'static str,
        state: &'static str,
        setting: &'static str,
    ) -> Self {
        Self {
            action,
            event,
            state,
            setting,
        }
    }
}
