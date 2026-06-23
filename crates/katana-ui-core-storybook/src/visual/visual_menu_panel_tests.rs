use super::{StorybookVisual, layout_metrics, palette, preview_detail};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn menu_preview_draws_menu_items_inside_panel_surface() {
    let canvas = StorybookVisual.render_preset("dark", "menu", 0, 0);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = preview_detail::component_action_hit_rect("menu");
    let panel_x = rect.x + 26;
    let panel_y = rect.y + 26;
    let inside_x = panel_x + 200;
    let inside_y = panel_y + 10;

    assert_eq!(Some(palette.panel), pixel_at(&canvas, inside_x, inside_y));
    assert_eq!(Some(palette.border), pixel_at(&canvas, panel_x, panel_y));
}

#[test]
fn navigation_menu_list_is_drawn_inside_a_panel() {
    let canvas = StorybookVisual.render_preset("dark", "button", 0, 0);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let panel_x = layout_metrics::NAV_ROW_X - 6;
    let panel_y = layout_metrics::NAV_FIRST_ROW_Y - 8;

    assert_eq!(Some(palette.border), pixel_at(&canvas, panel_x, panel_y));
}

fn pixel_at(canvas: &super::Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
