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
        let next =
            next_focus_index(&enabled, focused, key).unwrap_or(focused.unwrap_or(enabled[0]));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::placement::{Rect, Size};
    use crate::molecule::command_chrome::{
        CommandChromeAction, CommandChromeDropdownItem, CommandChromeDropdownTrigger,
    };

    fn layout(x: i32) -> CommandChromeDropdownLayout {
        CommandChromeDropdownLayout::new(
            Rect::new(x, 0, 20, 20),
            Rect::new(0, 0, 200, 100),
            Size::new(80, 60),
        )
    }

    fn dropdown(trigger: CommandChromeDropdownTrigger, disabled: bool) -> CommandChromeDropdown {
        CommandChromeDropdown::new(trigger)
            .item(CommandChromeDropdownItem::new("disabled", "Disabled").disabled(true))
            .item(CommandChromeDropdownItem::new("enabled", "Enabled").disabled(disabled))
            .item(CommandChromeDropdownItem::new("enabled-two", "Enabled two").disabled(disabled))
    }

    #[test]
    fn dropdown_fail_closed_paths_and_keyboard_boundaries_are_total() {
        let mut toolbar = CommandChromeToolbar::new()
            .action(CommandChromeAction::new("plain", "Plain"))
            .action(
                CommandChromeAction::new("menu", "Menu")
                    .dropdown(dropdown(CommandChromeDropdownTrigger::Primary, false)),
            );
        toolbar.update_dropdown_layout("missing".into(), layout(0));
        toolbar.update_dropdown_layout("plain".into(), layout(0));
        assert!(
            toolbar
                .select_dropdown_item(&"menu".into(), &"enabled".into())
                .is_empty()
        );
        assert!(
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::ArrowDown)
                .is_empty()
        );

        toolbar.update_dropdown_layout("menu".into(), layout(0));
        assert!(toolbar.open_primary_dropdown(&"menu".into()).is_some());
        assert!(
            toolbar
                .select_dropdown_item(&"other".into(), &"enabled".into())
                .is_empty()
        );
        assert!(
            toolbar
                .select_dropdown_item(&"menu".into(), &"disabled".into())
                .is_empty()
        );
        assert!(
            !toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::End)
                .is_empty()
        );
        assert!(
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::End)
                .is_empty()
        );
        assert!(
            !toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::Enter)
                .is_empty()
        );
        assert!(
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::Escape)
                .is_empty()
        );

        assert_eq!(
            next_focus_index(&[1], Some(1), CommandChromeDropdownKey::Enter),
            None
        );
        assert_eq!(
            next_focus_index(&[1, 3], Some(1), CommandChromeDropdownKey::ArrowUp),
            Some(3)
        );
        assert_eq!(
            next_focus_index(&[1, 3], Some(1), CommandChromeDropdownKey::ArrowDown),
            Some(3)
        );
        assert_eq!(
            next_focus_index(&[1, 3], Some(3), CommandChromeDropdownKey::Home),
            Some(1)
        );
    }

    #[test]
    fn dropdown_switching_missing_models_and_disabled_items_fail_closed() {
        let mut toolbar = CommandChromeToolbar::new()
            .action(
                CommandChromeAction::new("first", "First")
                    .dropdown(dropdown(CommandChromeDropdownTrigger::Primary, false)),
            )
            .action(
                CommandChromeAction::new("second", "Second")
                    .dropdown(dropdown(CommandChromeDropdownTrigger::Primary, false)),
            )
            .action(
                CommandChromeAction::new("all-disabled", "All disabled")
                    .dropdown(dropdown(CommandChromeDropdownTrigger::Primary, true)),
            );
        for (index, id) in ["first", "second", "all-disabled"].into_iter().enumerate() {
            toolbar.update_dropdown_layout(id.into(), layout(index as i32 * 20));
        }
        assert!(toolbar.open_primary_dropdown(&"first".into()).is_some());
        let switched = toolbar.open_primary_dropdown(&"second".into());
        assert!(matches!(
            switched.as_ref().and_then(|events| events.first()),
            Some(CommandChromeToolbarEvent::DropdownClosed { .. })
        ));

        toolbar.open_dropdown = Some(CommandChromeOpenDropdown::new(
            "missing".into(),
            layout(0),
            Some(0),
        ));
        assert!(
            toolbar
                .select_dropdown_item(&"missing".into(), &"enabled".into())
                .is_empty()
        );
        assert!(
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::ArrowDown)
                .is_empty()
        );

        toolbar.open_dropdown = Some(CommandChromeOpenDropdown::new(
            "all-disabled".into(),
            layout(0),
            None,
        ));
        assert!(
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::ArrowDown)
                .is_empty()
        );
    }

    #[test]
    fn trigger_and_layout_paths_cover_split_dropdown_and_layout_updates() {
        let mut toolbar = CommandChromeToolbar::new()
            .action(
                CommandChromeAction::new("primary", "Primary")
                    .dropdown(dropdown(CommandChromeDropdownTrigger::Primary, false)),
            )
            .action(
                CommandChromeAction::new("split", "Split").dropdown(dropdown(
                    CommandChromeDropdownTrigger::SplitSecondary,
                    false,
                )),
            );

        assert!(toolbar.open_primary_dropdown(&"split".into()).is_none());
        assert!(
            toolbar
                .open_split_secondary_dropdown(&"primary".into())
                .is_none()
        );

        assert!(toolbar.open_primary_dropdown(&"primary".into()).is_none());
        toolbar.update_dropdown_layout("primary".into(), layout(11));
        toolbar.update_dropdown_layout("split".into(), layout(21));

        let split_events = toolbar.open_split_secondary_dropdown(&"split".into());
        assert!(split_events.is_some());
        let before = toolbar.open_dropdown.as_ref().map(|it| it.placement());
        toolbar.update_dropdown_layout("split".into(), layout(31));
        let after = toolbar.open_dropdown.as_ref().map(|it| it.placement());
        assert!(before.is_some());
        assert!(after.is_some());
        assert_ne!(before, after);

        assert_eq!(
            1,
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::ArrowUp)
                .len()
        );
    }

    #[test]
    fn dropdown_open_fail_closed_when_layout_or_trigger_mismatch() {
        let mut toolbar = CommandChromeToolbar::new().action(
            CommandChromeAction::new("action", "Action").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::SplitSecondary)
                    .item(CommandChromeDropdownItem::new("item", "Item")),
            ),
        );

        assert!(toolbar.open_primary_dropdown(&"action".into()).is_none());

        toolbar.update_dropdown_layout("action".into(), layout(9));
        assert!(toolbar.open_primary_dropdown(&"action".into()).is_none());
        assert!(
            toolbar
                .open_split_secondary_dropdown(&"action".into())
                .is_some()
        );
    }

    #[test]
    fn dropdown_navigation_with_single_enabled_item_covers_noop_focus_move_path() {
        let mut toolbar = CommandChromeToolbar::new().action(
            CommandChromeAction::new("single", "Single").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                    .item(CommandChromeDropdownItem::new("only", "Only")),
            ),
        );
        toolbar.update_dropdown_layout("single".into(), layout(1));

        assert!(toolbar.open_primary_dropdown(&"single".into()).is_some());
        assert_eq!(
            0,
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::ArrowDown)
                .len()
        );
        assert_eq!(
            0,
            toolbar
                .apply_dropdown_key(CommandChromeDropdownKey::Home)
                .len()
        );
    }

    #[test]
    fn dropdown_open_reopen_reuses_existing_entry_with_no_duplicate_focus_change() {
        let mut toolbar = CommandChromeToolbar::new().action(
            CommandChromeAction::new("menu", "Menu").dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                    .item(CommandChromeDropdownItem::new("item", "Item")),
            ),
        );
        toolbar.update_dropdown_layout("menu".into(), layout(0));

        let first_open = toolbar.open_primary_dropdown(&"menu".into());
        assert_eq!(Some(2), first_open.as_ref().map(Vec::len));

        let second_open = toolbar.open_primary_dropdown(&"menu".into());
        assert!(
            second_open
                .as_ref()
                .is_some_and(|events| !events.is_empty())
        );
        assert_eq!(
            first_open.as_ref().and_then(|events| events.last()),
            second_open.as_ref().and_then(|events| events.last()),
            "reopening same action should avoid explicit dropdown-close event"
        );
    }
}
