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

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::molecule::selection::{ContextMenu, ContextMenuItemKind};

    fn key(value: egui::Key) -> egui::Event {
        egui::Event::Key {
            key: value,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    #[test]
    fn keyboard_input_maps_every_supported_navigation_key() {
        let mut type_ahead = ContextMenuTypeAheadBuffer::new(1_000);
        for value in [
            egui::Key::ArrowDown,
            egui::Key::ArrowUp,
            egui::Key::Home,
            egui::Key::End,
            egui::Key::Enter,
            egui::Key::Space,
            egui::Key::ArrowRight,
            egui::Key::ArrowLeft,
            egui::Key::Escape,
        ] {
            assert!(keyboard_input(key(value), &mut type_ahead, 0.0).is_some());
        }
        assert!(keyboard_input(key(egui::Key::F1), &mut type_ahead, 0.0).is_none());
        assert!(keyboard_input(egui::Event::Text(String::new()), &mut type_ahead, 0.0).is_none());
    }

    #[test]
    fn arrow_left_closes_the_current_submenu_path_without_emitting_an_action() {
        let context = egui::Context::default();
        let mut submenu_path = vec![0];
        let mut type_ahead = ContextMenuTypeAheadBuffer::new(1_000);
        let items = vec![ContextMenuPresentationItem::action("child", "Child")];
        let mut actions = None;
        let mut output = context.run_ui(
            egui::RawInput {
                events: vec![key(egui::Key::ArrowLeft)],
                ..egui::RawInput::default()
            },
            |ui| {
                actions = Some(ui.input(|input| {
                    keyboard_actions(input, &items, &mut submenu_path, &[0, 0], &mut type_ahead)
                }));
            },
        );
        output.textures_delta.clear();
        assert!(actions.expect("actions captured").is_empty());
        assert!(submenu_path.is_empty());
    }

    #[test]
    fn activation_without_a_highlight_fails_closed() {
        let context = egui::Context::default();
        let mut submenu_path = Vec::new();
        let mut type_ahead = ContextMenuTypeAheadBuffer::new(1_000);
        let items = vec![ContextMenuPresentationItem::action("item", "Item")];
        let mut actions = None;
        crate::run_ui_discard(
            &context,
            egui::RawInput {
                events: vec![key(egui::Key::Enter)],
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    actions = Some(ui.input(|input| {
                        keyboard_actions(input, &items, &mut submenu_path, &[], &mut type_ahead)
                    }));
                });
            },
        );
        assert!(actions.expect("actions captured").is_empty());
    }

    #[test]
    fn collects_events_from_multiple_actions_and_tracks_submenu_path_pop() {
        let mut menu = ContextMenu::new("id");
        let context_item = ContextMenuPresentationItem {
            id: "parent".into(),
            label: "Parent".into(),
            accessibility_label: String::new(),
            icon: None,
            enabled: true,
            checked: false,
            kind: ContextMenuItemKind::Submenu,
            children: Vec::new(),
        };
        let events = crate::context_menu::presentation::core_items(&[context_item.clone()]);
        menu.synchronize_items(events);
        assert!(
            menu.apply_context_action_events(&ContextMenuAction::OpenSubmenu { path: vec![0] })
                .into_iter()
                .any(|event| matches!(event, ContextMenuEvent::SubmenuOpened { .. }))
        );
        let output = collect_events(
            &mut menu,
            [
                ContextMenuAction::OpenSubmenu { path: vec![0] },
                ContextMenuAction::Close {
                    reason: ContextMenuCloseReason::Escape,
                },
            ],
        );
        assert!(output.iter().any(|event| matches!(
            event,
            ContextMenuEvent::SubmenuOpened { .. } | ContextMenuEvent::Closed { .. }
        )));
    }

    #[test]
    fn type_ahead_event_and_outside_pointer_logic() {
        let item = ContextMenuPresentationItem::action("item", "Item");
        let items = vec![item];
        let mut type_ahead = ContextMenuTypeAheadBuffer::new(1_000);
        let mut submenu = Vec::new();
        let path = vec![0];
        let actions = {
            let context = egui::Context::default();
            let mut actions = Vec::new();
            let mut output = context.run_ui(egui::RawInput::default(), |ui| {
                actions = ui.input(|input| {
                    keyboard_actions(input, &items, &mut submenu, &path, &mut type_ahead)
                });
            });
            output.textures_delta.clear();
            actions
        };
        assert!(actions.is_empty());

        let bounds = katana_ui_core::render_model::UiRect::new(0, 0, 10, 10);
        let mut outside = false;
        let raw = egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(20.0, 20.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        };
        let context = egui::Context::default();
        let mut output = context.run_ui(raw, |ui| {
            outside = ui.input(|input| is_outside_click(input, bounds));
        });
        output.textures_delta.clear();
        assert!(outside);
    }

    #[test]
    fn keyboard_actions_cover_typeahead_submenu_and_escape_intents() {
        let submenu_item = ContextMenuPresentationItem {
            id: "parent".into(),
            label: "Parent".into(),
            accessibility_label: String::new(),
            icon: None,
            enabled: true,
            checked: false,
            kind: ContextMenuItemKind::Submenu,
            children: vec![ContextMenuPresentationItem::action("child", "Child")],
        };
        let context = egui::Context::default();
        for event in [
            egui::Event::Text("p".into()),
            key(egui::Key::ArrowRight),
            key(egui::Key::Escape),
        ] {
            let mut submenu_path = Vec::new();
            let mut type_ahead = ContextMenuTypeAheadBuffer::new(1_000);
            let mut actions = Vec::new();
            crate::run_ui_discard(
                &context,
                egui::RawInput {
                    events: vec![event],
                    ..egui::RawInput::default()
                },
                |ui| {
                    actions = ui.input(|input| {
                        keyboard_actions(
                            input,
                            std::slice::from_ref(&submenu_item),
                            &mut submenu_path,
                            &[0],
                            &mut type_ahead,
                        )
                    });
                },
            );
            assert_eq!(actions.len(), 1);
        }
    }
}
