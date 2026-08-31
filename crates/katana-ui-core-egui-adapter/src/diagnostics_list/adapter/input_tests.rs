use super::super::super::types::{DiagnosticsListPaintPlan, EguiDiagnosticsListOutput};
use super::super::EguiDiagnosticsListAdapter;
use katana_ui_core::molecule::{
    DiagnosticItem, DiagnosticLocation, DiagnosticSeverity, DiagnosticsList,
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
