use super::layout_model::TextSurfaceLayout;
use super::scroll_request_types::{
    TextSurfaceLogicalPixels, TextSurfaceScrollAlignment, TextSurfaceScrollRequest,
    TextSurfaceScrollRequestRejection, TextSurfaceScrollTarget,
};
use super::state::TextSurfaceScrollBounds;
use crate::render_model::UiRect;

pub(super) fn target_bounds(
    layout: &TextSurfaceLayout,
    target: &TextSurfaceScrollTarget,
) -> Result<Option<UiRect>, TextSurfaceScrollRequestRejection> {
    match target {
        TextSurfaceScrollTarget::LogicalRow { logical_row } => layout
            .line_bounds(*logical_row)
            .map(Some)
            .ok_or(TextSurfaceScrollRequestRejection::LogicalRowNotFound),
        TextSurfaceScrollTarget::ByteOffset { byte_offset } => layout
            .bounds_for_byte_offset(*byte_offset)
            .map(Some)
            .ok_or(TextSurfaceScrollRequestRejection::InvalidUtf8Boundary),
        TextSurfaceScrollTarget::ByteRange {
            byte_start,
            byte_end,
        } => layout
            .bounds_for_byte_range(*byte_start, *byte_end)
            .map(Some)
            .ok_or_else(|| {
                if byte_start > byte_end {
                    TextSurfaceScrollRequestRejection::InvalidByteRange
                } else {
                    TextSurfaceScrollRequestRejection::InvalidUtf8Boundary
                }
            }),
        TextSurfaceScrollTarget::RelativePixels { delta_x, delta_y } => {
            if delta_x.is_finite() && delta_y.is_finite() {
                Ok(None)
            } else {
                Err(TextSurfaceScrollRequestRejection::NonFiniteRelativePixels)
            }
        }
    }
}

pub(super) fn aligned_scroll_offset(
    current_x: i32,
    current_y: i32,
    target: Option<UiRect>,
    viewport: UiRect,
    request: &TextSurfaceScrollRequest,
    bounds: TextSurfaceScrollBounds,
    scale_factor: f32,
) -> (i32, i32) {
    let (mut next_x, mut next_y) = (current_x, current_y);
    match (&request.target, target) {
        (TextSurfaceScrollTarget::RelativePixels { delta_x, delta_y }, _) => {
            next_x = next_x.saturating_add(normalize_logical_pixels(*delta_x, scale_factor));
            next_y = next_y.saturating_add(normalize_logical_pixels(*delta_y, scale_factor));
        }
        (_, Some(target)) => {
            let target_top = target.y.saturating_add(current_y);
            let target_bottom = target_top.saturating_add(target.height as i32);
            let viewport_top = viewport.y;
            let viewport_bottom = viewport.y.saturating_add(viewport.height as i32);
            next_y = match request.alignment {
                TextSurfaceScrollAlignment::Start => target_top.saturating_sub(viewport_top),
                TextSurfaceScrollAlignment::Center => {
                    target_top.saturating_sub(viewport_top).saturating_sub(
                        (viewport.height as i32).saturating_sub(target.height as i32) / 2,
                    )
                }
                TextSurfaceScrollAlignment::End => target_bottom.saturating_sub(viewport_bottom),
                TextSurfaceScrollAlignment::Nearest => {
                    if target.y < viewport_top {
                        target_top.saturating_sub(viewport_top)
                    } else if target.y.saturating_add(target.height as i32) > viewport_bottom {
                        target_bottom.saturating_sub(viewport_bottom)
                    } else {
                        current_y
                    }
                }
            };
        }
        (_, None) => {}
    }
    (next_x.clamp(0, bounds.max_x), next_y.clamp(0, bounds.max_y))
}

fn normalize_logical_pixels(value: TextSurfaceLogicalPixels, scale_factor: f32) -> i32 {
    let scale = if scale_factor.is_finite() && scale_factor > 0.0 {
        scale_factor
    } else {
        1.0
    };
    let device_pixels = (value.value() * scale).round();
    let logical_pixels = (device_pixels / scale).round();
    logical_pixels.clamp(i32::MIN as f32, i32::MAX as f32) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_model::UiRect;
    use crate::text_surface::scroll_request_types::{
        TextSurfaceLogicalPixels, TextSurfaceScrollAlignment, TextSurfaceScrollRequest,
        TextSurfaceScrollRequestToken,
    };
    use crate::text_surface::state::TextSurfaceScrollBounds;
    use crate::text_surface::{
        layout_model::{TextSurfaceGraphemeBox, TextSurfaceLayout},
        scroll_request_types::TextSurfaceScrollTarget,
    };

    #[test]
    fn target_bounds_covers_all_target_kinds_and_rejections() {
        let layout = TextSurfaceLayout::from_grapheme_boxes(
            "scroll-request-layout",
            UiRect::new(0, 0, 60, 40),
            "ab\nc",
            vec![
                TextSurfaceGraphemeBox {
                    grapheme_index: 0,
                    byte_start: 0,
                    byte_end: 1,
                    bounds: UiRect::new(0, 0, 10, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 1,
                    byte_start: 1,
                    byte_end: 2,
                    bounds: UiRect::new(10, 0, 10, 20),
                },
                TextSurfaceGraphemeBox {
                    grapheme_index: 2,
                    byte_start: 2,
                    byte_end: 3,
                    bounds: UiRect::new(0, 20, 10, 20),
                },
            ],
        );

        assert_eq!(
            Some(UiRect::new(0, 20, 10, 20)),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::LogicalRow { logical_row: 1 }
            )
            .ok()
            .flatten()
        );
        assert_eq!(
            Some(UiRect::new(0, 0, 10, 20)),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::ByteOffset { byte_offset: 0 }
            )
            .ok()
            .flatten()
        );
        assert_eq!(
            Some(UiRect::new(0, 0, 20, 20)),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::ByteRange {
                    byte_start: 0,
                    byte_end: 2,
                },
            )
            .ok()
            .flatten()
        );
        assert_eq!(
            None,
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::RelativePixels {
                    delta_x: TextSurfaceLogicalPixels::new(2.0),
                    delta_y: TextSurfaceLogicalPixels::new(-1.0),
                },
            )
            .ok()
            .flatten()
        );
        assert_eq!(
            Err(TextSurfaceScrollRequestRejection::LogicalRowNotFound),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::LogicalRow { logical_row: 9 }
            ),
        );
        assert_eq!(
            Err(TextSurfaceScrollRequestRejection::InvalidByteRange),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::ByteRange {
                    byte_start: 2,
                    byte_end: 1,
                }
            ),
        );
        assert_eq!(
            Err(TextSurfaceScrollRequestRejection::InvalidUtf8Boundary),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::ByteRange {
                    byte_start: 1,
                    byte_end: 9,
                }
            ),
        );
        assert_eq!(
            Err(TextSurfaceScrollRequestRejection::NonFiniteRelativePixels),
            target_bounds(
                &layout,
                &TextSurfaceScrollTarget::RelativePixels {
                    delta_x: TextSurfaceLogicalPixels::new(f32::INFINITY),
                    delta_y: TextSurfaceLogicalPixels::new(1.0),
                },
            ),
        );
    }

    #[test]
    fn aligned_scroll_offset_branches_by_alignment_and_clamps_into_bounds() {
        let request_start = TextSurfaceScrollRequest::new(
            TextSurfaceScrollRequestToken::new("start"),
            TextSurfaceScrollTarget::RelativePixels {
                delta_x: TextSurfaceLogicalPixels::new(3.0),
                delta_y: TextSurfaceLogicalPixels::new(4.0),
            },
            TextSurfaceScrollAlignment::Center,
        );
        let bounds = TextSurfaceScrollBounds::from_extents(20, 20, 8, 8);
        let (start_x, start_y) = aligned_scroll_offset(
            6,
            10,
            None,
            UiRect::new(0, 0, 8, 8),
            &request_start,
            bounds,
            2.0,
        );
        assert_eq!(9, start_x);
        assert_eq!(12, start_y);

        let request_nearest = TextSurfaceScrollRequest::new(
            TextSurfaceScrollRequestToken::new("nearest"),
            TextSurfaceScrollTarget::ByteOffset { byte_offset: 0 },
            TextSurfaceScrollAlignment::Nearest,
        );
        let (nearest_x, nearest_y) = aligned_scroll_offset(
            20,
            20,
            Some(UiRect::new(0, 2, 4, 2)),
            UiRect::new(0, 0, 8, 8),
            &request_nearest,
            TextSurfaceScrollBounds::from_extents(20, 20, 8, 8),
            1.0,
        );
        assert_eq!(12, nearest_x);
        assert_eq!(12, nearest_y);

        let (clamped_x, clamped_y) = aligned_scroll_offset(
            11,
            11,
            Some(UiRect::new(20, 20, 8, 8)),
            UiRect::new(0, 0, 8, 8),
            &TextSurfaceScrollRequest::new(
                TextSurfaceScrollRequestToken::new("end"),
                TextSurfaceScrollTarget::ByteOffset { byte_offset: 2 },
                TextSurfaceScrollAlignment::End,
            ),
            TextSurfaceScrollBounds::from_extents(20, 20, 8, 8),
            1.0,
        );
        assert_eq!(11, clamped_x);
        assert_eq!(12, clamped_y);

        let (_, centered_y) = aligned_scroll_offset(
            0,
            0,
            Some(UiRect::new(0, 8, 4, 2)),
            UiRect::new(0, 0, 8, 8),
            &TextSurfaceScrollRequest::new(
                TextSurfaceScrollRequestToken::new("center"),
                TextSurfaceScrollTarget::ByteOffset { byte_offset: 0 },
                TextSurfaceScrollAlignment::Center,
            ),
            TextSurfaceScrollBounds::from_extents(20, 20, 8, 8),
            1.0,
        );
        assert_eq!(5, centered_y);

        for (target, expected_y) in [
            (UiRect::new(0, -2, 4, 2), 8),
            (UiRect::new(0, 12, 4, 2), 12),
            (UiRect::new(0, 2, 4, 2), 10),
        ] {
            let (_, y) = aligned_scroll_offset(
                0,
                10,
                Some(target),
                UiRect::new(0, 0, 8, 8),
                &request_nearest,
                TextSurfaceScrollBounds::from_extents(20, 20, 8, 8),
                1.0,
            );
            assert_eq!(expected_y, y);
        }

        let no_target = aligned_scroll_offset(
            3,
            4,
            None,
            UiRect::new(0, 0, 8, 8),
            &TextSurfaceScrollRequest::new(
                TextSurfaceScrollRequestToken::new("no-target"),
                TextSurfaceScrollTarget::ByteOffset { byte_offset: 0 },
                TextSurfaceScrollAlignment::Nearest,
            ),
            TextSurfaceScrollBounds::from_extents(20, 20, 8, 8),
            f32::NAN,
        );
        assert_eq!((3, 4), no_target);

        assert_eq!(
            (4, 5),
            aligned_scroll_offset(
                3,
                4,
                None,
                UiRect::new(0, 0, 8, 8),
                &TextSurfaceScrollRequest::new(
                    TextSurfaceScrollRequestToken::new("invalid-scale"),
                    TextSurfaceScrollTarget::RelativePixels {
                        delta_x: TextSurfaceLogicalPixels::new(1.0),
                        delta_y: TextSurfaceLogicalPixels::new(1.0),
                    },
                    TextSurfaceScrollAlignment::Nearest,
                ),
                TextSurfaceScrollBounds::from_extents(20, 20, 8, 8),
                f32::NAN,
            )
        );
    }
}
