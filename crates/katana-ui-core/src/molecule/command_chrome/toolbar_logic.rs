use super::toolbar_mapping::{
    dropdown_key, map_toolbar_event, map_toolbar_violation, to_toolbar_action,
};
use super::{
    CommandChromeAction, CommandChromeContractViolation, CommandChromeDisplayMode,
    CommandChromeMeasuredAction, CommandChromeToolbar, CommandChromeToolbarAction,
    CommandChromeToolbarEvent,
};
use crate::molecule::toolbar::{
    MeasuredToolbarAction, ToolbarGroup, ToolbarInteractionAction, ToolbarKeyboardNavigator,
    ToolbarOptions, ToolbarOverflowInput, ToolbarOverflowPlan, ToolbarOverflowPlanner,
};

impl CommandChromeToolbar {
    #[must_use]
    pub fn toolbar_options(&self) -> ToolbarOptions {
        self.valid_actions()
            .into_iter()
            .fold(
                ToolbarOptions::new()
                    .display_mode(self.display_mode.into())
                    .density(self.density)
                    .overflow_strategy(self.overflow_strategy),
                |options, action| options.action(action.to_toolbar_action()),
            )
            .with_groups(&self.groups)
    }

    #[must_use]
    pub fn validate(&self) -> Vec<CommandChromeContractViolation> {
        let mut violations = self.icon_only_violations();
        violations.extend(self.dropdown_violations());
        violations.extend(
            self.toolbar_options()
                .validate()
                .into_iter()
                .filter_map(map_toolbar_violation),
        );
        violations
    }

    #[must_use]
    pub fn plan_overflow(
        &self,
        available_width: u32,
        overflow_trigger_width: u32,
        measured_actions: &[CommandChromeMeasuredAction],
    ) -> ToolbarOverflowPlan {
        let valid_actions = self.valid_actions();
        let measured = measured_actions
            .iter()
            .filter_map(|measured| {
                valid_actions
                    .iter()
                    .find(|action| action.id() == measured.action_id())
                    .map(|action| {
                        MeasuredToolbarAction::new(
                            action.id().clone(),
                            measured.width(),
                            action.priority_model(),
                        )
                    })
            })
            .collect();
        ToolbarOverflowPlanner::plan(&ToolbarOverflowInput::new(
            available_width,
            overflow_trigger_width,
            self.overflow_strategy,
            measured,
        ))
    }

    #[must_use]
    pub fn apply_action(
        &mut self,
        action: CommandChromeToolbarAction,
    ) -> Vec<CommandChromeToolbarEvent> {
        match &action {
            CommandChromeToolbarAction::UpdateDropdownLayout { action_id, layout } => {
                self.update_dropdown_layout(action_id.clone(), *layout);
                return Vec::new();
            }
            CommandChromeToolbarAction::DismissDropdown { reason } => {
                return self.dismiss_dropdown(*reason);
            }
            CommandChromeToolbarAction::SelectDropdownItem { action_id, item_id } => {
                return self.select_dropdown_item(action_id, item_id);
            }
            CommandChromeToolbarAction::DropdownKeyboard { input } => {
                return self.apply_dropdown_key(*input);
            }
            CommandChromeToolbarAction::Press { action_id }
            | CommandChromeToolbarAction::Activate { action_id } => {
                if let Some(events) = self.open_primary_dropdown(action_id) {
                    return events;
                }
            }
            CommandChromeToolbarAction::OpenSplitDropdown { action_id } => {
                if let Some(events) = self.open_split_secondary_dropdown(action_id) {
                    return events;
                }
            }
            CommandChromeToolbarAction::OpenOverflow
            | CommandChromeToolbarAction::ToggleGroupCollapse { .. }
            | CommandChromeToolbarAction::TriggerAccelerator { .. }
            | CommandChromeToolbarAction::Keyboard { .. } => {}
        }
        match action {
            CommandChromeToolbarAction::TriggerAccelerator { input, focus } => self
                .state
                .trigger_accelerator(&self.toolbar_actions(), &input, focus)
                .events()
                .iter()
                .filter_map(map_toolbar_event)
                .collect(),
            CommandChromeToolbarAction::Keyboard { input } => self.apply_keyboard(input),
            value => to_toolbar_action(value).map_or_else(Vec::new, |toolbar_action| {
                self.state
                    .apply_action(&toolbar_action, &self.toolbar_actions())
                    .iter()
                    .filter_map(map_toolbar_event)
                    .collect()
            }),
        }
    }

    fn apply_keyboard(
        &mut self,
        input: crate::molecule::toolbar::ToolbarKeyboardInput,
    ) -> Vec<CommandChromeToolbarEvent> {
        if self.open_dropdown_model().is_some() {
            return dropdown_key(input).map_or_else(Vec::new, |key| self.apply_dropdown_key(key));
        }
        let valid_actions = self.valid_actions();
        let toolbar_actions = valid_actions
            .iter()
            .map(|action| action.to_toolbar_action())
            .collect::<Vec<_>>();
        let result =
            ToolbarKeyboardNavigator::apply(self.focused_index, valid_actions.len(), input);
        self.focused_index = result.focused_index();
        let mut events = self
            .focused_index
            .and_then(|index| valid_actions.get(index))
            .map(|action| CommandChromeToolbarEvent::FocusChanged {
                action_id: action.id().clone(),
            })
            .into_iter()
            .collect::<Vec<_>>();
        if let Some(action_id) = result
            .activated_index()
            .and_then(|index| valid_actions.get(index))
            .map(|action| action.id().clone())
        {
            events.extend(
                self.state
                    .apply_action(
                        &ToolbarInteractionAction::activate(action_id),
                        &toolbar_actions,
                    )
                    .iter()
                    .filter_map(map_toolbar_event),
            );
        }
        events
    }

    pub(super) fn valid_actions(&self) -> Vec<CommandChromeAction> {
        let mut invalid_ids = self
            .icon_only_violations()
            .into_iter()
            .map(|violation| violation.action_id().clone())
            .collect::<Vec<_>>();
        invalid_ids.extend(
            self.dropdown_violations()
                .into_iter()
                .map(|violation| violation.action_id().clone()),
        );
        self.actions
            .iter()
            .filter(|action| !invalid_ids.iter().any(|id| id == action.id()))
            .cloned()
            .collect()
    }

    fn toolbar_actions(&self) -> Vec<crate::molecule::toolbar::ToolbarAction> {
        self.valid_actions()
            .into_iter()
            .map(|action| action.to_toolbar_action())
            .collect()
    }

    fn icon_only_violations(&self) -> Vec<CommandChromeContractViolation> {
        if self.display_mode != CommandChromeDisplayMode::IconOnly {
            return Vec::new();
        }
        self.actions
            .iter()
            .flat_map(|action| {
                let missing_icon = (!action.has_non_empty_icon()).then(|| {
                    CommandChromeContractViolation::MissingIconOnlyIcon {
                        action_id: action.id().clone(),
                    }
                });
                let missing_name = (!action.has_accessible_name()).then(|| {
                    CommandChromeContractViolation::MissingIconOnlyAccessibleName {
                        action_id: action.id().clone(),
                    }
                });
                [missing_icon, missing_name].into_iter().flatten()
            })
            .collect()
    }

    fn dropdown_violations(&self) -> Vec<CommandChromeContractViolation> {
        self.actions
            .iter()
            .filter(|action| {
                action
                    .dropdown_model()
                    .is_some_and(|dropdown| dropdown.items().is_empty())
            })
            .map(
                |action| CommandChromeContractViolation::EmptyDropdownItems {
                    action_id: action.id().clone(),
                },
            )
            .collect()
    }
}

trait ToolbarOptionsGroups {
    fn with_groups(self, groups: &[ToolbarGroup]) -> Self;
}

impl ToolbarOptionsGroups for ToolbarOptions {
    fn with_groups(self, groups: &[ToolbarGroup]) -> Self {
        groups.iter().cloned().fold(self, ToolbarOptions::group)
    }
}
