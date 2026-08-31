use super::{EguiSourceAddressStripAdapter, EguiSourceAddressStripOutput};
use crate::source_address_strip::SourceAddressFrameEventClass;
use crate::source_address_strip::SourceAddressRenderStyle;
use katana_ui_core::atom::{TextAreaEvent, TextAreaKeyChord, TextAreaValidationError};
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressAction, SourceAddressPresentation, SourceAddressStrip,
};
use katana_ui_core::text_surface::TextSurfaceEvent;

const SCREEN_SIZE: egui::Vec2 = egui::vec2(420.0, 80.0);

fn strip() -> SourceAddressStrip {
    SourceAddressStrip::new(SourceAddressPresentation::new(
        "ソース",
        "ソースを入力",
        "ソースを入力",
    ))
}

fn render_source_address_strip(
    context: &egui::Context,
    adapter: &mut EguiSourceAddressStripAdapter,
    strip: &mut SourceAddressStrip,
) -> bool {
    let mut disabled = false;
    crate::run_ui_discard(
        context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
            ..egui::RawInput::default()
        },
        |ui| {
            adapter.show(ui, strip).expect("render succeeds");
            disabled = adapter
                .last_input_artifact
                .as_ref()
                .expect("input surface is produced")
                .record
                .frame
                .accessibility
                .root
                .disabled;
        },
    );
    disabled
}

#[test]
fn source_address_adapter_text_surface_events_are_dispatched_to_output() {
    let adapter = EguiSourceAddressStripAdapter::new("source-address-dispatch-test")
        .expect("adapter should initialize");
    let mut strip = strip();
    let mut output = EguiSourceAddressStripOutput {
        event_classes: Vec::new(),
        submissions: Vec::new(),
    };

    adapter.apply_text_surface_events(
        &mut output,
        &mut strip,
        &[
            TextSurfaceEvent::FocusChanged(true),
            TextSurfaceEvent::TextArea(TextAreaEvent::Change("next".to_owned())),
            TextSurfaceEvent::TextArea(TextAreaEvent::Submit("value".to_owned())),
        ],
    );

    assert!(
        output
            .event_classes
            .contains(&SourceAddressFrameEventClass::Focused)
    );
    assert!(
        output
            .event_classes
            .contains(&SourceAddressFrameEventClass::Submitted)
    );
    assert!(
        output
            .event_classes
            .iter()
            .any(|event| matches!(event, SourceAddressFrameEventClass::DraftChanged))
    );
    let submissions = output.take_submissions();
    assert_eq!(submissions.len(), 1);
    assert_eq!(
        submissions.into_iter().next().map(|item| item.into_draft()),
        Some("next".to_owned())
    );
}

#[test]
fn source_address_adapter_drops_non_text_surface_events_without_output() {
    let adapter = EguiSourceAddressStripAdapter::new("source-address-dispatch-ignore-test")
        .expect("adapter should initialize");
    let mut strip = strip();
    let mut output = EguiSourceAddressStripOutput {
        event_classes: Vec::new(),
        submissions: Vec::new(),
    };

    adapter.apply_text_surface_events(
        &mut output,
        &mut strip,
        &[TextSurfaceEvent::KeyValidationFailed {
            key: TextAreaKeyChord::enter(),
            error: TextAreaValidationError::ConflictingKeyBindings,
        }],
    );
    assert!(output.event_classes.is_empty());
}

#[test]
fn source_address_adapter_displays_input_disabled_after_host_disables_input() {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-disabled-render-test")
        .expect("adapter should initialize");
    let mut strip = strip();

    assert!(!render_source_address_strip(
        &context,
        &mut adapter,
        &mut strip
    ));

    assert!(
        strip
            .apply_action(SourceAddressAction::SetEnabled(false))
            .is_some()
    );
    assert!(render_source_address_strip(
        &context,
        &mut adapter,
        &mut strip
    ));
}

#[test]
fn source_address_adapter_displays_input_enabled_after_host_enables_input() {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-enabled-render-test")
        .expect("adapter should initialize");
    let mut strip = strip();

    assert!(
        strip
            .apply_action(SourceAddressAction::SetEnabled(false))
            .is_some()
    );
    assert!(render_source_address_strip(
        &context,
        &mut adapter,
        &mut strip
    ));

    assert!(
        strip
            .apply_action(SourceAddressAction::SetEnabled(true))
            .is_some()
    );
    assert!(!render_source_address_strip(
        &context,
        &mut adapter,
        &mut strip
    ));
}

#[test]
fn source_address_adapter_preserves_raster_evidence_only_on_successful_frame() {
    let context = egui::Context::default();
    let mut adapter = EguiSourceAddressStripAdapter::new("source-address-failed-frame")
        .expect("adapter should initialize");
    let mut strip = strip();

    assert!(!render_source_address_strip(
        &context,
        &mut adapter,
        &mut strip
    ));
    assert!(adapter.raster_evidence().is_some());

    let mut failed_result = None;
    let mut style = SourceAddressRenderStyle::default();
    style.input_raster.font.size = f32::NAN;
    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
            ..egui::RawInput::default()
        },
        |ui| {
            failed_result = Some(adapter.show_with_style(ui, &mut strip, &style));
        },
    );
    let error = match failed_result.expect("failed frame render attempted") {
        Ok(_) => panic!("invalid style should fail and return error"),
        Err(error) => error,
    };

    assert!(
        error
            .to_string()
            .contains("source-address text surface failed")
    );
    assert!(adapter.raster_evidence().is_none());
}
