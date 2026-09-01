use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_surface::{
    TextSurfaceAnnotationPaint, TextSurfaceGutterPaint, TextSurfacePaintStyle,
    TextSurfaceRasterStyle,
};
use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAnnotation, TextSurfaceAnnotationStyle,
    TextSurfaceAutomaticGutterPresentation, TextSurfaceGutter, TextSurfaceGutterRow,
    TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use katana_ui_core::theme::{FontFamily, FontToken};

pub(super) const SURFACE_WIDTH: f32 = 960.0;
pub(super) const SURFACE_HEIGHT: f32 = 288.0;
pub(super) const GUTTER_WIDTH: u32 = 40;
const FONT_SIZE: f32 = 16.0;
const LINE_HEIGHT: f32 = 24.0;
const FONT_WEIGHT: u16 = 400;
pub(super) const STORY_LINE_COUNT: usize = 13;
const FIRST_DISPLAY_LINE: usize = 1;
const ANNOTATION_START: usize = 8;
const ANNOTATION_END: usize = 11;
const TEXT_COLOR: [u8; 4] = [235, 235, 235, 255];
const BACKGROUND_COLOR: [u8; 4] = [24, 24, 24, 255];
const GUTTER_COLOR: [u8; 4] = [32, 32, 32, 255];
const SELECTION_COLOR: [u8; 4] = [64, 96, 160, 180];
const PREEDIT_COLOR: [u8; 4] = [255, 196, 64, 255];
const CARET_COLOR: [u8; 4] = [255, 255, 255, 255];
const ACTIVE_GUTTER_BACKGROUND: [u8; 4] = [48, 64, 88, 255];
const MARKED_GUTTER_BACKGROUND: [u8; 4] = [72, 56, 40, 255];
const ANNOTATION_COLOR: [u8; 4] = [255, 196, 64, 255];

pub(super) fn text_surface_fixture() -> TextSurface {
    let text = "一行目: 日本語 ⭐️\n二行目: IME と selection\n三行目: gutter と annotation\n四行目: scroll target\n五行目: platform text raster\n六行目: KUC shared surface\n七行目: copied state\n八行目: history request\n九行目: context target\n十行目: accessibility text run\n十一行目: deterministic artifact\n十二行目: no fallback glyph\n十三行目: scrollable content";
    TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("storybook.text-surface")
                .stable_state_id("storybook.text-surface")
                .value(text),
            UiTextSpan::emoji_marked_spans(text, Default::default()),
            TextSurfaceViewport::new(0, 0, SURFACE_WIDTH as u32, SURFACE_HEIGHT as u32),
        )
        .accessibility_label("TextSurface story")
        .annotation(TextSurfaceAnnotation::new(
            "storybook-annotation",
            katana_ui_core::text_selection::UiTextSelectionRange::new(
                ANNOTATION_START,
                ANNOTATION_END,
            ),
            "storybook-annotation",
            TextSurfaceAnnotationStyle::Underline,
        ))
        .gutter(storybook_gutter()),
    )
}

pub(super) fn text_presentation() -> TextSurfacePresentation {
    let mut presentation = TextSurfacePresentation::from_props(text_surface_fixture().props());
    presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    presentation
}

pub(super) fn raster_style() -> TextSurfaceRasterStyle {
    TextSurfaceRasterStyle::new(
        FontToken {
            name: "storybook-text-surface".to_string(),
            family: FontFamily::Monospace,
            size: FONT_SIZE,
            weight: FONT_WEIGHT,
        },
        TEXT_COLOR,
        LINE_HEIGHT,
    )
}

pub(super) fn paint_style() -> TextSurfacePaintStyle {
    TextSurfacePaintStyle {
        background_rgba: BACKGROUND_COLOR,
        gutter_background_rgba: GUTTER_COLOR,
        gutter_paints: vec![
            TextSurfaceGutterPaint::new("active", TEXT_COLOR).background(ACTIVE_GUTTER_BACKGROUND),
            TextSurfaceGutterPaint::new("marked", TEXT_COLOR).background(MARKED_GUTTER_BACKGROUND),
        ],
        selection_rgba: SELECTION_COLOR,
        preedit_rgba: PREEDIT_COLOR,
        caret_rgba: CARET_COLOR,
        annotation_paints: vec![TextSurfaceAnnotationPaint::new(
            "storybook-annotation",
            ANNOTATION_COLOR,
        )],
    }
}

pub(super) const fn script_line_height() -> f32 {
    LINE_HEIGHT
}

fn storybook_gutter() -> TextSurfaceGutter {
    let mut gutter = TextSurfaceGutter::new(GUTTER_WIDTH);
    for logical_row in 0..STORY_LINE_COUNT {
        let label = logical_row.saturating_add(FIRST_DISPLAY_LINE).to_string();
        let row = match logical_row {
            0 => TextSurfaceGutterRow::new(logical_row, label).visual_role("active"),
            1 => TextSurfaceGutterRow::new(logical_row, label).visual_role("default"),
            2 => TextSurfaceGutterRow::new(logical_row, label).visual_role("marked"),
            _ => TextSurfaceGutterRow::new(logical_row, label),
        };
        gutter.rows.push(row);
    }
    gutter
}
