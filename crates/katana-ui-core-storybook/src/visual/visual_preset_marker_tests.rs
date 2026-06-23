use super::{Canvas, StorybookVisual, layout_metrics, palette, preview_detail, preview_effects};
use katana_ui_core::theme::ThemeSnapshot;

const THEME_PRESET_INDEX: usize = 3;

#[test]
fn theme_tokens_later_preset_does_not_render_generic_preset_number_marker() {
    let canvas = StorybookVisual.render_preset("dark", "theme-tokens", THEME_PRESET_INDEX, 0);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = preview_detail::component_action_hit_rect("theme-tokens");
    let marker = preview_effects::legacy_preset_marker_rect_for_test(rect);

    assert!(accent_pixel_count(&canvas, marker, palette.accent) < 8);
}

fn accent_pixel_count(canvas: &Canvas, rect: layout_metrics::LayoutRect, accent: u32) -> usize {
    let mut count = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            if pixel_at(canvas, current_x, current_y) == Some(accent) {
                count += 1;
            }
        }
    }
    count
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
