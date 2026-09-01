use crate::text_raster::{
    PlatformTextGraphemeRange, PlatformTextRasterConfig, PlatformTextRasterError,
    PlatformTextRasterRequest, PlatformTextRasterizer,
};
use crate::text_selection::UiTextSelectionRange;
use crate::text_surface::TextSurfacePoint;
use crate::theme::{FontFamily, FontToken};

const RGBA_CHANNEL_COUNT: usize = 4;
const TEXT_COLOR: [u8; RGBA_CHANNEL_COUNT] = [245, 245, 245, 255];
const EDITOR_FONT_SIZE_PX: f32 = 18.0;
const EDITOR_FONT_WEIGHT: u16 = 400;
const SURFACE_ORIGIN_X: i32 = 24;
const SURFACE_ORIGIN_Y: i32 = 48;
const CARET_WIDTH_PX: u32 = 1;

#[test]
fn shared_text_surface_layout_keeps_unicode_ranges_hit_testing_and_caret_on_one_layout()
-> Result<(), Box<dyn std::error::Error>> {
    let text = "A⭐️🧑‍💻e\u{301}日本";
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = match rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        text,
        editor_font(),
        TEXT_COLOR,
    )) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            assert!(!face.is_available());
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let layout = raster.text_surface_layout(
        "platform-text-raster:unicode",
        TextSurfacePoint::new(SURFACE_ORIGIN_X, SURFACE_ORIGIN_Y),
    );
    let ranges = PlatformTextGraphemeRange::ranges(text);

    assert_eq!(text, raster.text);
    assert_eq!(ranges.len(), layout.graphemes.len());
    assert_eq!(
        ranges.len(),
        layout.visible_graphemes(layout.content_bounds).len()
    );

    for (index, range) in ranges.iter().enumerate() {
        let selection = layout.grapheme_range_for_byte_offsets(range.byte_start, range.byte_end);
        let bounds = layout.graphemes[index].bounds;
        let hit = layout.hit_test(bounds.x, bounds.y.saturating_add(1));
        let caret = layout.caret_rect(UiTextSelectionRange::caret(index));

        assert_eq!(
            UiTextSelectionRange::new(index, index.saturating_add(1)),
            selection
        );
        assert_eq!(UiTextSelectionRange::caret(index), hit);
        assert_eq!(bounds.x, caret.x);
        assert_eq!(bounds.y, caret.y);
        assert_eq!(CARET_WIDTH_PX, caret.width);
        assert_eq!(bounds.height, caret.height);
        assert_eq!(vec![bounds], layout.selection_rects(selection));
    }
    Ok(())
}

#[test]
fn composed_raster_layout_records_preedit_range_and_caret_without_separate_measurement()
-> Result<(), Box<dyn std::error::Error>> {
    let base_text = "A日本";
    let preedit = "⭐️";
    let composed_text = "A⭐️日本";
    let mut rasterizer = PlatformTextRasterizer::new(PlatformTextRasterConfig::default());
    let raster = match rasterizer.rasterize(&PlatformTextRasterRequest::from_text(
        composed_text,
        editor_font(),
        TEXT_COLOR,
    )) {
        Ok(raster) => raster,
        Err(PlatformTextRasterError::ColorEmojiUnavailable { face }) => {
            assert!(!face.is_available());
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let layout = raster.text_surface_layout_with_composition(
        "platform-text-raster:composition",
        TextSurfacePoint::new(SURFACE_ORIGIN_X, SURFACE_ORIGIN_Y),
        "A".len(),
        "A".len(),
        preedit,
        preedit.len(),
    );

    let composition = layout.composition_model();

    assert_eq!("A日本", base_text);
    assert_eq!(composed_text, raster.text);
    assert!(matches!(
        composition,
        Some(value)
            if value.preedit == preedit
                && value.source_start == "A".len()
                && value.source_end == "A".len()
                && value.preedit_range == UiTextSelectionRange::new(1, 2)
                && value.caret_range == UiTextSelectionRange::caret(2)
    ));
    Ok(())
}

fn editor_font() -> FontToken {
    FontToken {
        name: "editor".to_string(),
        family: FontFamily::Monospace,
        size: EDITOR_FONT_SIZE_PX,
        weight: EDITOR_FONT_WEIGHT,
    }
}
