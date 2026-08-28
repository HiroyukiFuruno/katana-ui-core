use super::{
    TextSurface, TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfacePresentation,
    TextSurfaceProps, TextSurfaceScrollAlignment, TextSurfaceScrollRequest,
    TextSurfaceScrollRequestAcknowledgement, TextSurfaceScrollRequestRejection,
    TextSurfaceScrollRequestResult, TextSurfaceScrollRequestToken, TextSurfaceScrollTarget,
    TextSurfaceViewport,
};
use crate::atom::{TextArea, TextAreaAction, TextAreaCompositionPhase};
use crate::render_model::UiRect;

const GRAPHEME_WIDTH: u32 = 12;
const GRAPHEME_ADVANCE: i32 = GRAPHEME_WIDTH as i32;
const LINE_HEIGHT: u32 = 20;
const LINE_ADVANCE: i32 = LINE_HEIGHT as i32;

#[test]
fn controlled_scroll_rejections_are_typed_and_leave_scroll_offsets_unchanged() {
    let text = "日本⭐️A";
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "scroll-request",
        UiRect::new(0, 0, GRAPHEME_WIDTH * 4, LINE_HEIGHT),
        text,
        vec![
            grapheme_box(0, 0, "日".len(), 0),
            grapheme_box(1, "日".len(), "日本".len(), GRAPHEME_ADVANCE),
            grapheme_box(2, "日本".len(), "日本⭐️".len(), GRAPHEME_ADVANCE * 2),
            grapheme_box(3, "日本⭐️".len(), text.len(), GRAPHEME_ADVANCE * 3),
        ],
    );
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll-request").value(text),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT).scroll_offset(0, 7),
    ));
    surface.synchronize_scroll_bounds(
        UiRect::new(0, 0, GRAPHEME_WIDTH * 4, LINE_HEIGHT),
        UiRect::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT),
    );
    let before = (surface.state().scroll_x, surface.state().scroll_y);
    for (token, target, reason) in [
        (
            "invalid-boundary",
            TextSurfaceScrollTarget::ByteOffset {
                byte_offset: "日本".len().saturating_add(1),
            },
            TextSurfaceScrollRequestRejection::InvalidUtf8Boundary,
        ),
        (
            "invalid-range",
            TextSurfaceScrollTarget::ByteRange {
                byte_start: "日本⭐️".len(),
                byte_end: "日本".len(),
            },
            TextSurfaceScrollRequestRejection::InvalidByteRange,
        ),
        (
            "missing-row",
            TextSurfaceScrollTarget::LogicalRow { logical_row: 1 },
            TextSurfaceScrollRequestRejection::LogicalRowNotFound,
        ),
    ] {
        let mut presentation = TextSurfacePresentation::from_props(surface.props());
        presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
            TextSurfaceScrollRequestToken::new(token),
            target,
            TextSurfaceScrollAlignment::Nearest,
        ));
        assert!(surface.synchronize_presentation(presentation));
        assert_eq!(
            Some(TextSurfaceScrollRequestResult::Rejected {
                token: TextSurfaceScrollRequestToken::new(token),
                reason,
            }),
            surface.resolve_controlled_scroll_request(
                &layout,
                UiRect::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT),
            )
        );
        assert_eq!(before, (surface.state().scroll_x, surface.state().scroll_y));
    }
}

fn grapheme_box(
    grapheme_index: usize,
    byte_start: usize,
    byte_end: usize,
    x: i32,
) -> super::TextSurfaceGraphemeBox {
    super::TextSurfaceGraphemeBox {
        grapheme_index,
        byte_start,
        byte_end,
        bounds: UiRect::new(x, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
    }
}

#[test]
fn controlled_scroll_request_defers_until_bounds_then_resolves_the_same_token() {
    let text = "日本語 ⭐️\n次行";
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "scroll-request-defer",
        UiRect::new(0, 0, GRAPHEME_WIDTH * 4, LINE_HEIGHT * 2),
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
                byte_start: "日".len(),
                byte_end: "日本".len(),
                bounds: UiRect::new(GRAPHEME_ADVANCE, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 2,
                byte_start: "日本".len(),
                byte_end: "日本語".len(),
                bounds: UiRect::new(GRAPHEME_ADVANCE * 2, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 3,
                byte_start: "日本語".len(),
                byte_end: "日本語 ".len(),
                bounds: UiRect::new(GRAPHEME_ADVANCE * 3, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 4,
                byte_start: "日本語 ".len(),
                byte_end: "日本語 ⭐️".len(),
                bounds: UiRect::new(GRAPHEME_ADVANCE * 4, 0, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 5,
                byte_start: "日本語 ⭐️\n".len(),
                byte_end: "日本語 ⭐️\n次".len(),
                bounds: UiRect::new(0, LINE_ADVANCE, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
            TextSurfaceGraphemeBox {
                grapheme_index: 6,
                byte_start: "日本語 ⭐️\n次".len(),
                byte_end: text.len(),
                bounds: UiRect::new(GRAPHEME_ADVANCE, LINE_ADVANCE, GRAPHEME_WIDTH, LINE_HEIGHT),
            },
        ],
    );
    let viewport = UiRect::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT);
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll-request-defer").value(text),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, GRAPHEME_WIDTH * 2, LINE_HEIGHT),
    ));
    let _ = surface.apply_action(super::TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(super::TextSurfaceAction::TextArea(
        TextAreaAction::composition(TextAreaCompositionPhase::Update, "入力中 ⭐️", 3),
    ));
    let request = TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("defer-then-resolve"),
        TextSurfaceScrollTarget::LogicalRow { logical_row: 1 },
        TextSurfaceScrollAlignment::Start,
    );
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(request.clone());
    assert!(surface.synchronize_presentation(presentation));

    let before_defer = surface.state().clone();
    assert_eq!(
        None,
        surface.resolve_controlled_scroll_request(&layout, viewport)
    );
    assert_eq!(before_defer, surface.state().clone());

    surface.synchronize_scroll_bounds(
        UiRect::new(0, 0, GRAPHEME_WIDTH * 5, LINE_HEIGHT * 2),
        viewport,
    );
    assert_eq!(
        Some(TextSurfaceScrollRequestResult::Acknowledged(
            TextSurfaceScrollRequestAcknowledgement {
                token: request.token.clone(),
                target_bounds: Some(UiRect::new(
                    0,
                    LINE_ADVANCE,
                    GRAPHEME_WIDTH * 2,
                    LINE_HEIGHT,
                )),
                scroll_x: 0,
                scroll_y: LINE_ADVANCE,
            }
        )),
        surface.resolve_controlled_scroll_request(&layout, viewport)
    );

    let before_invalid = (surface.state().scroll_x, surface.state().scroll_y);
    let mut presentation = TextSurfacePresentation::from_props(surface.props());
    presentation.scroll_request = Some(TextSurfaceScrollRequest::new(
        TextSurfaceScrollRequestToken::new("invalid-utf8-boundary"),
        TextSurfaceScrollTarget::ByteOffset {
            byte_offset: "日".len() + 1,
        },
        TextSurfaceScrollAlignment::Nearest,
    ));
    assert!(surface.synchronize_presentation(presentation));
    assert_eq!(
        Some(TextSurfaceScrollRequestResult::Rejected {
            token: TextSurfaceScrollRequestToken::new("invalid-utf8-boundary"),
            reason: TextSurfaceScrollRequestRejection::InvalidUtf8Boundary,
        }),
        surface.resolve_controlled_scroll_request(&layout, viewport)
    );
    assert_eq!(
        before_invalid,
        (surface.state().scroll_x, surface.state().scroll_y)
    );
}
