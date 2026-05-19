use super::actions::ContextMenuAction;
use super::events::{ContextMenuCloseReason, ContextMenuEvent};
use super::item_state::{apply_checked_state, command_for_path};
use super::keyboard::{ContextMenuKeyboardInput, ContextMenuKeyboardNavigator};
use super::placement::{ContextMenuPlacementResolver, ContextMenuSize, ContextMenuViewport};
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
    pub(super) pending_submenu_path: Vec<usize>,
    pub(super) submenu_state_ids: Vec<UiStateId>,
    pub(super) callback_log: Vec<ContextMenuEvent>,
}

impl ContextMenuState {
    pub(super) fn new() -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::ContextMenu),
            open: false,
            item_count: 0,
            pending_submenu_path: Vec::new(),
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
    ) -> Vec<ContextMenuEvent> {
        match action {
            ContextMenuAction::Open { anchor } => vec![self.open(anchor.clone(), props, None)],
            ContextMenuAction::OpenWithLayout {
                anchor,
                menu_size,
                viewport,
            } => vec![self.open(anchor.clone(), props, Some((*menu_size, *viewport)))],
            ContextMenuAction::Close { reason } => vec![self.close(*reason)],
            ContextMenuAction::Highlight { path } => vec![self.highlight(path.clone(), props)],
            ContextMenuAction::Activate { path } => self.activate(path.clone(), props),
            ContextMenuAction::OpenSubmenu { path } => {
                self.pending_submenu_path = path.clone();
                props.highlighted_path = path.clone();
                vec![ContextMenuEvent::SubmenuOpened { path: path.clone() }]
            }
            ContextMenuAction::CloseSubmenu { path } => {
                self.pending_submenu_path.clear();
                vec![ContextMenuEvent::SubmenuClosed { path: path.clone() }]
            }
            ContextMenuAction::TypeAhead { prefix } => vec![self.typeahead(prefix, props)],
        }
    }

    fn open(
        &mut self,
        anchor: crate::render_model::UiContextMenuAnchor,
        props: &mut UiContextMenuProps,
        layout: Option<(ContextMenuSize, ContextMenuViewport)>,
    ) -> ContextMenuEvent {
        self.open = true;
        props.anchor = anchor.clone();
        if let Some((menu_size, viewport)) = layout {
            props.placement_used = ContextMenuPlacementResolver::resolve(
                &anchor,
                menu_size,
                viewport,
                &props.placement_priority,
            )
            .placement;
        }
        props.highlighted_path = first_enabled_path(props);
        ContextMenuEvent::Opened {
            anchor,
            placement_used: props.placement_used,
        }
    }

    fn close(&mut self, reason: ContextMenuCloseReason) -> ContextMenuEvent {
        self.open = false;
        self.pending_submenu_path.clear();
        ContextMenuEvent::Closed { reason }
    }

    fn highlight(&mut self, path: Vec<usize>, props: &mut UiContextMenuProps) -> ContextMenuEvent {
        props.highlighted_path = path.clone();
        ContextMenuEvent::ItemHighlighted { path }
    }

    fn activate(
        &mut self,
        path: Vec<usize>,
        props: &mut UiContextMenuProps,
    ) -> Vec<ContextMenuEvent> {
        self.open = false;
        props.highlighted_path = path.clone();
        self.pending_submenu_path.clear();
        apply_checked_state(props, &path);
        vec![
            ContextMenuEvent::ItemSelected {
                command: command_for_path(props, &path),
                path,
            },
            ContextMenuEvent::Closed {
                reason: ContextMenuCloseReason::Selected,
            },
        ]
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
