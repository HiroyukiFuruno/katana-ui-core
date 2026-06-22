use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{ChoiceItem, SegmentedToggle};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct SegmentedToggleScreenState {
    pub(super) selected_index: usize,
    pub(super) focused: bool,
    pub(super) hovered: bool,
    pub(super) disabled_blocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SegmentedToggleScreenAction {
    Select,
    Focus,
    Hover,
    KeyboardSelect,
    DisabledSelect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SegmentedToggleScreenUpdate {
    pub(super) action: &'static str,
    pub(super) event: &'static str,
    pub(super) state: &'static str,
}

impl SegmentedToggleScreenState {
    pub(in crate::visual) fn apply(
        &mut self,
        action: SegmentedToggleScreenAction,
    ) -> SegmentedToggleScreenUpdate {
        match action {
            SegmentedToggleScreenAction::Select => self.select("segment_select"),
            SegmentedToggleScreenAction::Focus => self.focus(),
            SegmentedToggleScreenAction::Hover => self.hover(),
            SegmentedToggleScreenAction::KeyboardSelect => self.select("segment_keyboard_select"),
            SegmentedToggleScreenAction::DisabledSelect => self.disabled_select(),
        }
    }

    fn select(&mut self, story_action: &'static str) -> SegmentedToggleScreenUpdate {
        let mut toggle = core_toggle(false);
        let action = UiAction::segmented_toggle_selected(toggle.state_id().clone(), 1);
        let result = toggle.apply_action(&action);
        if result.handled {
            self.selected_index = result.after.selected_index;
        }
        SegmentedToggleScreenUpdate::new(story_action, action.name(), "segment=1")
    }

    fn focus(&mut self) -> SegmentedToggleScreenUpdate {
        let mut toggle = core_toggle(false);
        let action = UiAction::focus(toggle.state_id().clone());
        let result = toggle.apply_action(&action);
        self.focused = result.handled && result.after.focused;
        SegmentedToggleScreenUpdate::new("segment_focus", action.name(), "focus=true")
    }

    fn hover(&mut self) -> SegmentedToggleScreenUpdate {
        let mut toggle = core_toggle(false);
        let action = UiAction::hover(toggle.state_id().clone(), true);
        let result = toggle.apply_action(&action);
        self.hovered = result.handled && result.after.hovered;
        SegmentedToggleScreenUpdate::new("segment_hover", action.name(), "hover=true")
    }

    fn disabled_select(&mut self) -> SegmentedToggleScreenUpdate {
        let mut toggle = core_toggle(true);
        let action = UiAction::segmented_toggle_selected(toggle.state_id().clone(), 1);
        let result = toggle.apply_action(&action);
        self.disabled_blocked = !result.handled;
        SegmentedToggleScreenUpdate::new(
            "segment_disabled_select",
            "segmented_toggle_ignored",
            "disabled=true",
        )
    }
}

fn core_toggle(disabled_second: bool) -> SegmentedToggle {
    SegmentedToggle::new("Storybook segments")
        .item(ChoiceItem::new("preview", "Preview"))
        .item(ChoiceItem::new("code", "Code").disabled(disabled_second))
        .selected_index(0)
        .keyboard_navigation("left-right")
}

impl SegmentedToggleScreenUpdate {
    const fn new(action: &'static str, event: &'static str, state: &'static str) -> Self {
        Self {
            action,
            event,
            state,
        }
    }
}
