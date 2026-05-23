use super::*;

const TRACK: LayoutRect = LayoutRect::new(10, 20, 8, 800);
const HORIZONTAL_TRACK: LayoutRect = LayoutRect::new(10, 20, 800, 8);
const VIEWPORT: usize = 800;
const SMALL_CONTENT: usize = 900;
const LARGE_CONTENT: usize = 3_200;
const MIN_THUMB: usize = 32;

#[test]
fn visible_scrollbar_thumb_length_uses_viewport_content_ratio_without_minimum_clamp() {
    let model = ScrollbarModel::vertical(TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);
    let expected = TRACK.height * VIEWPORT / SMALL_CONTENT;

    assert_eq!(expected, model.thumb_rect(0).height);
    assert!(model.thumb_rect(0).height > TRACK.height * 7 / 8);
}

#[test]
fn visible_scrollbar_thumb_uses_full_track_when_content_fits_viewport() {
    let model = ScrollbarModel::vertical(TRACK, VIEWPORT, VIEWPORT, MIN_THUMB);

    assert_eq!(TRACK.height, model.thumb_rect(0).height);
    assert_eq!(0, model.max_offset());
    assert_eq!(TRACK.y, model.thumb_y(0));
}

#[test]
fn visible_scrollbar_small_overflow_has_longer_thumb_and_smaller_travel() {
    let small = ScrollbarModel::vertical(TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);
    let large = ScrollbarModel::vertical(TRACK, VIEWPORT, LARGE_CONTENT, MIN_THUMB);

    assert!(small.thumb_rect(0).height > large.thumb_rect(0).height);
    assert!(
        TRACK.height.saturating_sub(small.thumb_rect(0).height)
            < TRACK.height.saturating_sub(large.thumb_rect(0).height)
    );
}

#[test]
fn visible_vertical_scrollbar_thumb_starts_and_ends_at_track_edges() {
    let model = ScrollbarModel::vertical(TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);
    let top = model.thumb_rect(0);
    let bottom = model.thumb_rect(model.max_offset());

    assert_eq!(TRACK.y, top.y);
    assert_eq!(TRACK.bottom(), bottom.bottom());
}

#[test]
fn visible_horizontal_scrollbar_thumb_starts_and_ends_at_track_edges() {
    let model = ScrollbarModel::horizontal(HORIZONTAL_TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);
    let start = model.horizontal_thumb_rect(0);
    let end = model.horizontal_thumb_rect(model.max_offset());

    assert_eq!(HORIZONTAL_TRACK.x, start.x);
    assert_eq!(HORIZONTAL_TRACK.right(), end.right());
}

#[test]
fn visible_scrollbar_drag_reverse_maps_track_travel_start_and_end_to_offsets() {
    let vertical = ScrollbarModel::vertical(TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);
    let horizontal =
        ScrollbarModel::horizontal(HORIZONTAL_TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);

    assert_eq!(0, vertical.offset_from_thumb_y(TRACK.y));
    assert_eq!(
        vertical.max_offset(),
        vertical.offset_from_thumb_y(vertical.thumb_y(vertical.max_offset()))
    );
    assert_eq!(0, horizontal.offset_from_thumb_x(HORIZONTAL_TRACK.x));
    assert_eq!(
        horizontal.max_offset(),
        horizontal.offset_from_thumb_x(horizontal.thumb_x(horizontal.max_offset()))
    );
}

#[test]
fn visible_scrollbar_max_offset_constructor_uses_viewport_plus_offset_content_length() {
    let from_content = ScrollbarModel::vertical(TRACK, VIEWPORT, SMALL_CONTENT, MIN_THUMB);
    let from_offset = ScrollbarModel::vertical_from_max_offset(
        TRACK,
        VIEWPORT,
        SMALL_CONTENT - VIEWPORT,
        MIN_THUMB,
    );

    assert_eq!(from_content.max_offset(), from_offset.max_offset());
    assert_eq!(
        from_content.thumb_rect(0).height,
        from_offset.thumb_rect(0).height
    );
}

#[test]
fn visible_scrollbar_minimum_thumb_length_only_clamps_large_overflow() {
    let content = 80_000;
    let model = ScrollbarModel::vertical(TRACK, VIEWPORT, content, MIN_THUMB);

    assert_eq!(MIN_THUMB, model.thumb_rect(0).height);
}

#[test]
fn visible_scrollbar_small_max_offset_pairs_long_thumb_with_short_travel() {
    let max_offset = 70;
    let model = ScrollbarModel::vertical_from_max_offset(TRACK, VIEWPORT, max_offset, MIN_THUMB);
    let thumb = model.thumb_rect(0);
    let end = model.thumb_rect(max_offset);

    assert!(thumb.height > TRACK.height * 9 / 10);
    assert!(end.y.saturating_sub(thumb.y) < TRACK.height / 10);
    assert_eq!(TRACK.bottom(), end.bottom());
}
