#![cfg(feature = "egui")]
use katana_ui_core::egui::context_menu::{
    ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuPresentationItem,
    ContextMenuRasterStyle, EguiContextMenuAdapter,
};
use katana_ui_core::egui::text_surface::TextSurfaceContextTargetAnchor;
use katana_ui_core::theme::{FontFamily, FontToken};
use serde_json::json;

#[test]
fn serialized_node_anchor_renders_inside_its_retained_viewport() -> Result<(), String> {
    let anchor: TextSurfaceContextTargetAnchor = serde_json::from_value(json!({
        "anchor": { "NodeId": "opaque-node" },
        "selection": { "anchor": 0, "focus": 0 },
        "viewport_bounds": { "x": 10, "y": 20, "width": 320, "height": 180 }
    }))
    .map_err(|error| error.to_string())?;
    let viewport = anchor.viewport_bounds();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core::text_raster::PlatformTextRasterConfig::default(),
    )
    .map_err(|error| error.to_string())?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![ContextMenuPresentationItem::action(
            "opaque-action",
            "Action",
        )],
    });
    adapter.request_open(anchor);

    let context = egui::Context::default();
    let mut result = None;
    let mut full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(360.0, 220.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            result = Some(adapter.show(ui, &raster_style(), &paint_style()));
        },
    );
    full.textures_delta.clear();
    let record = result
        .ok_or_else(|| "context menu frame did not run".to_owned())?
        .map_err(|error| error.to_string())?
        .record
        .ok_or_else(|| "node anchor did not open the context menu".to_owned())?;

    assert_eq!(record.viewport_bounds, viewport);
    assert!(record.bounds.x >= viewport.x);
    assert!(record.bounds.y >= viewport.y);
    assert!(
        record.bounds.x.saturating_add_unsigned(record.bounds.width)
            <= viewport.x.saturating_add_unsigned(viewport.width)
    );
    assert!(
        record
            .bounds
            .y
            .saturating_add_unsigned(record.bounds.height)
            <= viewport.y.saturating_add_unsigned(viewport.height)
    );

    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: Vec::new(),
    });
    let mut empty_result = None;
    let mut empty_full = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(360.0, 220.0),
            )),
            events: vec![
                key(egui::Key::ArrowUp),
                key(egui::Key::End),
                key(egui::Key::Space),
                key(egui::Key::Tab),
            ],
            ..egui::RawInput::default()
        },
        |ui| empty_result = Some(adapter.show(ui, &raster_style(), &paint_style())),
    );
    empty_full.textures_delta.clear();
    let empty = empty_result
        .ok_or_else(|| "empty context menu frame did not run".to_owned())?
        .map_err(|error| error.to_string())?;
    assert!(empty.events.is_empty());
    Ok(())
}

fn key(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn raster_style() -> ContextMenuRasterStyle {
    ContextMenuRasterStyle {
        font: FontToken {
            name: "system-ui".to_owned(),
            family: FontFamily::Proportional,
            size: 14.0,
            weight: 400,
        },
        text_color_rgba: [230, 230, 230, 255],
        icon_color_rgba: [230, 230, 230, 255],
        line_height_px: 20.0,
    }
}

const fn paint_style() -> ContextMenuPaintStyle {
    ContextMenuPaintStyle {
        background_rgba: [30, 30, 32, 255],
        highlighted_rgba: [60, 80, 112, 255],
        disabled_rgba: [45, 45, 48, 255],
    }
}
