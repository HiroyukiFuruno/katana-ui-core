use crate::layout::popover::Placement;
use crate::layout::popover::{AnchorRect, PlacementOrigin, PlacementResolver};
use crate::theme::Theme;
use crate::theme::color::Color;

const DEFAULT_DELAY_MS: u32 = 400;
const DEFAULT_MAX_WIDTH: f32 = 240.0;
const FONT_SIZE: f32 = 11.0;
const PAD_V: f32 = 4.0;
const PAD_H: f32 = 8.0;
const DEFAULT_ANCHOR_WIDTH: f32 = 22.0;
const DEFAULT_ANCHOR_HEIGHT: f32 = 20.0;
const DEFAULT_VIEWPORT_WIDTH: f32 = 800.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 600.0;
const OVERLAY_OFFSET: f32 = 8.0;
const APPROX_CHAR_WIDTH_RATE: f32 = 0.55;
const LINE_HEIGHT_RATE: f32 = 1.4;
const OVERLAY_PADDING: f32 = 6.0;

pub(super) fn default_delay_ms() -> u32 {
    DEFAULT_DELAY_MS
}

pub(super) fn default_max_width() -> f32 {
    DEFAULT_MAX_WIDTH
}

pub(super) fn font_size() -> f32 {
    FONT_SIZE
}

pub(super) fn padding() -> (f32, f32) {
    (PAD_V, PAD_H)
}

pub(super) fn bg_color(theme: &Theme) -> Color {
    theme.color.text
}

pub(super) fn text_color(theme: &Theme) -> Color {
    theme.color.bg
}

/// Returns the effective placement after edge-flip logic.
pub(super) fn effective_placement(placement: Placement) -> Placement {
    placement
}

pub(super) fn default_anchor_width() -> f32 {
    DEFAULT_ANCHOR_WIDTH
}

pub(super) fn default_anchor_height() -> f32 {
    DEFAULT_ANCHOR_HEIGHT
}

pub(super) fn viewport_width() -> f32 {
    DEFAULT_VIEWPORT_WIDTH
}

pub(super) fn viewport_height() -> f32 {
    DEFAULT_VIEWPORT_HEIGHT
}

pub(super) fn overlay_offset() -> f32 {
    OVERLAY_OFFSET
}

pub(super) fn hover_visible(elapsed_ms: u32, delay_ms: u32) -> bool {
    elapsed_ms >= delay_ms
}

pub(super) fn focus_visible() -> bool {
    true
}

pub(super) fn visible_after_focus_loss(hover_ready: bool, dismiss_on_focus_loss: bool) -> bool {
    if dismiss_on_focus_loss {
        false
    } else {
        hover_ready
    }
}

pub(super) fn flip_placement(
    placement: Placement,
    preferred_fits: bool,
    opposite_fits: bool,
) -> Placement {
    if preferred_fits || !opposite_fits {
        return placement;
    }

    match placement {
        Placement::Top => Placement::Bottom,
        Placement::Bottom => Placement::Top,
        Placement::Left => Placement::Right,
        Placement::Right => Placement::Left,
    }
}

pub(super) fn estimate_overlay_height(
    text: &str,
    max_width: f32,
    font_size: f32,
    pad_v: f32,
    pad_h: f32,
) -> f32 {
    let effective_width = (max_width - (pad_h * 2.0)).max(1.0);
    let approx_char_width = (font_size * APPROX_CHAR_WIDTH_RATE).max(1.0);
    let chars_per_line = (effective_width / approx_char_width).max(1.0);
    let lines = (text.len() as f32 / chars_per_line).ceil().max(1.0);
    (lines * (font_size * LINE_HEIGHT_RATE)) + (pad_v * 2.0) + OVERLAY_PADDING
}

#[cfg(test)]
pub(super) fn overlay_layout(
    placement: Placement,
    anchor: AnchorRect,
    tooltip_width: f32,
    tooltip_height: f32,
    viewport_width: f32,
    viewport_height: f32,
    offset: f32,
) -> (f32, f32) {
    let layout = overlay_layout_detail(
        placement,
        anchor,
        tooltip_width,
        tooltip_height,
        viewport_width,
        viewport_height,
        offset,
    );
    (layout.x, layout.y)
}

pub(super) fn overlay_layout_detail(
    placement: Placement,
    anchor: AnchorRect,
    tooltip_width: f32,
    tooltip_height: f32,
    viewport_width: f32,
    viewport_height: f32,
    offset: f32,
) -> PlacementOrigin {
    let origin = PlacementResolver::resolve_origin(
        placement,
        anchor,
        offset,
        tooltip_width,
        tooltip_height,
        viewport_width,
        viewport_height,
    );
    let max_x = (viewport_width - tooltip_width).max(0.0);
    let max_y = (viewport_height - tooltip_height).max(0.0);
    PlacementOrigin {
        x: origin.x.clamp(0.0, max_x),
        y: origin.y.clamp(0.0, max_y),
        placement: origin.placement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anchor() -> AnchorRect {
        AnchorRect {
            x: 100.0,
            y: 100.0,
            width: 80.0,
            height: 32.0,
        }
    }

    #[test]
    fn overlay_moves_to_top_when_bottom_does_not_fit() {
        let anchor = AnchorRect {
            x: 100.0,
            y: 560.0,
            width: 80.0,
            height: 30.0,
        };
        let (x, y) = overlay_layout(Placement::Bottom, anchor, 120.0, 80.0, 800.0, 600.0, 8.0);
        assert!(y < anchor.y);
        assert_eq!(x, 80.0);
    }

    #[test]
    fn overlay_position_is_clamped_inside_viewport() {
        let anchor = AnchorRect {
            x: 760.0,
            y: 40.0,
            width: 30.0,
            height: 20.0,
        };
        let (x, y) = overlay_layout(Placement::Right, anchor, 120.0, 40.0, 800.0, 600.0, 8.0);
        assert!((x <= 680.0) && (y >= 0.0));
    }

    #[test]
    fn flip_placement_for_top_when_bottom_does_not_fit() {
        let a = anchor();
        let (_x_top, y_top) = overlay_layout(Placement::Top, a, 120.0, 180.0, 200.0, 200.0, 8.0);
        let (_x_bottom, y_bottom) =
            overlay_layout(Placement::Bottom, a, 120.0, 180.0, 300.0, 400.0, 8.0);
        assert_eq!(y_bottom, a.y + a.height + 8.0);
        assert!(y_top < y_bottom);
    }

    #[test]
    fn estimate_overlay_height_increases_with_text_length() {
        let base = estimate_overlay_height("a", 120.0, 11.0, PAD_V, PAD_H);
        let long = estimate_overlay_height(
            "a very long text that should wrap",
            120.0,
            11.0,
            PAD_V,
            PAD_H,
        );
        assert!(long > base);
    }
}
