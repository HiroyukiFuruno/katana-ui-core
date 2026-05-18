use super::{Canvas, StorybookVisual, layout_metrics, palette};
use katana_ui_core::theme::ThemeSnapshot;

const ACTIVE_TAB_SAMPLE_X_OFFSET: usize = layout_metrics::PRESET_WIDTH / 2;
const ACTIVE_TAB_SAMPLE_Y_OFFSET: usize = 1;
const OPERATION_DIFF_THRESHOLD: usize = 8_000;
const CANVAS_WIDTH: usize = 1440;
const CANVAS_HEIGHT: usize = 920;
const MIN_NON_BACKGROUND_PIXELS: usize = 10_000;
const EDGE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;

#[test]
fn visual_renderer_draws_nonblank_panel() {
    let canvas = StorybookVisual.render();

    assert_eq!(CANVAS_WIDTH, canvas.width());
    assert_eq!(CANVAS_HEIGHT, canvas.height());
    assert!(canvas.non_background_pixels(palette::DEFAULT_BACKGROUND) > MIN_NON_BACKGROUND_PIXELS);
}

#[test]
fn visual_renderer_covers_required_ui_without_fallback() {
    let report = StorybookVisual.coverage_report();

    assert_eq!(
        crate::requirements::StoryRequirements::required_pages().len(),
        report.required_ui
    );
    assert!(report.modal_required);
    assert_eq!(0, report.required_ui_fallbacks);
    assert_eq!(0, report.initial_visible_fallbacks);
}

#[test]
fn active_preset_tab_has_measured_bottom_accent() {
    let canvas = StorybookVisual.render_scenario("dark", "button", false);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let active = layout_metrics::preset_tab_rect(0);
    let inactive = layout_metrics::preset_tab_rect(1);
    let active_bottom_y = active.bottom() - ACTIVE_TAB_SAMPLE_Y_OFFSET;

    assert_eq!(
        Some(palette.accent),
        pixel_at(
            &canvas,
            active.x + ACTIVE_TAB_SAMPLE_X_OFFSET,
            active_bottom_y
        )
    );
    assert_ne!(
        Some(palette.accent),
        pixel_at(
            &canvas,
            inactive.x + ACTIVE_TAB_SAMPLE_X_OFFSET,
            active_bottom_y
        )
    );
}

#[test]
fn operation_preset_changes_tab_and_canvas_pixels() {
    let before = StorybookVisual.render_scenario("dark", "button", false);
    let after = StorybookVisual.render_scenario("dark", "button", true);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let active = layout_metrics::preset_tab_rect(layout_metrics::PRESET_INTERACTIVE_INDEX);
    let active_y = active.bottom() - ACTIVE_TAB_SAMPLE_Y_OFFSET;

    assert_eq!(
        Some(palette.accent),
        pixel_at(&after, active.x + ACTIVE_TAB_SAMPLE_X_OFFSET, active_y)
    );
    assert!(pixel_diff(&before, &after) > OPERATION_DIFF_THRESHOLD);
}

#[test]
fn later_preset_tabs_render_as_selected() {
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let edge = StorybookVisual.render_preset("dark", "button", EDGE_PRESET_INDEX, 0);
    let theme = StorybookVisual.render_preset("dark", "button", THEME_PRESET_INDEX, 0);
    let edge_rect = layout_metrics::preset_tab_rect(EDGE_PRESET_INDEX);
    let theme_rect = layout_metrics::preset_tab_rect(THEME_PRESET_INDEX);
    let edge_y = edge_rect.bottom() - ACTIVE_TAB_SAMPLE_Y_OFFSET;
    let theme_y = theme_rect.bottom() - ACTIVE_TAB_SAMPLE_Y_OFFSET;

    assert_eq!(
        Some(palette.accent),
        pixel_at(&edge, edge_rect.x + ACTIVE_TAB_SAMPLE_X_OFFSET, edge_y)
    );
    assert_eq!(
        Some(palette.accent),
        pixel_at(&theme, theme_rect.x + ACTIVE_TAB_SAMPLE_X_OFFSET, theme_y)
    );
}

#[test]
fn scrolled_storybook_viewport_changes_pixels() {
    let before = StorybookVisual.render_scenario("dark", "button", false);
    let after =
        StorybookVisual.render_scrolled("dark", "button", false, layout_metrics::SCROLL_STEP);

    assert!(pixel_diff(&before, &after) > OPERATION_DIFF_THRESHOLD);
}

#[test]
fn scrollbar_visibility_is_rendered_from_state() {
    let visible = StorybookVisual.render_preset_with_scrollbar("dark", "button", 0, 0, true);
    let hidden = StorybookVisual.render_preset_with_scrollbar("dark", "button", 0, 0, false);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let thumb = super::scrollbar::thumb_rect(0);

    assert_eq!(Some(palette.accent), pixel_at(&visible, thumb.x, thumb.y));
    assert_ne!(Some(palette.accent), pixel_at(&hidden, thumb.x, thumb.y));
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    before
        .pixels()
        .iter()
        .zip(after.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}
