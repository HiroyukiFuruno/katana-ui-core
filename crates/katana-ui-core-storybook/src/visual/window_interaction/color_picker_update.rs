#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) struct ColorPickerUpdate {
    pub(in crate::visual) action: &'static str,
    pub(in crate::visual) event: &'static str,
    pub(in crate::visual) state: &'static str,
    pub(in crate::visual) setting: &'static str,
    pub(in crate::visual) count_action: bool,
}

impl ColorPickerUpdate {
    pub(in crate::visual) const fn counted(
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
            count_action: true,
        }
    }

    pub(in crate::visual) const fn uncounted(
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
            count_action: false,
        }
    }
}
