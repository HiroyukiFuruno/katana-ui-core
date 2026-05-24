use super::scrollbar_model::ScrollbarModel;
use super::{panel_layout, panel_scroll_state, panel_scrollbars};

#[test]
fn horizontal_bar_hidden_when_outer_preview_content_fits_viewport() {
    let region = panel_scroll_state::PanelScrollRegion::Preview;
    let track = panel_scrollbars::horizontal_track_rect_for(region);
    let overflow = panel_scroll_state::overflow_for(region, "button", Default::default());
    let model = ScrollbarModel::horizontal(
        track,
        overflow.viewport_width,
        overflow.content_width,
        panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    );

    assert!(!overflow.overflows_x());
    assert_eq!(track.width, model.horizontal_thumb_rect(0).width);
    assert!(!panel_scrollbars::horizontal_bar_visible_for(
        region,
        "button",
        Default::default(),
        true,
    ));
}

#[test]
fn horizontal_bar_reacts_to_dynamic_outer_content_width_change() {
    let region = panel_scroll_state::PanelScrollRegion::Preview;
    let track = panel_scrollbars::horizontal_track_rect_for(region);
    let viewport = panel_layout::region_layout(region).content_viewport;

    let overflow_fits = panel_scroll_state::PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height,
    );
    let overflow_overflowed = panel_scroll_state::PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width * 2,
        viewport.height,
    );

    let fits_model = ScrollbarModel::horizontal(
        track,
        overflow_fits.viewport_width,
        overflow_fits.content_width,
        panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    );
    let overflowed_model = ScrollbarModel::horizontal(
        track,
        overflow_overflowed.viewport_width,
        overflow_overflowed.content_width,
        panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    );

    let expected = expected_thumb_len(
        track.width,
        overflow_overflowed.viewport_width,
        overflow_overflowed.content_width,
    );

    assert!(!overflow_fits.overflows_x());
    assert!(overflow_overflowed.overflows_x());
    assert_eq!(track.width, fits_model.horizontal_thumb_rect(0).width);
    assert_eq!(expected, overflowed_model.horizontal_thumb_rect(0).width);
}

#[test]
fn horizontal_bar_reduces_to_no_overflow_when_content_shrinks_back() {
    let region = panel_scroll_state::PanelScrollRegion::Preview;
    let track = panel_scrollbars::horizontal_track_rect_for(region);
    let viewport = panel_layout::region_layout(region).content_viewport;

    let expanded = panel_scroll_state::PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width * 2,
        viewport.height,
    );
    let shrunk = panel_scroll_state::PanelOverflow::new(
        viewport.width,
        viewport.height,
        viewport.width,
        viewport.height,
    );

    let expanded_model = ScrollbarModel::horizontal(
        track,
        expanded.viewport_width,
        expanded.content_width,
        panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    );
    let shrunk_model = ScrollbarModel::horizontal(
        track,
        shrunk.viewport_width,
        shrunk.content_width,
        panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    );

    assert!(expanded.overflows_x());
    assert!(!shrunk.overflows_x());
    assert!(
        expanded_model
            .horizontal_thumb_rect(expanded.max_x())
            .right()
            <= track.right()
    );
    assert_eq!(track.width, shrunk_model.horizontal_thumb_rect(0).width);
}

fn expected_thumb_len(track_len: usize, viewport_len: usize, content_len: usize) -> usize {
    if content_len <= viewport_len {
        return track_len;
    }
    (track_len * viewport_len / content_len)
        .max(panel_scrollbars::PANEL_SCROLLBAR_THUMB_MIN_LENGTH)
        .min(track_len)
}
