use super::*;

#[test]
fn meter_and_segment_accessors_return_stored_values() {
    let meter = ProgressMeterSpec::new(ProgressMeterShape::Linear, 250)
        .label("Build progress")
        .tone(UiTone::Success)
        .tooltip("Completion percentage");
    assert_eq!(meter.percent(), 100);
    assert_eq!(meter.label_text(), "Build progress");
    assert_eq!(meter.tone_value(), UiTone::Success);
    assert_eq!(meter.tooltip_text(), "Completion percentage");

    let popover = StatusBarPopoverSpec::new("Status hint", "Tooltip detail");
    let segment = StatusBarSegment::new("seg-1", "Segment")
        .alignment(StatusBarSegmentAlignment::Trailing)
        .tooltip("Segment tooltip")
        .popover(popover)
        .progress(ProgressMeterSpec::new(ProgressMeterShape::Ring, 40))
        .accessibility_label("Accessible segment")
        .interactive(true);

    assert_eq!(
        segment.alignment_value(),
        StatusBarSegmentAlignment::Trailing
    );
    assert_eq!(segment.tooltip_text(), Some("Segment tooltip"));
    assert_eq!(segment.accessibility_label_text(), "Accessible segment");
    assert!(segment.is_interactive());
    assert!(segment.popover_spec().is_some());
    assert!(segment.progress_spec().is_some());
    assert_eq!(segment.id(), "seg-1");
    assert_eq!(segment.label(), "Segment");
    assert_eq!(segment.icon_name(), None);
    assert_eq!(
        segment.progress_spec().map(ProgressMeterSpec::percent),
        Some(40)
    );
}
