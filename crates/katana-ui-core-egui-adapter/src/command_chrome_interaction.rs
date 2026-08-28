use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbar, CommandChromeToolbarAction,
    CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::toolbar::ToolbarKeyboardInput;
use katana_ui_core::render_model::UiRect;

pub(super) struct CommandChromeInteraction;

impl CommandChromeInteraction {
    pub(super) fn keyboard_events(
        ui: &egui::Ui,
        toolbar: &mut CommandChromeToolbar,
        pointer_activation_consumed: bool,
    ) -> Vec<CommandChromeToolbarEvent> {
        let mut pointer_activation_consumed = pointer_activation_consumed;
        ui.input(|input| input.events.clone())
            .into_iter()
            .filter_map(keyboard_input)
            .filter_map(|input| {
                consume_pointer_activation_key(input, &mut pointer_activation_consumed)
            })
            .flat_map(|input| toolbar.apply_action(CommandChromeToolbarAction::Keyboard { input }))
            .collect()
    }

    pub(super) fn publish_button_accesskit(
        ui: &egui::Ui,
        id: egui::Id,
        action: &CommandChromeAction,
        bounds: UiRect,
        target_class: crate::text_command_surface::accesskit_evidence::AccessKitTargetClass,
    ) {
        let label = action
            .accessibility_label_model()
            .or_else(|| action.tooltip_model())
            .map_or_else(|| action.label_model().to_string(), Clone::clone);
        Self::publish_labeled_button_accesskit(
            ui,
            id,
            &label,
            action.disabled_model(),
            bounds,
            action.id().as_str(),
            target_class,
        );
    }

    pub(super) fn publish_labeled_button_accesskit(
        ui: &egui::Ui,
        id: egui::Id,
        label: &str,
        disabled: bool,
        bounds: UiRect,
        target_identity: &str,
        target_class: crate::text_command_surface::accesskit_evidence::AccessKitTargetClass,
    ) {
        ui.ctx().accesskit_node_builder(id, |node| {
            node.set_role(egui::accesskit::Role::Button);
            node.set_label(label);
            node.set_bounds(egui::accesskit::Rect {
                x0: bounds.x.into(),
                y0: bounds.y.into(),
                x1: bounds.x.saturating_add(bounds.width as i32).into(),
                y1: bounds.y.saturating_add(bounds.height as i32).into(),
            });
            node.add_action(egui::accesskit::Action::Click);
            if disabled {
                node.set_disabled();
            }
        });
        crate::text_command_surface::accesskit_evidence::AccessKitEvidenceLedger::record(
            ui.ctx(),
            crate::text_command_surface::accesskit_evidence::AccessKitEvidence {
                response_id: id,
                bounds,
                label: label.to_owned(),
                disabled,
                target_identity: target_identity.to_owned(),
                target_class,
            },
        );
    }
}

fn keyboard_input(event: egui::Event) -> Option<ToolbarKeyboardInput> {
    let egui::Event::Key {
        key, pressed: true, ..
    } = event
    else {
        return None;
    };
    match key {
        egui::Key::ArrowLeft => Some(ToolbarKeyboardInput::ArrowLeft),
        egui::Key::ArrowRight => Some(ToolbarKeyboardInput::ArrowRight),
        egui::Key::ArrowUp => Some(ToolbarKeyboardInput::ArrowUp),
        egui::Key::ArrowDown => Some(ToolbarKeyboardInput::ArrowDown),
        egui::Key::Home => Some(ToolbarKeyboardInput::Home),
        egui::Key::End => Some(ToolbarKeyboardInput::End),
        egui::Key::Enter => Some(ToolbarKeyboardInput::Enter),
        egui::Key::Space => Some(ToolbarKeyboardInput::Space),
        egui::Key::Escape => Some(ToolbarKeyboardInput::Escape),
        _ => None,
    }
}

fn consume_pointer_activation_key(
    input: ToolbarKeyboardInput,
    consumed: &mut bool,
) -> Option<ToolbarKeyboardInput> {
    if *consumed
        && matches!(
            input,
            ToolbarKeyboardInput::Enter | ToolbarKeyboardInput::Space
        )
    {
        *consumed = false;
        None
    } else {
        Some(input)
    }
}

#[cfg(test)]
mod tests {
    use super::CommandChromeInteraction;
    use katana_ui_core::molecule::command_chrome::{
        CommandChromeAction, CommandChromeToolbar, CommandChromeToolbarEvent,
    };

    #[test]
    fn pointer_activation_consumes_raw_activation_key_but_preserves_navigation() {
        for navigation_key in [
            egui::Key::ArrowLeft,
            egui::Key::ArrowRight,
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::Home,
            egui::Key::End,
            egui::Key::Escape,
        ] {
            for activation_key in [egui::Key::Enter, egui::Key::Space] {
                let context = egui::Context::default();
                let mut toolbar =
                    CommandChromeToolbar::new().action(CommandChromeAction::new("action", "操作"));
                let mut events = None;
                crate::run_ui_discard(
                    &context,
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(320.0, 120.0),
                        )),
                        events: vec![
                            egui::Event::Key {
                                key: navigation_key,
                                physical_key: None,
                                pressed: true,
                                repeat: false,
                                modifiers: egui::Modifiers::default(),
                            },
                            egui::Event::Key {
                                key: activation_key,
                                physical_key: None,
                                pressed: true,
                                repeat: false,
                                modifiers: egui::Modifiers::default(),
                            },
                        ],
                        ..egui::RawInput::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            events = Some(CommandChromeInteraction::keyboard_events(
                                ui,
                                &mut toolbar,
                                true,
                            ));
                        });
                    },
                );
                let events = events.expect("keyboard events collected");
                assert!(
                    events.iter().any(|event| matches!(
                        event,
                        CommandChromeToolbarEvent::FocusChanged { .. }
                    ))
                );
                assert!(!events.iter().any(|event| matches!(
                    event,
                    CommandChromeToolbarEvent::CommandActivated { .. }
                )));
            }
        }

        for key in [egui::Key::Enter, egui::Key::Space] {
            let context = egui::Context::default();
            let mut toolbar =
                CommandChromeToolbar::new().action(CommandChromeAction::new("action", "操作"));
            let mut events = None;
            crate::run_ui_discard(
                &context,
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(320.0, 120.0),
                    )),
                    events: vec![egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::default(),
                    }],
                    ..egui::RawInput::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        events = Some(CommandChromeInteraction::keyboard_events(
                            ui,
                            &mut toolbar,
                            false,
                        ));
                    });
                },
            );
            let events = events.expect("unconsumed keyboard events collected");
            assert!(
                events.iter().any(|event| matches!(
                    event,
                    CommandChromeToolbarEvent::CommandActivated { .. }
                ))
            );
        }
    }
}
