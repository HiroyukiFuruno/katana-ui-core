use super::{consume_pointer_activation_key, keyboard_events, keyboard_input};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeToolbar, CommandChromeToolbarEvent,
};
use katana_ui_core::molecule::toolbar::ToolbarKeyboardInput;

const RUN_UI_SCREEN_WIDTH_PX: f32 = 320.0;
const RUN_UI_SCREEN_HEIGHT_PX: f32 = 120.0;

fn pressed_key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

fn run_events_with_input(
    events: Vec<egui::Event>,
    consumed: bool,
) -> Vec<CommandChromeToolbarEvent> {
    let context = egui::Context::default();
    let mut toolbar =
        CommandChromeToolbar::new().action(CommandChromeAction::new("action", "操作"));
    let mut output_events = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(RUN_UI_SCREEN_WIDTH_PX, RUN_UI_SCREEN_HEIGHT_PX),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                output_events = Some(keyboard_events(ui, &mut toolbar, consumed));
            });
        },
    );
    output.textures_delta.clear();
    output_events.expect("keyboard events collected")
}

#[test]
fn pointer_activation_consumes_raw_activation_key_but_preserves_navigation() {
    for navigation_key in [
        egui::Key::ArrowLeft,
        egui::Key::ArrowRight,
        egui::Key::Home,
        egui::Key::End,
    ] {
        for activation_key in [egui::Key::Enter, egui::Key::Space] {
            let events = run_events_with_input(
                vec![
                    pressed_key_event(navigation_key),
                    pressed_key_event(activation_key),
                ],
                true,
            );
            assert!(
                events
                    .iter()
                    .any(|event| matches!(event, CommandChromeToolbarEvent::FocusChanged { .. }))
            );
            assert!(
                !events.iter().any(|event| matches!(
                    event,
                    CommandChromeToolbarEvent::CommandActivated { .. }
                ))
            );
        }
    }
}

#[test]
fn pointer_activation_consumes_activation_key_before_navigation_when_arrives_first() {
    let events = run_events_with_input(
        vec![
            pressed_key_event(egui::Key::Enter),
            pressed_key_event(egui::Key::ArrowLeft),
        ],
        true,
    );

    assert!(
        events
            .iter()
            .any(|event| matches!(event, CommandChromeToolbarEvent::FocusChanged { .. }))
    );
    assert!(
        !events
            .iter()
            .any(|event| matches!(event, CommandChromeToolbarEvent::CommandActivated { .. }))
    );
}

#[test]
fn keyboard_input_maps_expected_navigation_and_activation_keys() {
    let cases = [
        (egui::Key::ArrowLeft, Some(ToolbarKeyboardInput::ArrowLeft)),
        (
            egui::Key::ArrowRight,
            Some(ToolbarKeyboardInput::ArrowRight),
        ),
        (egui::Key::ArrowUp, Some(ToolbarKeyboardInput::ArrowUp)),
        (egui::Key::ArrowDown, Some(ToolbarKeyboardInput::ArrowDown)),
        (egui::Key::Home, Some(ToolbarKeyboardInput::Home)),
        (egui::Key::End, Some(ToolbarKeyboardInput::End)),
        (egui::Key::Enter, Some(ToolbarKeyboardInput::Enter)),
        (egui::Key::Space, Some(ToolbarKeyboardInput::Space)),
        (egui::Key::Escape, Some(ToolbarKeyboardInput::Escape)),
    ];
    for (input, expected) in cases {
        assert_eq!(expected, keyboard_input(pressed_key_event(input)));
    }

    assert_eq!(None, keyboard_input(egui::Event::Text("a".to_owned())));
    assert_eq!(
        None,
        keyboard_input(egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        })
    );
}

#[test]
fn consume_pointer_activation_key_only_filters_consumed_activation_events() {
    let mut consumed = true;
    assert_eq!(
        None,
        consume_pointer_activation_key(ToolbarKeyboardInput::Enter, &mut consumed)
    );
    assert!(!consumed);

    let mut retained = true;
    assert_eq!(
        Some(ToolbarKeyboardInput::ArrowLeft),
        consume_pointer_activation_key(ToolbarKeyboardInput::ArrowLeft, &mut retained),
    );
    assert!(retained);

    let mut retained = true;
    assert_eq!(
        Some(ToolbarKeyboardInput::Escape),
        consume_pointer_activation_key(ToolbarKeyboardInput::Escape, &mut retained),
    );
    assert!(retained);
}
