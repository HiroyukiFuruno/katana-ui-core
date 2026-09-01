const CLOSEABLE_GROUP_OPAQUE_TARGET: [u8; 1] = [0xaa];
const SELECTABLE_GROUP_OPAQUE_TARGET: [u8; 2] = [0xaa, 0xbb];
const SELECTABLE_SECOND_TAB_OPAQUE_TARGET: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

fn empty_projection() -> SanitizedTabProjection {
    SanitizedTabProjection::new([])
}

fn closeable_projection() -> SanitizedTabProjection {
    SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes(CLOSEABLE_GROUP_OPAQUE_TARGET),
        0,
        "Documents",
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x01]), 0, "First")
            .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
    )
    .tab(
        SanitizedTab::new(SanitizedTabTarget::from_opaque_bytes([0x02]), 1, "Second")
            .with_capabilities(SanitizedTabCapabilities::new().close_state(true))
            .with_close_presentation(SanitizedTabClosePresentation::new(
                "×",
                "Close tab",
                "Close second tab",
            )),
    )])
}

fn selectable_projection() -> SanitizedTabProjection {
    SanitizedTabProjection::new([SanitizedTabGroup::new(
        SanitizedTabGroupTarget::from_opaque_bytes(SELECTABLE_GROUP_OPAQUE_TARGET),
        0,
        "Documents",
    )
    .tab(
        SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([0x01, 0x02]),
            0,
            "First",
        )
        .with_capabilities(SanitizedTabCapabilities::new().active_state(true)),
    )
    .tab(
        SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes(SELECTABLE_SECOND_TAB_OPAQUE_TARGET),
            1,
            "Second",
        )
        .with_icon(UiIconProps::new("<svg/>")),
    )])
}

fn run_frame(
    context: &egui::Context,
    adapter: &mut SanitizedTabProjectionAdapter,
    events: Vec<egui::Event>,
) -> super::SanitizedTabProjectionFrame {
    let mut output = None;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN_SIZE)),
            events,
            ..egui::RawInput::default()
        },
        |ui| output = Some(adapter.show(ui)),
    );
    platform_output.textures_delta.clear();
    output.expect("sanitized tab frame is produced")
}

fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}
