use super::EguiStatusBarAdapter;
use katana_ui_core::molecule::{
    StatusBar, StatusBarAction, StatusBarEvent, StatusBarMode, StatusBarPopoverSpec,
    StatusBarSegment,
};

#[test]
fn fresh_adapter_exposes_empty_artifact_and_raster_evidence() {
    let adapter = EguiStatusBarAdapter::new("status-bar-default-evidence")
        .expect("status bar adapter should retain its platform rasterizer");

    assert!(adapter.artifact_paint_plan().is_none());
    assert!(adapter.raster_evidence().is_empty());
}

#[test]
fn unit_adapter_renders_single_message_and_closes_an_open_popover_from_escape() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-unit-render")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut single = StatusBar::new("single")
        .mode(StatusBarMode::SingleMessage)
        .message("ready");
    let mut single_output = None;
    crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        single_output = Some(adapter.show(ui, &mut single));
    });
    assert!(
        single_output
            .expect("single-message frame runs")
            .expect("single-message frame renders")
            .events()
            .is_empty()
    );

    let mut with_popover = StatusBar::new("popover")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("details", "Details")
                .popover(StatusBarPopoverSpec::new("Status detail", "Current status")),
        );
    let opened = with_popover.apply_action(&StatusBarAction::PressSegment {
        id: "details".to_owned(),
    });
    assert!(opened.contains(&StatusBarEvent::SegmentPopoverOpened {
        id: "details".to_owned(),
    }));

    let mut escaped_output = None;
    crate::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| escaped_output = Some(adapter.show(ui, &mut with_popover)),
    );
    let escaped = escaped_output
        .expect("escape frame runs")
        .expect("escape frame renders");
    assert!(
        escaped
            .events()
            .contains(&StatusBarEvent::SegmentPopoverClosed {
                id: "details".to_owned(),
            })
    );
    assert!(with_popover.state().open_popover().is_none());
}
