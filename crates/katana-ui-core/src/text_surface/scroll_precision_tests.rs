use super::{
    TextSurface, TextSurfaceAutomaticGutterPresentation, TextSurfaceGraphemeBox, TextSurfaceLayout,
    TextSurfaceLogicalPixels, TextSurfacePresentation, TextSurfaceProps,
    TextSurfaceScrollAlignment, TextSurfaceScrollRequest, TextSurfaceScrollRequestAcknowledgement,
    TextSurfaceScrollRequestRejection, TextSurfaceScrollRequestResult,
    TextSurfaceScrollRequestToken, TextSurfaceScrollTarget, TextSurfaceViewport,
};
use crate::atom::TextArea;
use crate::render_model::UiRect;

const GRAPHEME_WIDTH: u32 = 12;
const LINE_HEIGHT: u32 = 20;
const LINE_ADVANCE: i32 = LINE_HEIGHT as i32;
const CONTENT_COLUMNS: u32 = 5;
const CONTENT_ROWS: u32 = 3;

#[test]
fn logical_pixel_json_round_trips_finite_and_nonfinite_bit_patterns()
-> Result<(), Box<dyn std::error::Error>> {
    let request = TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("nonfinite-wire"),
        TextSurfaceScrollTarget::relative_pixels(
            TextSurfaceLogicalPixels::new(f32::NAN),
            TextSurfaceLogicalPixels::new(f32::INFINITY),
        ),
        TextSurfaceScrollAlignment::Nearest,
    );

    let json = serde_json::to_string(&request)?;
    assert!(!json.contains("NaN"));
    assert!(!json.contains("Infinity"));
    let decoded: TextSurfaceScrollRequest = serde_json::from_str(&json)?;
    assert_eq!(request, decoded);
    let TextSurfaceScrollTarget::RelativePixels { delta_x, delta_y } = decoded.target else {
        return Err(std::io::Error::other(
            "round-tripped request must retain the relative-pixel target",
        )
        .into());
    };
    assert_eq!(f32::NAN.to_bits(), delta_x.bit_pattern());
    assert_eq!(f32::INFINITY.to_bits(), delta_y.bit_pattern());
    assert_eq!(
        TextSurfaceLogicalPixels::new(0.0),
        TextSurfaceLogicalPixels::new(-0.0),
    );
    Ok(())
}

#[test]
fn relative_pixel_precision_nonfinite_rejection_and_visible_rows_are_kuc_facts() {
    let text = "日本語 ⭐️\n二行目\n三行目";
    let layout = precision_layout(text);
    let viewport = UiRect::new(0, LINE_ADVANCE / 2, GRAPHEME_WIDTH * 2, LINE_HEIGHT);
    let mut surface = precision_surface(text);
    surface.synchronize_scroll_bounds(layout.content_bounds, viewport);

    let empty_surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("empty-visible-rows"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
    ));
    assert!(
        empty_surface
            .frame(&TextSurfaceLayout::new(
                "empty-visible-rows",
                UiRect::new(0, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
            ))
            .visible_logical_rows
            .is_empty()
    );
    assert_eq!(
        vec![0, 1],
        surface
            .frame_with_bounds(
                &layout,
                UiRect::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT),
                viewport,
            )
            .visible_logical_rows
    );

    let mut gutter_presentation = TextSurfacePresentation::from_props(surface.props());
    gutter_presentation.automatic_gutter = Some(TextSurfaceAutomaticGutterPresentation::new());
    assert!(surface.synchronize_presentation(gutter_presentation));
    assert_eq!(
        vec![0, 1],
        surface
            .frame_with_bounds(
                &layout,
                UiRect::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT),
                viewport,
            )
            .visible_logical_rows
    );

    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("fractional"),
        TextSurfaceScrollTarget::relative_pixels(0.0, 20.6),
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(presentation));
    assert_eq!(
        Some(TextSurfaceScrollRequestResult::Acknowledged(
            TextSurfaceScrollRequestAcknowledgement {
                token: TextSurfaceScrollRequestToken::new("fractional"),
                target_bounds: None,
                scroll_x: 0,
                scroll_y: 21,
            }
        )),
        surface.resolve_controlled_scroll_request_with_scale(&layout, viewport, 2.0)
    );

    let before = (surface.state().scroll_x, surface.state().scroll_y);
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("nonfinite"),
        TextSurfaceScrollTarget::relative_pixels(TextSurfaceLogicalPixels::new(f32::NAN), 0.0),
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(presentation));
    assert_eq!(
        Some(TextSurfaceScrollRequestResult::Rejected {
            token: TextSurfaceScrollRequestToken::new("nonfinite"),
            reason: TextSurfaceScrollRequestRejection::NonFiniteRelativePixels,
        }),
        surface.resolve_controlled_scroll_request(&layout, viewport)
    );
    assert_eq!(before, (surface.state().scroll_x, surface.state().scroll_y));
}

fn precision_layout(text: &str) -> TextSurfaceLayout {
    TextSurfaceLayout::from_grapheme_boxes(
        "scroll-precision",
        UiRect::new(
            0,
            0,
            GRAPHEME_WIDTH * CONTENT_COLUMNS,
            LINE_HEIGHT * CONTENT_ROWS,
        ),
        text,
        vec![
            TextSurfaceGraphemeBox {
                grapheme_index: 0,
                byte_start: 0,
                byte_end: "日".len(),
                bounds: UiRect::new(0, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 1,
                byte_start: "日本語 ⭐️\n".len(),
                byte_end: "日本語 ⭐️\n二".len(),
                bounds: UiRect::new(0, LINE_ADVANCE, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: "日本語 ⭐️\n二行目\n".len(),
                byte_end: text.len(),
                bounds: UiRect::new(0, LINE_ADVANCE * 2, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
        ],
    )
}

fn precision_surface(text: &str) -> TextSurface {
    TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll-precision").value(text),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT),
    ))
}
