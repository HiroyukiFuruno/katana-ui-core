use super::presentation::full_path;
use super::types::ContextMenuPresentationItem;
use katana_ui_core::molecule::selection::{
    ContextMenuAction, ContextMenuCloseReason, ContextMenuEvent, ContextMenuKeyboardInput,
    ContextMenuKeyboardIntent, ContextMenuKeyboardNavigator, ContextMenuTypeAheadBuffer,
};
use katana_ui_core::render_model::UiRect;

const MILLIS_PER_SECOND: f64 = 1_000.0;

pub(super) fn keyboard_actions(
    input: &egui::InputState,
    items: &[ContextMenuPresentationItem],
    submenu_path: &mut Vec<usize>,
    highlighted_path: &[usize],
    type_ahead: &mut ContextMenuTypeAheadBuffer,
) -> Vec<ContextMenuAction> {
    let core_items = items
        .iter()
        .map(|item| {
            katana_ui_core::molecule::selection::ContextMenuItem::new(
                item.id.clone(),
                item.label.clone(),
                item.kind,
            )
            .disabled(!item.enabled)
        })
        .collect::<Vec<_>>();
    let current = highlighted_path.last().copied();
    input
        .events
        .iter()
        .cloned()
        .filter_map(|event| keyboard_input(event, type_ahead, input.time))
        .filter_map(|key| {
            if let ContextMenuKeyboardInput::TypeAhead(prefix) = key {
                return Some(ContextMenuAction::TypeAhead { prefix });
            }
            match ContextMenuKeyboardNavigator::intent(&core_items, current, &key) {
                ContextMenuKeyboardIntent::MoveTo(index) => Some(ContextMenuAction::Highlight {
                    path: full_path(submenu_path, index),
                }),
                ContextMenuKeyboardIntent::Activate if !highlighted_path.is_empty() => {
                    Some(ContextMenuAction::Activate {
                        path: highlighted_path.to_vec(),
                    })
                }
                ContextMenuKeyboardIntent::OpenSubmenu if !highlighted_path.is_empty() => {
                    Some(ContextMenuAction::OpenSubmenu {
                        path: highlighted_path.to_vec(),
                    })
                }
                ContextMenuKeyboardIntent::CloseSubmenu => {
                    submenu_path.pop();
                    None
                }
                ContextMenuKeyboardIntent::Close => Some(ContextMenuAction::Close {
                    reason: ContextMenuCloseReason::Escape,
                }),
                _ => None,
            }
        })
        .collect()
}

pub(super) fn consume_item_click(
    item: &ContextMenuPresentationItem,
    path: Vec<usize>,
) -> Option<ContextMenuAction> {
    item.enabled.then(|| {
        if item.kind == katana_ui_core::molecule::selection::ContextMenuItemKind::Submenu {
            ContextMenuAction::OpenSubmenu { path }
        } else {
            ContextMenuAction::Activate { path }
        }
    })
}

pub(super) fn is_outside_click(input: &egui::InputState, bounds: UiRect) -> bool {
    input.events.iter().any(|event| {
        let egui::Event::PointerButton {
            pos, pressed: true, ..
        } = event
        else {
            return false;
        };
        !contains(bounds, *pos)
    })
}

fn contains(bounds: UiRect, point: egui::Pos2) -> bool {
    let x = point.x.round() as i32;
    let y = point.y.round() as i32;
    x >= bounds.x
        && x < bounds.x.saturating_add_unsigned(bounds.width)
        && y >= bounds.y
        && y < bounds.y.saturating_add_unsigned(bounds.height)
}

pub(super) fn collect_events(
    menu: &mut katana_ui_core::molecule::selection::ContextMenu,
    actions: impl IntoIterator<Item = ContextMenuAction>,
) -> Vec<ContextMenuEvent> {
    actions
        .into_iter()
        .flat_map(|action| menu.apply_context_action_events(&action))
        .collect()
}

fn keyboard_input(
    event: egui::Event,
    type_ahead: &mut ContextMenuTypeAheadBuffer,
    time_seconds: f64,
) -> Option<ContextMenuKeyboardInput> {
    match event {
        egui::Event::Key {
            key, pressed: true, ..
        } => match key {
            egui::Key::ArrowDown => Some(ContextMenuKeyboardInput::ArrowDown),
            egui::Key::ArrowUp => Some(ContextMenuKeyboardInput::ArrowUp),
            egui::Key::Home => Some(ContextMenuKeyboardInput::Home),
            egui::Key::End => Some(ContextMenuKeyboardInput::End),
            egui::Key::Enter => Some(ContextMenuKeyboardInput::Enter),
            egui::Key::Space => Some(ContextMenuKeyboardInput::Space),
            egui::Key::ArrowRight => Some(ContextMenuKeyboardInput::ArrowRight),
            egui::Key::ArrowLeft => Some(ContextMenuKeyboardInput::ArrowLeft),
            egui::Key::Escape => Some(ContextMenuKeyboardInput::Escape),
            _ => None,
        },
        egui::Event::Text(value) if !value.is_empty() => Some(ContextMenuKeyboardInput::TypeAhead(
            type_ahead.push(&value, (time_seconds * MILLIS_PER_SECOND).max(0.0) as u64),
        )),
        _ => None,
    }
}
