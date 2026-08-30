use super::super::super::types::{DiagnosticsListPaintPlan, EguiDiagnosticsListOutput};
use super::super::EguiDiagnosticsListAdapter;
use katana_ui_core::molecule::{
    DiagnosticAction, DiagnosticId, DiagnosticItem, DiagnosticLocation, DiagnosticSeverity,
    DiagnosticsList, DiagnosticsListAction,
};
use katana_ui_core::render_model::UiRect;

#[test]
fn process_keyboard_accepts_shift_f8_from_real_egui_input() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-input-shift-f8")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = DiagnosticsList::new("診断").item(DiagnosticItem::new(
        "item",
        DiagnosticSeverity::Error,
        "エラー",
        DiagnosticLocation::new("src/lib.rs", 1, 1),
    ));
    let mut output = EguiDiagnosticsListOutput {
        events: Vec::new(),
        paint_plan: DiagnosticsListPaintPlan {
            surface_bounds: UiRect::new(0, 0, 200, 40),
            operations: Vec::new(),
        },
    };
    let shift = egui::Modifiers {
        shift: true,
        ..egui::Modifiers::NONE
    };

    let mut full_output = context.run_ui(
        egui::RawInput {
            events: vec![
                egui::Event::ModifiersChanged(shift),
                egui::Event::Key {
                    key: egui::Key::F8,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: shift,
                },
            ],
            ..egui::RawInput::default()
        },
        |ui| {
            adapter.process_keyboard(
                ui,
                &mut output,
                &[20.0],
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 40.0)),
                0.0,
                &mut diagnostics,
            );
        },
    );
    full_output.textures_delta.clear();

    assert_eq!(output.events.len(), 1);
}

#[test]
fn process_keyboard_accepts_f8_from_real_egui_input() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-input-f8")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = DiagnosticsList::new("診断").item(DiagnosticItem::new(
        "item",
        DiagnosticSeverity::Error,
        "エラー",
        DiagnosticLocation::new("src/lib.rs", 1, 1),
    ));
    let mut output = EguiDiagnosticsListOutput {
        events: Vec::new(),
        paint_plan: DiagnosticsListPaintPlan {
            surface_bounds: UiRect::new(0, 0, 200, 40),
            operations: Vec::new(),
        },
    };

    let mut full_output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::F8,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            adapter.process_keyboard(
                ui,
                &mut output,
                &[20.0],
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 40.0)),
                0.0,
                &mut diagnostics,
            );
        },
    );
    full_output.textures_delta.clear();

    assert_eq!(output.events.len(), 1);
}

#[test]
fn process_keyboard_routes_arrow_right_as_scope_next_when_scope_is_focused_from_real_egui_input() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-input-scope-next")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = DiagnosticsList::new("診断")
        .scope("scope", "Scope", "Scope")
        .scope("next", "Next", "Next")
        .item(DiagnosticItem::new(
            "item",
            DiagnosticSeverity::Error,
            "エラー",
            DiagnosticLocation::new("src/lib.rs", 1, 1),
        ));
    let mut output = EguiDiagnosticsListOutput {
        events: Vec::new(),
        paint_plan: DiagnosticsListPaintPlan {
            surface_bounds: UiRect::new(0, 0, 200, 40),
            operations: Vec::new(),
        },
    };
    adapter.focused_scope = Some("scope".to_owned());
    context.memory_mut(|memory| {
        memory.request_focus(
            adapter
                .id
                .with(super::super::super::identity::DiagnosticsTargetIdentity::scope("scope")),
        );
    });

    let mut full_output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::ArrowRight,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            adapter.process_keyboard(
                ui,
                &mut output,
                &[20.0],
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 40.0)),
                0.0,
                &mut diagnostics,
            );
        },
    );
    full_output.textures_delta.clear();

    assert_eq!(output.events.len(), 1);
}

#[test]
fn process_keyboard_ignores_arrow_enter_space_when_not_focused() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-input-ignore")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = DiagnosticsList::new("診断").item(
        DiagnosticItem::new(
            "item",
            DiagnosticSeverity::Error,
            "エラー",
            DiagnosticLocation::new("src/lib.rs", 1, 1),
        )
        .quickfix(DiagnosticAction::new("quickfix", "Quick fix")),
    );
    diagnostics.apply_action(DiagnosticsListAction::Select(DiagnosticId::new("item")));
    adapter.focused_item = Some("item".to_owned());
    context.memory_mut(|memory| memory.request_focus(egui::Id::new("other-control")));

    for key in [
        egui::Key::ArrowUp,
        egui::Key::ArrowDown,
        egui::Key::ArrowLeft,
        egui::Key::ArrowRight,
        egui::Key::Enter,
        egui::Key::Space,
    ] {
        let mut output = EguiDiagnosticsListOutput {
            events: Vec::new(),
            paint_plan: DiagnosticsListPaintPlan {
                surface_bounds: UiRect::new(0, 0, 200, 40),
                operations: Vec::new(),
            },
        };

        let mut full_output = context.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..egui::RawInput::default()
            },
            |ui| {
                adapter.process_keyboard(
                    ui,
                    &mut output,
                    &[20.0],
                    egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 40.0)),
                    0.0,
                    &mut diagnostics,
                );
            },
        );
        full_output.textures_delta.clear();

        assert_eq!(
            output.events.len(),
            0,
            "{key:?} should not be handled when not focused"
        );
    }
}

#[test]
fn process_keyboard_accepts_arrow_enter_space_when_focused() {
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-input-accept")
        .expect("diagnostics adapter should initialize");
    let context = egui::Context::default();

    let make_diagnostics = || {
        DiagnosticsList::new("診断")
            .item(
                DiagnosticItem::new(
                    "item",
                    DiagnosticSeverity::Error,
                    "エラー",
                    DiagnosticLocation::new("src/lib.rs", 1, 1),
                )
                .quickfix(DiagnosticAction::new("quickfix", "Quick fix")),
            )
            .item(DiagnosticItem::new(
                "next",
                DiagnosticSeverity::Warning,
                "警告",
                DiagnosticLocation::new("src/lib.rs", 1, 2),
            ))
    };

    for (key, selected, expanded_before_key_left) in [
        (egui::Key::ArrowUp, "next", false),
        (egui::Key::ArrowDown, "item", false),
        (egui::Key::ArrowLeft, "item", true),
        (egui::Key::ArrowRight, "item", false),
        (egui::Key::Enter, "item", false),
        (egui::Key::Space, "item", false),
    ] {
        let mut diagnostics = make_diagnostics();
        diagnostics.apply_action(DiagnosticsListAction::Select(DiagnosticId::new(selected)));
        if expanded_before_key_left {
            diagnostics.apply_action(DiagnosticsListAction::ToggleFixPreview(DiagnosticId::new(
                "item",
            )));
        }

        let mut expected = diagnostics.clone();
        let expected_events = expected.apply_action(DiagnosticsListAction::Keyboard(match key {
            egui::Key::ArrowUp => katana_ui_core::molecule::DiagnosticKeyboardInput::ArrowUp,
            egui::Key::ArrowDown => katana_ui_core::molecule::DiagnosticKeyboardInput::ArrowDown,
            egui::Key::ArrowLeft => katana_ui_core::molecule::DiagnosticKeyboardInput::ArrowLeft,
            egui::Key::ArrowRight => katana_ui_core::molecule::DiagnosticKeyboardInput::ArrowRight,
            egui::Key::Enter => katana_ui_core::molecule::DiagnosticKeyboardInput::Enter,
            egui::Key::Space => katana_ui_core::molecule::DiagnosticKeyboardInput::Space,
            _ => unreachable!(),
        }));
        let expected_events_count = expected_events.len();

        let mut output = EguiDiagnosticsListOutput {
            events: Vec::new(),
            paint_plan: DiagnosticsListPaintPlan {
                surface_bounds: UiRect::new(0, 0, 200, 40),
                operations: Vec::new(),
            },
        };
        adapter.focused_item = Some(selected.to_owned());
        context.memory_mut(|memory| memory.request_focus(adapter.id.with(selected)));

        let mut full_output = context.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..egui::RawInput::default()
            },
            |ui| {
                adapter.process_keyboard(
                    ui,
                    &mut output,
                    &[20.0],
                    egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 40.0)),
                    0.0,
                    &mut diagnostics,
                );
            },
        );
        full_output.textures_delta.clear();

        assert_eq!(
            output.events.len(),
            expected_events_count,
            "{key:?} should be handled when focused"
        );
    }
}
