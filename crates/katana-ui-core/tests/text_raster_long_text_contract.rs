#![cfg(feature = "text-raster")]

use katana_ui_core::render_model::UiTextSpan;
use katana_ui_core::text_raster::{
    PlatformTextGraphemeRange, PlatformTextRasterError, PlatformTextRasterRequest,
    PlatformTextRasterizer,
};
use katana_ui_core::theme::{FontFamily, FontToken};

const TEXT_COLOR: [u8; 4] = [245, 245, 245, 255];

fn font(size: f32) -> FontToken {
    FontToken {
        name: "text-raster-contract".to_owned(),
        family: FontFamily::Monospace,
        size,
        weight: 400,
    }
}

fn request(text: &str, size: f32, line_height: f32) -> PlatformTextRasterRequest {
    let mut request = PlatformTextRasterRequest {
        spans: vec![UiTextSpan::plain(text)],
        font: font(size),
        fallback_color_rgba: TEXT_COLOR,
        line_height_px: line_height,
        max_width_px: None,
        scale_factor: 1.0,
    };
    request.max_width_px = Some(8192.0);
    request
}

#[test]
fn long_text_keeps_last_glyph_bounds_and_opaque_crop_beyond_legacy_height() {
    let text = (1..=240)
        .map(|line| format!("KUC raster row {line:03}"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut rasterizer = PlatformTextRasterizer::new(Default::default());
    let raster = rasterizer
        .rasterize(&request(&text, 16.0, 24.0))
        .expect("240 lines must rasterize beyond the legacy 4096px buffer height");

    assert!(
        raster.height > 4096,
        "raster height must include all 240 rows"
    );
    let ranges = PlatformTextGraphemeRange::ranges(&text);
    let last_range = ranges.last().expect("last grapheme range");
    let last = raster
        .grapheme_bounds
        .iter()
        .find(|bounds| {
            bounds.byte_start == last_range.byte_start && bounds.byte_end == last_range.byte_end
        })
        .expect("final grapheme must remain in grapheme_bounds");
    assert!(last.y + last.height <= raster.height as f32);
    let crop = raster
        .grapheme_crop(last, 1.0)
        .expect("final grapheme must have a valid crop");
    assert!(
        crop.pixels.iter().any(|pixel| pixel[3] > 0),
        "final glyph crop must contain rendered alpha"
    );
}

#[test]
fn short_control_text_exposes_complete_grapheme_metrics() {
    let text = "short\ncontrol";
    let mut rasterizer = PlatformTextRasterizer::new(Default::default());
    let raster = rasterizer
        .rasterize(&request(text, 16.0, 24.0))
        .expect("short control text must rasterize");
    for range in PlatformTextGraphemeRange::ranges(text)
        .into_iter()
        .filter(|range| &text[range.byte_start..range.byte_end] != "\n")
    {
        assert!(raster.grapheme_bounds.iter().any(
            |bounds| bounds.byte_start == range.byte_start && bounds.byte_end == range.byte_end
        ));
    }
}

#[test]
fn raster_rejects_dimension_and_pixel_limit_overflow() {
    let mut rasterizer = PlatformTextRasterizer::new(Default::default());
    let dimension_error = rasterizer.rasterize(&request("x", 9000.0, 9000.0));
    assert!(matches!(
        dimension_error,
        Err(PlatformTextRasterError::RasterTooLarge { .. })
    ));

    let pixel_error = rasterizer.rasterize(&request("x", 8192.0, 8192.0));
    assert!(matches!(
        pixel_error,
        Err(PlatformTextRasterError::RasterTooLarge {
            max_pixels: 16_777_216,
            ..
        })
    ));
}
