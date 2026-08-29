use super::{
    CommandChromeDropdown, CommandChromeDropdownCloseReason, CommandChromeDropdownItemId,
    CommandChromeDropdownKey, CommandChromeDropdownLayout, CommandChromeDropdownTrigger,
    CommandChromeOpenDropdown, CommandChromeToolbar, CommandChromeToolbarEvent,
};
use crate::molecule::toolbar::ToolbarActionId;

impl CommandChromeToolbar {
    pub(super) fn update_dropdown_layout(
        &mut self,
        action_id: ToolbarActionId,
        layout: CommandChromeDropdownLayout,
    ) {
        if self
            .actions
            .iter()
            .all(|action| action.id() != &action_id || action.dropdown_model().is_none())
        {
            return;
        }
        if let Some((_, stored)) = self
            .dropdown_layouts
            .iter_mut()
            .find(|(stored_id, _)| stored_id == &action_id)
        {
            *stored = layout;
        } else {
            self.dropdown_layouts.push((action_id.clone(), layout));
        }
        if let Some(open) = self
            .open_dropdown
            .as_mut()
            .filter(|open| open.action_id() == &action_id)
        {
            open.update_layout(layout);
        }
    }

    pub(super) fn open_primary_dropdown(
        &mut self,
        action_id: &ToolbarActionId,
    ) -> Option<Vec<CommandChromeToolbarEvent>> {
        self.open_dropdown(action_id, CommandChromeDropdownTrigger::Primary)
    }

    pub(super) fn open_split_secondary_dropdown(
        &mut self,
        action_id: &ToolbarActionId,
    ) -> Option<Vec<CommandChromeToolbarEvent>> {
        self.open_dropdown(action_id, CommandChromeDropdownTrigger::SplitSecondary)
    }

    pub(super) fn dismiss_dropdown(
        &mut self,
        reason: CommandChromeDropdownCloseReason,
    ) -> Vec<CommandChromeToolbarEvent> {
        self.open_dropdown.take().map_or_else(Vec::new, |open| {
            vec![CommandChromeToolbarEvent::DropdownClosed {
                action_id: open.action_id().clone(),
                reason,
            }]
        })
    }

    pub(super) fn select_dropdown_item(
        &mut self,
        action_id: &ToolbarActionId,
        item_id: &CommandChromeDropdownItemId,
    ) -> Vec<CommandChromeToolbarEvent> {
        let Some(open) = self.open_dropdown.as_ref() else {
            return Vec::new();
        };
        if open.action_id() != action_id {
            return Vec::new();
        }
        let Some(dropdown) = self.dropdown_for(action_id) else {
            return Vec::new();
        };
        if dropdown
            .items()
            .iter()
            .all(|item| item.id() != item_id || item.disabled_model())
        {
            return Vec::new();
        }
        self.open_dropdown = None;
        vec![
            CommandChromeToolbarEvent::DropdownItemActivated {
                action_id: action_id.clone(),
                item_id: item_id.clone(),
            },
            CommandChromeToolbarEvent::DropdownClosed {
                action_id: action_id.clone(),
                reason: CommandChromeDropdownCloseReason::ItemActivated,
            },
        ]
    }

    pub(super) fn apply_dropdown_key(
        &mut self,
        key: CommandChromeDropdownKey,
    ) -> Vec<CommandChromeToolbarEvent> {
        if key == CommandChromeDropdownKey::Escape {
            return self.dismiss_dropdown(CommandChromeDropdownCloseReason::Escape);
        }
        let Some(open) = self.open_dropdown.as_ref() else {
            return Vec::new();
        };
        let action_id = open.action_id().clone();
        let focused = open.focused_item_index();
        let Some(dropdown) = self.dropdown_for(&action_id) else {
            return Vec::new();
        };
        let enabled = dropdown
            .items()
            .iter()
            .enumerate()
            .filter_map(|(index, item)| (!item.disabled_model()).then_some(index))
            .collect::<Vec<_>>();
        if enabled.is_empty() {
            return Vec::new();
        }
        if matches!(
            key,
            CommandChromeDropdownKey::Enter | CommandChromeDropdownKey::Space
        ) {
            return focused
                .and_then(|index| dropdown.items().get(index))
                .map(|item| self.select_dropdown_item(&action_id, item.id()))
                .unwrap_or_default();
        }
        let next = next_focus_index(&enabled, focused, key);
        let Some(next) = next else {
            return Vec::new();
        };
        if focused == Some(next) {
            return Vec::new();
        }
        let item_id = dropdown.items()[next].id().clone();
        if let Some(open) = self.open_dropdown.as_mut() {
            open.set_focused_item_index(Some(next));
        }
        vec![CommandChromeToolbarEvent::DropdownFocusChanged { action_id, item_id }]
    }

    fn open_dropdown(
        &mut self,
        action_id: &ToolbarActionId,
        trigger: CommandChromeDropdownTrigger,
    ) -> Option<Vec<CommandChromeToolbarEvent>> {
        let dropdown = self.dropdown_for(action_id)?;
        if dropdown.trigger_model() != trigger {
            return None;
        }
        let layout = self
            .dropdown_layouts
            .iter()
            .find(|(stored_id, _)| stored_id == action_id)
            .map(|(_, layout)| *layout)?;
        let focused = dropdown
            .items()
            .iter()
            .position(|item| !item.disabled_model());
        let open = CommandChromeOpenDropdown::new(action_id.clone(), layout, focused);
        let mut events = self
            .open_dropdown
            .take()
            .filter(|current| current.action_id() != action_id)
            .map(|current| CommandChromeToolbarEvent::DropdownClosed {
                action_id: current.action_id().clone(),
                reason: CommandChromeDropdownCloseReason::Explicit,
            })
            .into_iter()
            .collect::<Vec<_>>();
        events.push(CommandChromeToolbarEvent::DropdownOpened {
            action_id: action_id.clone(),
            placement: open.placement(),
        });
        if let Some(index) = focused {
            events.push(CommandChromeToolbarEvent::DropdownFocusChanged {
                action_id: action_id.clone(),
                item_id: dropdown.items()[index].id().clone(),
            });
        }
        self.open_dropdown = Some(open);
        Some(events)
    }

    fn dropdown_for(&self, action_id: &ToolbarActionId) -> Option<CommandChromeDropdown> {
        self.actions
            .iter()
            .find(|action| action.id() == action_id && !action.disabled_model())
            .and_then(|action| action.dropdown_model())
            .filter(|dropdown| !dropdown.items().is_empty())
            .cloned()
    }
}

fn next_focus_index(
    enabled: &[usize],
    focused: Option<usize>,
    key: CommandChromeDropdownKey,
) -> Option<usize> {
    let current = focused
        .and_then(|index| enabled.iter().position(|candidate| *candidate == index))
        .unwrap_or(0);
    match key {
        CommandChromeDropdownKey::ArrowUp => enabled
            .get(current.checked_sub(1).unwrap_or(enabled.len() - 1))
            .copied(),
        CommandChromeDropdownKey::ArrowDown => enabled.get((current + 1) % enabled.len()).copied(),
        CommandChromeDropdownKey::Home => enabled.first().copied(),
        CommandChromeDropdownKey::End => enabled.last().copied(),
        CommandChromeDropdownKey::Enter
        | CommandChromeDropdownKey::Space
        | CommandChromeDropdownKey::Escape => None,
    }
}
