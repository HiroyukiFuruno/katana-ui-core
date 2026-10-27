use super::{TabStripDropIndicator, drop_indicator};

fn build_state() -> super::TabStripRetainedState {
    let config = crate::text_raster::PlatformTextRasterConfig::default();
    let catalog = std::sync::Arc::new(crate::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    super::TabStripRetainedState::from_lease(
        crate::egui::text_command_surface::TabStripProjectionLease::new(
            crate::egui::text_command_surface::TabStripProjection::new(
                1,
                crate::egui::text_command_surface::TabStripCorrelation::from_opaque_bytes(b"corr"),
            ),
        ),
        catalog,
        config,
    )
    .expect("tab strip retained state should be constructible")
}

#[test]
fn group_drop_indicators_use_the_actual_target_edges_and_strip_end() {
    let target = egui::Rect::from_min_max(egui::pos2(100.0, 20.0), egui::pos2(180.0, 52.0));
    let strip = egui::Rect::from_min_max(egui::pos2(20.0, 0.0), egui::pos2(400.0, 72.0));
    let before = drop_indicator(TabStripDropIndicator::Before, target, strip, 101.0);
    let after = drop_indicator(TabStripDropIndicator::After, target, strip, 179.0);
    let end = drop_indicator(TabStripDropIndicator::EndOfStrip, target, strip, 360.0);

    assert_eq!(before.min, egui::pos2(99.0, 20.0));
    assert_eq!(before.max, egui::pos2(101.0, 52.0));
    assert_eq!(after.min, egui::pos2(179.0, 20.0));
    assert_eq!(after.max, egui::pos2(181.0, 52.0));
    assert_eq!(end.min, egui::pos2(359.0, 4.0));
    assert_eq!(end.max, egui::pos2(361.0, 68.0));
}

#[test]
fn paint_plan_contains_ghost_and_indicator_and_refreshes_label_texture() {
    let mut state = build_state();
    let context = egui::Context::default();
    let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 36.0));
    let indicator = drop_indicator(
        TabStripDropIndicator::Before,
        egui::Rect::from_min_size(egui::pos2(120.0, 0.0), egui::vec2(80.0, 36.0)),
        strip,
        120.0,
    );
    let mut first_operations = Vec::new();
    let mut first = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .paint_drag_feedback(
                ui,
                super::TabStripDragFeedback {
                    label: &super::TabStripText::new("first"),
                    pointer: egui::pos2(20.0, 4.0),
                    ghost_width: None,
                    indicator: Some(indicator),
                },
                strip,
                &mut first_operations,
            )
            .expect("first drag feedback should render");
    });
    first.textures_delta.clear();

    let first_identity = first_operations
        .iter()
        .find_map(|operation| match &operation.kind {
            super::TabStripPaintOperationKind::Texture { texture, .. } => {
                Some(texture.identity.clone())
            }
            super::TabStripPaintOperationKind::Fill { .. } => None,
        });
    assert!(first_identity.is_some());
    assert_eq!(
        first_operations
            .iter()
            .filter(|operation| matches!(
                operation.kind,
                super::TabStripPaintOperationKind::Fill { .. }
            ))
            .count(),
        2
    );

    let mut second_operations = Vec::new();
    let mut second = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .paint_drag_feedback(
                ui,
                super::TabStripDragFeedback {
                    label: &super::TabStripText::new("second"),
                    pointer: egui::pos2(20.0, 4.0),
                    ghost_width: None,
                    indicator: None,
                },
                strip,
                &mut second_operations,
            )
            .expect("second drag feedback should render");
    });
    second.textures_delta.clear();
    let second_identity = second_operations
        .iter()
        .find_map(|operation| match &operation.kind {
            super::TabStripPaintOperationKind::Texture { texture, .. } => {
                Some(texture.identity.clone())
            }
            super::TabStripPaintOperationKind::Fill { .. } => None,
        });
    assert_ne!(first_identity, second_identity);
    assert_eq!(
        second_operations
            .iter()
            .filter(|operation| matches!(
                operation.kind,
                super::TabStripPaintOperationKind::Fill { .. }
            ))
            .count(),
        1
    );
}
