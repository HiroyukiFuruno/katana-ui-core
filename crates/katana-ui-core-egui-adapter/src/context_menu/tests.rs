use super::{
    ContextMenuPaintStyle, ContextMenuPresentation, ContextMenuPresentationItem,
    ContextMenuRasterStyle, EguiContextMenuAdapter,
};
use crate::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use crate::text_surface::TextSurfaceContextTargetAnchor;
use katana_ui_core::molecule::selection::ContextMenuItemKind;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::theme::{FontFamily, FontToken};

const FRAME_WIDTH_PX: f32 = 640.0;
const FRAME_HEIGHT_PX: f32 = 360.0;
const CONTEXT_X: i32 = 620;
const CONTEXT_Y: i32 = 340;
const FONT_SIZE_PX: f32 = 15.0;
const FONT_WEIGHT: u16 = 400;
const LINE_HEIGHT_PX: f32 = 22.0;
const TEXT_RGBA: [u8; 4] = [230, 230, 230, 255];
const MENU_BACKGROUND_RGBA: [u8; 4] = [30, 30, 32, 255];
const MENU_HIGHLIGHTED_RGBA: [u8; 4] = [60, 80, 112, 255];
const MENU_DISABLED_RGBA: [u8; 4] = [45, 45, 48, 255];

#[test]
fn actual_context_menu_adapter_keeps_opaque_tree_and_composites_repeatably()
-> Result<(), Box<dyn std::error::Error>> {
    let first = run_menu()?;
    let second = run_menu()?;
    assert_eq!(first, second);
    assert!(first.0.width > 0);
    assert!(first.0.x < CONTEXT_X);
    assert!(first.1 > 0);
    Ok(())
}

fn run_menu() -> Result<(UiRect, usize, String), Box<dyn std::error::Error>> {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut adapter = EguiContextMenuAdapter::new(
        katana_ui_core_text_raster::PlatformTextRasterConfig::default(),
    )?;
    adapter.synchronize_presentation(ContextMenuPresentation {
        visible: true,
        items: vec![
            ContextMenuPresentationItem::action("format", "整形 ⭐️"),
            ContextMenuPresentationItem::action("code", "コード種別").child(
                ContextMenuPresentationItem::action("opaque-code-kind", "code kind"),
            ),
            ContextMenuPresentationItem {
                id: "disabled".to_string(),
                label: "利用不可".to_string(),
                accessibility_label: "利用不可".to_string(),
                icon: None,
                enabled: false,
                checked: false,
                kind: ContextMenuItemKind::Action,
                children: Vec::new(),
            },
        ],
    });
    adapter.request_open(TextSurfaceContextTargetAnchor::pointer(
        CONTEXT_X,
        CONTEXT_Y,
        UiTextSelectionRange::caret(0),
        UiRect::new(0, 0, FRAME_WIDTH_PX as u32, FRAME_HEIGHT_PX as u32),
    ));
    let mut output = None;
    let _ = context.run_ui(frame_input(), |ui| {
        output = Some(adapter.show(ui, &raster_style(), &paint_style()));
    });
    let output = output.ok_or_else(|| std::io::Error::other("actual egui frame did not run"))??;
    let record = output
        .record
        .ok_or_else(|| std::io::Error::other("visible menu record was absent"))?;
    let artifact = output
        .artifact
        .ok_or_else(|| std::io::Error::other("visible menu artifact was absent"))?;
    let plans = [ArtifactPaintPlanRef::ContextMenu(&artifact.paint_plan)];
    let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(UiRect::new(
            0,
            0,
            FRAME_WIDTH_PX as u32,
            FRAME_HEIGHT_PX as u32,
        )),
        plans: &plans,
    })?;
    Ok((
        record.bounds,
        composite.non_transparent_pixel_count,
        composite.pixel_hash,
    ))
}

fn frame_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(FRAME_WIDTH_PX, FRAME_HEIGHT_PX),
        )),
        ..egui::RawInput::default()
    }
}

fn raster_style() -> ContextMenuRasterStyle {
    ContextMenuRasterStyle {
        font: FontToken {
            name: "context-menu-test".to_string(),
            family: FontFamily::Proportional,
            size: FONT_SIZE_PX,
            weight: FONT_WEIGHT,
        },
        text_color_rgba: TEXT_RGBA,
        icon_color_rgba: TEXT_RGBA,
        line_height_px: LINE_HEIGHT_PX,
    }
}

const fn paint_style() -> ContextMenuPaintStyle {
    ContextMenuPaintStyle {
        background_rgba: MENU_BACKGROUND_RGBA,
        highlighted_rgba: MENU_HIGHLIGHTED_RGBA,
        disabled_rgba: MENU_DISABLED_RGBA,
    }
}
