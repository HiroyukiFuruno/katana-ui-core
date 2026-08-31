use super::super::paint::{
    DIAGNOSTICS_QUICKFIX_RIGHT_INSET, DIAGNOSTICS_QUICKFIX_WIDTH, DiagnosticsPaint,
};
use super::super::types::{DiagnosticsListStyle, EguiDiagnosticsListOutput};
use super::EguiDiagnosticsListAdapter;
use katana_ui_core::molecule::{
    DiagnosticAction, DiagnosticItem, DiagnosticLocation, DiagnosticSeverity, DiagnosticsList,
    DiagnosticsListEvent,
};

const TEST_SCREEN_WIDTH: f32 = 900.0;
const TEST_SCREEN_HEIGHT: f32 = 240.0;
const MANY_ITEM_COUNT: u32 = 20;
const EXTRA_ITEM_COUNT: u32 = 5;
const TEST_WHEEL_DELTA_Y: f32 = -16.0;

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
                egui::vec2(TEST_SCREEN_WIDTH, TEST_SCREEN_HEIGHT),
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

fn diagnostics_with_many_items() -> DiagnosticsList {
    (0..MANY_ITEM_COUNT).fold(DiagnosticsList::new("診断"), |diagnostics, index| {
        diagnostics.item(DiagnosticItem::new(
            format!("item-{index}"),
            DiagnosticSeverity::Error,
            format!("項目 {index}"),
            DiagnosticLocation::new(format!("src/{index}.rs"), index + 1, 1),
        ))
    })
}

fn diagnostics_with_quickfix_first() -> DiagnosticsList {
    (0..EXTRA_ITEM_COUNT).fold(
        DiagnosticsList::new("診断").item(
            DiagnosticItem::new(
                "quick",
                DiagnosticSeverity::Error,
                "クイック修正対象",
                DiagnosticLocation::new("src/lib.rs", 1, 1),
            )
            .quickfix(DiagnosticAction::new("quickfix", "Quick fix")),
        ),
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

fn wheel_event() -> egui::Event {
    egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        delta: egui::vec2(0.0, TEST_WHEEL_DELTA_Y),
        phase: egui::TouchPhase::Move,
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
fn scroll_wheel_only_changes_scroll_while_viewport_is_hovered() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-scroll-hover")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = diagnostics_with_many_items();
    let initial = run_show_frame(&context, &mut adapter, &mut diagnostics, Vec::new());
    let surface = DiagnosticsPaint::egui_rect(initial.paint_plan.surface_bounds);
    let style = DiagnosticsListStyle::standard();
    let header_pointer = egui::pos2(
        surface.center().x,
        surface.top() + style.header_height / 2.0,
    );
    let viewport_pointer = egui::pos2(
        surface.center().x,
        surface.top() + style.header_height + style.row_height / 2.0,
    );

    let _ = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::PointerMoved(header_pointer)],
    );
    let _ = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![wheel_event()],
    );
    assert_eq!(adapter.scroll_y(), 0.0);

    let _ = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![egui::Event::PointerMoved(viewport_pointer)],
    );
    let _ = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        vec![wheel_event()],
    );
    assert!(adapter.scroll_y() > 0.0);
}

#[test]
fn partially_clipped_quickfix_only_accepts_clicks_inside_visible_row() {
    let context = egui::Context::default();
    let mut adapter = EguiDiagnosticsListAdapter::new("diagnostics-partial-quickfix")
        .expect("diagnostics adapter should initialize");
    let mut diagnostics = diagnostics_with_quickfix_first();
    let style = DiagnosticsListStyle::standard();
    let clipped_amount = style.row_height / 2.0;
    adapter.scroll_y = clipped_amount;

    let initial = run_show_frame(&context, &mut adapter, &mut diagnostics, Vec::new());
    let surface = DiagnosticsPaint::egui_rect(initial.paint_plan.surface_bounds);
    let viewport_top = surface.top() + style.header_height;
    let quickfix_left = surface.right() - DIAGNOSTICS_QUICKFIX_RIGHT_INSET;
    let quickfix_center_x = quickfix_left + DIAGNOSTICS_QUICKFIX_WIDTH / 2.0;
    let visible_first_row_height = style.row_height - clipped_amount;
    let inside = egui::pos2(
        quickfix_center_x,
        viewport_top + visible_first_row_height / 2.0,
    );
    let outside = egui::pos2(
        quickfix_center_x,
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
        matches!(event, DiagnosticsListEvent::DiagnosticFixApplied { id } if id.as_str() == "quick")
    }));

    let inside_output = run_show_frame(
        &context,
        &mut adapter,
        &mut diagnostics,
        click_events(inside),
    );
    assert!(inside_output.events.iter().any(|event| {
        matches!(event, DiagnosticsListEvent::DiagnosticFixApplied { id } if id.as_str() == "quick")
    }));
}
