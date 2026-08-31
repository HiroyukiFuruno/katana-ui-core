use super::super::paint::DiagnosticsPaint;
use super::super::types::{DiagnosticsListStyle, EguiDiagnosticsListOutput};
use super::EguiDiagnosticsListAdapter;
use katana_ui_core::molecule::{
    CodeDiff, DiagnosticAction, DiagnosticFixPreview, DiagnosticItem, DiagnosticLocation,
    DiagnosticSeverity, DiagnosticsList, DiagnosticsListEvent,
};

const TEST_SCREEN_SIZE: egui::Vec2 = egui::vec2(900.0, 240.0);
const EXTRA_ITEM_COUNT: u32 = 5;

fn run_show_frame(
    context: &egui::Context,
    adapter: &mut EguiDiagnosticsListAdapter,
    diagnostics: &mut DiagnosticsList,
    events: Vec<egui::Event>,
) -> EguiDiagnosticsListOutput {
    let mut result = None;
    crate::run_ui_discard(
        context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                TEST_SCREEN_SIZE,
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                result = Some(adapter.show(ui, diagnostics));
            });
        },
    );
    result
        .expect("diagnostics frame should run")
        .expect("diagnostics adapter should render")
}

fn diagnostics_with_first_item(item: DiagnosticItem) -> DiagnosticsList {
    (0..EXTRA_ITEM_COUNT).fold(
        DiagnosticsList::new("診断").item(item),
        |diagnostics, index| {
            diagnostics.item(DiagnosticItem::new(
                format!("plain-{index}"),
                DiagnosticSeverity::Warning,
                format!("通常項目 {index}"),
                DiagnosticLocation::new("src/lib.rs", index + 2, 1),
            ))
        },
    )
}

fn quickfix_diagnostics() -> DiagnosticsList {
    diagnostics_with_first_item(
        DiagnosticItem::new(
            "quick",
            DiagnosticSeverity::Error,
            "クイック修正対象",
            DiagnosticLocation::new("src/lib.rs", 1, 1),
        )
        .quickfix(DiagnosticAction::new("quickfix", "Quick fix")),
    )
}

fn disclosure_diagnostics() -> DiagnosticsList {
    diagnostics_with_first_item(
        DiagnosticItem::new(
            "preview",
            DiagnosticSeverity::Error,
            "修正プレビュー対象",
            DiagnosticLocation::new("src/lib.rs", 1, 1),
        )
        .fix_preview(DiagnosticFixPreview::new(CodeDiff::new("preview"))),
    )
}

fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn click_events(pointer: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(pointer),
        egui::Event::PointerButton {
            pos: pointer,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: pointer,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

#[test]
fn partially_clipped_disclosure_only_accepts_clicks_inside_visible_row() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-partial-disclosure")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = disclosure_diagnostics();
    let style = DiagnosticsListStyle::standard();
    let clipped_amount = style.row_height / 2.0;
    adapter.scroll_y = clipped_amount;

    let initial = run_show_frame(&context, &mut adapter, &mut diagnostics, Vec::new());
    let surface = DiagnosticsPaint::egui_rect(initial.paint_plan.surface_bounds);
    let viewport_top = surface.top() + style.header_height;
    let disclosure_center_x = surface.left()
        + super::super::paint::DIAGNOSTICS_SMALL_INSET
        + super::super::paint::DIAGNOSTICS_DISCLOSURE_WIDTH / 2.0;
    let visible_first_row_height = style.row_height - clipped_amount;
    let inside = egui::pos2(
        disclosure_center_x,
        viewport_top + visible_first_row_height / 2.0,
    );
    let outside = egui::pos2(
        disclosure_center_x,
        viewport_top + visible_first_row_height + style.row_height / 4.0,
    );

    let outside_output = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        click_events(outside),
    );
    assert!(outside_output.events.iter().any(|event| {
        matches!(event, DiagnosticsListEvent::DiagnosticSelected { id } if id.as_str() == "plain-0")
    }));
    assert!(!outside_output.events.iter().any(|event| {
        matches!(event, DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, .. } if id.as_str() == "preview")
    }));

    let inside_output = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        click_events(inside),
    );
    assert!(inside_output.events.iter().any(|event| {
        matches!(event, DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded: true } if id.as_str() == "preview")
    }));
}

#[test]
fn focused_quickfix_activates_once_for_enter_and_space_from_real_egui_input() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let context = egui::Context::default();
        let mut adapter = EguiDiagnosticsListAdapter::new(("diagnostics-quickfix-keyboard", key))
            .expect("diagnostics adapter should initialize");
        let mut diagnostics = quickfix_diagnostics();
        let quickfix_id = adapter.id.with(("quick", "quickfix"));
        let _ = run_show_frame(&context, &mut adapter, &mut diagnostics, Vec::new());

        for _ in 0..12 {
            if context.memory(|memory| memory.focused()) == Some(quickfix_id) {
                break;
            }
            let _ = run_show_frame(
                &context,
                &mut adapter,
                &mut diagnostics,
                vec![key_event(egui::Key::Tab)],
            );
        }
        assert_eq!(
            context.memory(|memory| memory.focused()),
            Some(quickfix_id),
            "Tab must focus the quickfix before {key:?} activation"
        );

        let output = run_show_frame(
            &context,
            &mut adapter,
            &mut diagnostics,
            vec![key_event(key)],
        );
        assert_eq!(
            output
                .events
                .iter()
                .filter(|event| {
                    matches!(event, DiagnosticsListEvent::DiagnosticFixApplied { id } if id.as_str() == "quick")
                })
                .count(),
            1,
            "{key:?} activates the focused quickfix exactly once"
        );
    }
}
