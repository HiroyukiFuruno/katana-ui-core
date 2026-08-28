use super::text_surface_fixture::{GUTTER_WIDTH, script_line_height};
use super::text_surface_script_types::TextSurfaceScriptStep;

const TEXT_ORIGIN_OFFSET_X: f32 = 32.0;
const DRAG_END_OFFSET_X: f32 = 152.0;
const IME_CARET_OFFSET_X: f32 = 48.0;
const SCRIPT_POINTER_Y: f32 = 12.0;
const WHEEL_LINE_MULTIPLIER: f32 = 3.0;

pub(super) const EXPECTED_SCRIPT_NAMES: [&str; 17] = [
    "idle",
    "hover",
    "focus-press",
    "focus-release",
    "focus-sync",
    "selection-press",
    "selection-drag",
    "selection-release",
    "select-all",
    "copy",
    "history-undo",
    "wheel-scroll",
    "ime-caret-press",
    "ime-caret-release",
    "ime-preedit-star",
    "ime-commit-star",
    "context-target",
];

pub(super) fn scripted_steps() -> Vec<TextSurfaceScriptStep> {
    let text_origin = point(TEXT_ORIGIN_OFFSET_X);
    let drag_end = point(DRAG_END_OFFSET_X);
    let ime_caret = point(IME_CARET_OFFSET_X);
    vec![
        TextSurfaceScriptStep {
            name: "idle",
            events: Vec::new(),
        },
        TextSurfaceScriptStep {
            name: "hover",
            events: vec![egui::Event::PointerMoved(text_origin)],
        },
        TextSurfaceScriptStep {
            name: "focus-press",
            events: vec![pointer_button(
                text_origin,
                egui::PointerButton::Primary,
                true,
            )],
        },
        TextSurfaceScriptStep {
            name: "focus-release",
            events: vec![pointer_button(
                text_origin,
                egui::PointerButton::Primary,
                false,
            )],
        },
        TextSurfaceScriptStep {
            name: "focus-sync",
            events: Vec::new(),
        },
        TextSurfaceScriptStep {
            name: "selection-press",
            events: vec![pointer_button(
                text_origin,
                egui::PointerButton::Primary,
                true,
            )],
        },
        TextSurfaceScriptStep {
            name: "selection-drag",
            events: vec![egui::Event::PointerMoved(drag_end)],
        },
        TextSurfaceScriptStep {
            name: "selection-release",
            events: vec![pointer_button(
                drag_end,
                egui::PointerButton::Primary,
                false,
            )],
        },
        TextSurfaceScriptStep {
            name: "select-all",
            events: vec![key_event(egui::Key::A, command_modifiers())],
        },
        TextSurfaceScriptStep {
            name: "copy",
            events: vec![egui::Event::Copy],
        },
        TextSurfaceScriptStep {
            name: "history-undo",
            events: vec![key_event(egui::Key::Z, command_modifiers())],
        },
        TextSurfaceScriptStep {
            name: "wheel-scroll",
            events: vec![
                egui::Event::PointerMoved(drag_end),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, script_line_height() * WHEEL_LINE_MULTIPLIER),
                    phase: egui::TouchPhase::Move,
                    modifiers: egui::Modifiers::default(),
                },
            ],
        },
        TextSurfaceScriptStep {
            name: "ime-caret-press",
            events: vec![pointer_button(
                ime_caret,
                egui::PointerButton::Primary,
                true,
            )],
        },
        TextSurfaceScriptStep {
            name: "ime-caret-release",
            events: vec![pointer_button(
                ime_caret,
                egui::PointerButton::Primary,
                false,
            )],
        },
        TextSurfaceScriptStep {
            name: "ime-preedit-star",
            events: vec![egui::Event::Ime(egui::ImeEvent::Preedit {
                text: "⭐️".to_string(),
                active_range_chars: None,
            })],
        },
        TextSurfaceScriptStep {
            name: "ime-commit-star",
            events: vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
        },
        TextSurfaceScriptStep {
            name: "context-target",
            events: vec![
                egui::Event::PointerMoved(drag_end),
                pointer_button(drag_end, egui::PointerButton::Secondary, true),
                pointer_button(drag_end, egui::PointerButton::Secondary, false),
            ],
        },
    ]
}

fn point(offset_x: f32) -> egui::Pos2 {
    egui::pos2(GUTTER_WIDTH as f32 + offset_x, SCRIPT_POINTER_Y)
}

fn pointer_button(pos: egui::Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn key_event(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn command_modifiers() -> egui::Modifiers {
    egui::Modifiers {
        command: true,
        ..egui::Modifiers::default()
    }
}
