use super::presentation::full_path;
use super::types::ContextMenuPresentationItem;
use crate::molecule::selection::{
    ContextMenuAction, ContextMenuCloseReason, ContextMenuEvent, ContextMenuKeyboardInput,
    ContextMenuKeyboardIntent, ContextMenuKeyboardNavigator, ContextMenuTypeAheadBuffer,
};
use crate::render_model::UiRect;

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
            crate::molecule::selection::ContextMenuItem::new(
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
        if item.kind == crate::molecule::selection::ContextMenuItemKind::Submenu {
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
    menu: &mut crate::molecule::selection::ContextMenu,
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

#[cfg(test)]
mod tests {
    use super::{
        ContextMenuTypeAheadBuffer, UiRect, consume_item_click, is_outside_click, keyboard_input,
    };
    use crate::egui::context_menu::ContextMenuPresentationItem;
    use crate::molecule::selection::{
        ContextMenuAction, ContextMenuItemKind, ContextMenuKeyboardInput,
    };

    #[test]
    fn outside_click_distinguishes_inside_and_outside_boundaries() {
        let context = egui::Context::default();
        let bounds = UiRect::new(10, 20, 40, 30);
        let mut inside = false;
        let mut outside = false;

        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(100.0, 100.0),
                )),
                events: vec![egui::Event::PointerButton {
                    pos: egui::pos2(20.0, 30.0),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| {
                ui.input(|input| {
                    inside = is_outside_click(input, bounds);
                });
            },
        );
        output.textures_delta.clear();

        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(100.0, 100.0),
                )),
                events: vec![egui::Event::PointerButton {
                    pos: egui::pos2(9.0, 19.0),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| {
                ui.input(|input| {
                    outside = is_outside_click(input, bounds);
                });
            },
        );
        output.textures_delta.clear();

        assert!(!inside);
        assert!(outside);
    }

    #[test]
    fn consume_item_click_separates_submenu_and_action_paths() {
        let action = ContextMenuPresentationItem {
            id: "action".to_string(),
            label: "action".to_string(),
            accessibility_label: "action".to_string(),
            icon: None,
            enabled: true,
            checked: false,
            kind: ContextMenuItemKind::Action,
            children: Vec::new(),
        };
        let submenu = ContextMenuPresentationItem {
            id: "submenu".to_string(),
            label: "submenu".to_string(),
            accessibility_label: "submenu".to_string(),
            icon: None,
            enabled: true,
            checked: false,
            kind: ContextMenuItemKind::Submenu,
            children: Vec::new(),
        };

        assert_eq!(
            consume_item_click(&action, vec![0]),
            Some(ContextMenuAction::Activate { path: vec![0] })
        );
        assert_eq!(
            consume_item_click(&submenu, vec![1]),
            Some(ContextMenuAction::OpenSubmenu { path: vec![1] })
        );
        let disabled = ContextMenuPresentationItem {
            id: "disabled".to_string(),
            label: "disabled".to_string(),
            accessibility_label: "disabled".to_string(),
            icon: None,
            enabled: false,
            checked: false,
            kind: ContextMenuItemKind::Action,
            children: Vec::new(),
        };
        assert!(consume_item_click(&disabled, vec![2]).is_none());
    }

    #[test]
    fn keyboard_input_treats_release_and_text_events_as_non_actions() {
        let mut type_ahead = ContextMenuTypeAheadBuffer::new(1000);
        assert!(
            keyboard_input(
                egui::Event::Key {
                    key: egui::Key::Tab,
                    physical_key: None,
                    pressed: false,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
                &mut type_ahead,
                0.0
            )
            .is_none()
        );
        assert_eq!(
            keyboard_input(egui::Event::Text("x".to_string()), &mut type_ahead, 0.0),
            Some(ContextMenuKeyboardInput::TypeAhead("x".to_string()))
        );
        assert!(keyboard_input(egui::Event::Copy, &mut type_ahead, 0.0).is_none());
    }
}
