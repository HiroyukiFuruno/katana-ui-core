use crate::render_model::UiRect;

pub(super) fn clip_to_viewport(bounds: UiRect, viewport: UiRect) -> UiRect {
    let left = i64::from(bounds.x).max(i64::from(viewport.x));
    let top = i64::from(bounds.y).max(i64::from(viewport.y));
    let right = (i64::from(bounds.x) + i64::from(bounds.width))
        .min(i64::from(viewport.x) + i64::from(viewport.width));
    let bottom = (i64::from(bounds.y) + i64::from(bounds.height))
        .min(i64::from(viewport.y) + i64::from(viewport.height));
    UiRect::new(
        left as i32,
        top as i32,
        u32::try_from((right - left).max(0)).unwrap_or_default(),
        u32::try_from((bottom - top).max(0)).unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::clip_to_viewport;
    use crate::render_model::UiRect;

    #[test]
    fn selection_bounds_with_an_unbounded_right_edge_are_clipped_to_the_viewport() {
        let viewport = UiRect::new(0, 33, 640, 327);
        assert_eq!(
            UiRect::new(0, 33, 640, 20),
            clip_to_viewport(UiRect::new(0, 33, u32::MAX, 20), viewport),
        );
    }
}
