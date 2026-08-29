use super::actions::WorkspaceTabBarAction;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::WorkspaceTabId;
use super::keyboard::{WorkspaceTabKeyboardController, WorkspaceTabKeyboardInput};
pub use super::model::WorkspaceTabBar;
use super::options::{WorkspaceTab, WorkspaceTabBarOptions, WorkspaceTabGroup};
use super::ordering::ordered_visible_tabs;
use super::overflow::{
    MeasuredWorkspaceTab, WorkspaceTabOverflowConfig, WorkspaceTabOverflowPlan,
    WorkspaceTabOverflowPlanner,
};
use super::state::WorkspaceTabBarState;
use crate::render_model::UiStateId;

impl WorkspaceTabBar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let options = WorkspaceTabBarOptions::default();
        Self {
            label: label.into(),
            state: WorkspaceTabBarState::new(&options.tabs),
            options,
            event_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn tab(mut self, tab: WorkspaceTab) -> Self {
        self.options.tabs.push(tab);
        self.normalize_tabs();
        self
    }

    #[must_use]
    pub fn recently_closed_tab(mut self, tab: WorkspaceTab) -> Self {
        self.state.record_closed_tab(tab);
        self
    }

    #[must_use]
    pub fn group(mut self, group: WorkspaceTabGroup) -> Self {
        self.options.groups.push(group);
        self
    }

    #[must_use]
    pub fn active_tab_id(mut self, value: impl Into<WorkspaceTabId>) -> Self {
        self.state.active_tab_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn stable_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.state.state_id = value.into();
        self.state.sync_child_states(&self.options.tabs);
        self
    }

    #[must_use]
    pub fn overflow_trigger_width(mut self, value: u16) -> Self {
        self.options.overflow_trigger_width = value;
        self
    }

    #[must_use]
    pub fn collapsed_group_auto_expand_ms(mut self, value: u16) -> Self {
        self.options.collapsed_group_auto_expand_ms = value;
        self
    }

    #[must_use]
    pub fn options(&self) -> &WorkspaceTabBarOptions {
        &self.options
    }

    #[must_use]
    pub fn state(&self) -> &WorkspaceTabBarState {
        &self.state
    }

    #[must_use]
    pub fn event_log(&self) -> &[WorkspaceTabBarEvent] {
        &self.event_log
    }

    #[must_use]
    pub fn visual_tabs(&self) -> Vec<&WorkspaceTab> {
        ordered_visible_tabs(&self.options.tabs, &self.options.groups)
    }

    #[must_use]
    pub fn overflow_plan(
        &self,
        available_width: u16,
        measured_tabs: &[MeasuredWorkspaceTab],
    ) -> WorkspaceTabOverflowPlan {
        WorkspaceTabOverflowPlanner::compute(
            WorkspaceTabOverflowConfig::new(available_width, self.options.overflow_trigger_width),
            measured_tabs,
            self.state.active_tab_id.as_ref(),
        )
    }

    pub fn apply_keyboard_input(
        &mut self,
        input: WorkspaceTabKeyboardInput,
        visible_tab_ids: &[WorkspaceTabId],
    ) -> Vec<WorkspaceTabBarEvent> {
        if input == WorkspaceTabKeyboardInput::CancelDrag {
            return self.apply_action(WorkspaceTabBarAction::CancelDrag);
        }
        let action = WorkspaceTabKeyboardController::action_for_input(
            &input,
            self.state.active_tab_id.as_ref(),
            visible_tab_ids,
        );
        action.map_or_else(Vec::new, |it| self.apply_action(it))
    }
}
