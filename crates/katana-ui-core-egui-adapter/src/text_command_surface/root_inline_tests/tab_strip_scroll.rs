#[test]
fn tab_strip_manual_horizontal_scroll_changes_artifact_without_a_proposal()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::text_command_surface::{
        TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripTabDescriptor,
        TabStripTabTarget, TabStripText,
    };

    let mut projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"manual-scroll-correlation"),
    );
    for index in 0..6 {
        projection = projection.tab(TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(format!("tab-{index}").into_bytes()),
            TabStripText::new(format!("手動スクロール {index} ⭐️")),
        ));
    }
    let mut root = EguiTextCommandSurfaceRoot::with_identity(
        "tab-strip-manual-scroll-root",
        EguiTextCommandSurface::new(EguiTextSurfaceForTest::surface()),
    )?;
    root.attach_tab_strip(TabStripProjectionLease::new(projection))?;
    let context = context_for_test();
    let before = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(1.0),
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;
    let _ = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(2.0),
            events: vec![
                egui::Event::PointerMoved(egui::pos2(
                    HORIZONTAL_SCROLL_POINTER_X,
                    TAB_STRIP_POINTER_Y,
                )),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(-160.0, 0.0),
                    phase: egui::TouchPhase::Move,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;
    let visible = render_with_input_at_size(
        &context,
        &mut root,
        egui::RawInput {
            time: Some(3.0),
            ..egui::RawInput::default()
        },
        egui::vec2(COMPACT_ROOT_WIDTH, ROOT_FRAME_HEIGHT),
    )?;

    assert_ne!(
        before.evidence_composite.pixel_hash, visible.evidence_composite.pixel_hash,
        "horizontal wheel input must change only the clipped retained tab artifact"
    );

    Ok(())
}
