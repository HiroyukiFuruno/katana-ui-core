use super::actions::ContextMenuAction;
use super::events::{ContextMenuCloseReason, ContextMenuEvent};
use super::keyboard::{ContextMenuKeyboardInput, ContextMenuKeyboardNavigator};
use crate::render_model::{
    UiContextMenuItem, UiContextMenuItemKind, UiContextMenuProps, UiInteractionState, UiNodeKind,
    UiStateId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ContextMenuState {
    pub(super) state_id: UiStateId,
    pub(super) open: bool,
    pub(super) item_count: usize,
    pub(super) submenu_state_ids: Vec<UiStateId>,
    pub(super) callback_log: Vec<ContextMenuEvent>,
}

impl ContextMenuState {
    pub(super) fn new() -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::ContextMenu),
            open: false,
            item_count: 0,
            submenu_state_ids: Vec::new(),
            callback_log: Vec::new(),
        }
    }

    pub(super) fn sync_submenu_state_ids(&mut self, items: &[UiContextMenuItem]) {
        let submenu_count = count_submenus(items);
        while self.submenu_state_ids.len() < submenu_count {
            self.submenu_state_ids
                .push(UiStateId::next_for(UiNodeKind::ContextMenu));
        }
        self.submenu_state_ids.truncate(submenu_count);
    }

    pub(super) fn interaction(&self) -> UiInteractionState {
        UiInteractionState {
            open: self.open,
            item_count: self.item_count,
            ..UiInteractionState::default()
        }
    }

    pub(super) fn apply(
        &mut self,
        action: &ContextMenuAction,
        props: &mut UiContextMenuProps,
    ) -> ContextMenuEvent {
        match action {
            ContextMenuAction::Open { anchor } => self.open(anchor.clone(), props),
            ContextMenuAction::Close { reason } => self.close(*reason),
            ContextMenuAction::Highlight { path } => self.highlight(path.clone(), props),
            ContextMenuAction::Activate { path } => self.activate(path.clone(), props),
            ContextMenuAction::OpenSubmenu { path } => {
                props.highlighted_path = path.clone();
                ContextMenuEvent::SubmenuOpened { path: path.clone() }
            }
            ContextMenuAction::CloseSubmenu { path } => {
                ContextMenuEvent::SubmenuClosed { path: path.clone() }
            }
            ContextMenuAction::TypeAhead { prefix } => self.typeahead(prefix, props),
        }
    }

    fn open(
        &mut self,
        anchor: crate::render_model::UiContextMenuAnchor,
        props: &mut UiContextMenuProps,
    ) -> ContextMenuEvent {
        self.open = true;
        props.anchor = anchor.clone();
        props.highlighted_path = first_enabled_path(props);
        ContextMenuEvent::Opened {
            anchor,
            placement_used: props.placement_used,
        }
    }

    fn close(&mut self, reason: ContextMenuCloseReason) -> ContextMenuEvent {
        self.open = false;
        ContextMenuEvent::Closed { reason }
    }

    fn highlight(&mut self, path: Vec<usize>, props: &mut UiContextMenuProps) -> ContextMenuEvent {
        props.highlighted_path = path.clone();
        ContextMenuEvent::ItemHighlighted { path }
    }

    fn activate(&mut self, path: Vec<usize>, props: &mut UiContextMenuProps) -> ContextMenuEvent {
        self.open = false;
        props.highlighted_path = path.clone();
        ContextMenuEvent::ItemSelected {
            command: command_for_path(props, &path),
            path,
        }
    }

    fn typeahead(&mut self, prefix: &str, props: &mut UiContextMenuProps) -> ContextMenuEvent {
        let current = props.highlighted_path.first().copied();
        let input = ContextMenuKeyboardInput::TypeAhead(prefix.to_string());
        let path = ContextMenuKeyboardNavigator::move_highlight(&props.items, current, &input)
            .map_or(Vec::new(), |it| vec![it]);
        props.highlighted_path = path.clone();
        ContextMenuEvent::TypeAheadMatched {
            prefix: prefix.to_string(),
            path,
        }
    }
}

fn count_submenus(items: &[UiContextMenuItem]) -> usize {
    items
        .iter()
        .map(|item| {
            let current = usize::from(item.kind == UiContextMenuItemKind::Submenu);
            current + count_submenus(&item.children)
        })
        .sum()
}

fn first_enabled_path(props: &UiContextMenuProps) -> Vec<usize> {
    ContextMenuKeyboardNavigator::move_highlight(
        &props.items,
        None,
        &ContextMenuKeyboardInput::Home,
    )
    .map_or(Vec::new(), |it| vec![it])
}

fn command_for_path(props: &UiContextMenuProps, path: &[usize]) -> String {
    let Some(index) = path.first() else {
        return String::new();
    };
    props.items.get(*index).map_or_else(String::new, |item| {
        if matches!(
            item.kind,
            UiContextMenuItemKind::Divider | UiContextMenuItemKind::Section
        ) {
            return String::new();
        }
        item.id.clone()
    })
}
