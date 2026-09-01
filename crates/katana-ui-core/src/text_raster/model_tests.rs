use super::*;
use crate::theme::{FontFamily, FontToken};

const TEST_FONT_SIZE_PX: f32 = 16.0;
const TEST_FONT_WEIGHT: u16 = 400;

fn font() -> FontToken {
    FontToken {
        name: "coverage".to_string(),
        family: FontFamily::Monospace,
        size: TEST_FONT_SIZE_PX,
        weight: TEST_FONT_WEIGHT,
    }
}

#[test]
fn invalid_metric_scales_normalize_and_frame_begin_fails_closed() {
    for scale in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let request = PlatformTextMetricsRequest::from_text("metrics", font(), scale);
        assert_eq!(request.normalized_scale_factor(), 1.0);

        let mut frame = PlatformTextMetricsFrame::new();
        assert_eq!(
            frame.begin(scale),
            Err(PlatformTextRasterError::NonFiniteLayoutExtent)
        );
    }
}

#[test]
fn invalid_raster_scale_and_line_height_use_stable_defaults() {
    let mut request = PlatformTextRasterRequest::from_text("text", font(), [0; 4]);
    for scale in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        request.scale_factor = scale;
        assert_eq!(request.normalized_scale_factor(), 1.0);
    }
    for line_height in [0.0, -1.0, f32::NAN] {
        request.line_height_px = line_height;
        assert_eq!(
            request.normalized_line_height(),
            request.font.size * DEFAULT_LINE_HEIGHT_RATIO
        );
    }
}
