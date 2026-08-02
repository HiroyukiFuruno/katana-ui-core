#[path = "panel_options_draw.rs"]
mod panel_options_draw;

use super::layout_metrics::{
    panel_active_details_rect, panel_active_nav_rect, panel_active_preview_rect,
    panel_scrollbar_off_rect, panel_scrollbar_on_rect,
};
use super::panel_screen_state::{PanelChildKey, PanelOptionControl};

pub(super) use panel_options_draw::draw_controls;

pub(super) fn control_at(x: usize, y: usize) -> Option<PanelOptionControl> {
    if panel_active_nav_rect().contains(x, y) {
        return Some(PanelOptionControl::ActivePanel(PanelChildKey::Navigation));
    }
    if panel_active_preview_rect().contains(x, y) {
        return Some(PanelOptionControl::ActivePanel(PanelChildKey::Preview));
    }
    if panel_active_details_rect().contains(x, y) {
        return Some(PanelOptionControl::ActivePanel(PanelChildKey::Details));
    }
    if panel_scrollbar_on_rect().contains(x, y) {
        return Some(PanelOptionControl::ScrollbarVisible(true));
    }
    if panel_scrollbar_off_rect().contains(x, y) {
        return Some(PanelOptionControl::ScrollbarVisible(false));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_option_hit_testing_covers_every_control() {
        let cases = [
            (
                panel_active_nav_rect(),
                PanelOptionControl::ActivePanel(PanelChildKey::Navigation),
            ),
            (
                panel_active_preview_rect(),
                PanelOptionControl::ActivePanel(PanelChildKey::Preview),
            ),
            (
                panel_active_details_rect(),
                PanelOptionControl::ActivePanel(PanelChildKey::Details),
            ),
            (
                panel_scrollbar_on_rect(),
                PanelOptionControl::ScrollbarVisible(true),
            ),
            (
                panel_scrollbar_off_rect(),
                PanelOptionControl::ScrollbarVisible(false),
            ),
        ];

        for (rect, expected) in cases {
            assert_eq!(Some(expected), control_at(rect.x, rect.y));
        }
        assert_eq!(None, control_at(usize::MAX, usize::MAX));
    }
}
