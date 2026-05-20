use super::{
    Canvas, StorybookVisual, dedicated_dod_molecule_tree_parts as tree_parts, layout_metrics,
    palette, preview, preview_contract, preview_detail,
};
use katana_ui_core::theme::ThemeSnapshot;
use std::collections::BTreeMap;

const ACTIVE_TAB_SAMPLE_X_OFFSET: usize = layout_metrics::PRESET_WIDTH / 2;
const ACTIVE_TAB_SAMPLE_Y_OFFSET: usize = 1;
const OPERATION_DIFF_THRESHOLD: usize = 8_000;
const OPERATION_DETAIL_DIFF_THRESHOLD: usize = 1_000;
const CANVAS_WIDTH: usize = 1440;
const CANVAS_HEIGHT: usize = 920;
const MIN_NON_BACKGROUND_PIXELS: usize = 10_000;
const EDGE_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const PREVIEW_SIGNATURE_SEED: u64 = 17;
const PREVIEW_SIGNATURE_PRIME: u64 = 1_099_511_628_211;
const LEGACY_DOD_PREVIEW_PAGES: &[&str] = &[
    "theme-tokens",
    "text",
    "icon",
    "chip",
    "loading-dots",
    "spinner",
    "button",
    "text-button",
    "svg-button",
    "icon-text-button",
    "toggle",
    "segmented-toggle",
    "select-box",
    "color-swatch",
    "text-input",
    "search-box",
    "tooltip",
    "badge",
    "key-cap",
    "card",
    "accordion",
    "split-pane",
    "modal",
    "popover",
    "color-picker-rgba",
    "code-diff",
    "attachment-chip",
    "chip-group",
];

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
    assert!(report.selected_preview_visible);
    assert!(report.selected_preview_interaction_visible);
    assert!(report.detail_tables_hidden);
    assert!(report.scrollbar_thumb_bottom);
    assert!(report.contract_rows_fit);
    assert!(report.inspector_rows_fit);
    assert!(report.tree_view_selected);
    assert!(report.tree_view_settings_visible);
    assert!(report.tree_view_line_option_visible);
    assert!(report.tree_view_icon_option_visible);
    assert!(report.tree_view_trigger_option_visible);
    assert!(report.tree_view_action_logged);
    assert!(report.panel_scrollbars_visible);
    assert!(report.navigation_collapsed_pixels_changed > OPERATION_DIFF_THRESHOLD);
    assert_eq!(
        LEGACY_DOD_PREVIEW_PAGES.len(),
        report.legacy_preview_signatures
    );
    assert_eq!(0, report.legacy_preview_signature_collisions);
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
fn active_preset_tab_is_connected_to_preview_surface() {
    let active = layout_metrics::preset_tab_rect(0);

    assert_eq!(active.bottom(), preview_detail::selected_hero_y());
}

#[test]
fn katana_storybook_typography_and_spacing_samples_fit_preview_lane() {
    assert!(preview::summary_controls_right_edge() < layout_metrics::INSPECTOR_X);
    assert_eq!(24, preview::summary_control_height());
    assert_eq!(0, layout_metrics::PRESET_GAP);
    assert_eq!(24, layout_metrics::NAV_ROW_HEIGHT);
    assert_eq!(28, layout_metrics::NAV_ROW_STEP);
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
    assert!(selected_detail_pixel_diff(&before, &after) > OPERATION_DETAIL_DIFF_THRESHOLD);
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
    let sample_x = thumb.x + thumb.width / 2;
    let sample_y = thumb.y + thumb.height / 2;

    assert_eq!(Some(palette.accent), pixel_at(&visible, sample_x, sample_y));
    assert_ne!(Some(palette.accent), pixel_at(&hidden, sample_x, sample_y));
}

#[test]
fn legacy_01_24_pages_have_dedicated_preview_signatures() {
    let mut signatures = BTreeMap::new();
    for page in LEGACY_DOD_PREVIEW_PAGES {
        let canvas = StorybookVisual.render_scenario("dark", page, false);
        let signature = hero_preview_signature(&canvas);
        let previous = signatures.insert(signature, *page);
        assert!(
            previous.is_none(),
            "preview signature duplicated: {previous:?} and {page}"
        );
    }

    assert_eq!(LEGACY_DOD_PREVIEW_PAGES.len(), signatures.len());
}

#[test]
fn tree_view_preview_draws_tree_specific_affordances() {
    let canvas = StorybookVisual.render_preset("dark", "tree-view", 1, 0);
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let rect = preview_detail::component_action_hit_rect("tree-view");
    let row_y = rect.y + tree_parts::TREE_PANEL_Y + 6;
    let selected_row_y = row_y + tree_parts::ROW_HEIGHT;

    assert_eq!(
        Some(palette.accent),
        pixel_at(
            &canvas,
            rect.x + tree_parts::TREE_PANEL_X + 4,
            selected_row_y - 1
        )
    );
    assert_eq!(
        Some(0xd7ba7d),
        pixel_at(&canvas, rect.x + tree_parts::NODE_ICON_X + 2, row_y + 5)
    );
    assert_eq!(
        Some(0x9cdcfe),
        pixel_at(
            &canvas,
            rect.x + tree_parts::CHILD_ICON_X + 2,
            selected_row_y + 5
        )
    );
    assert_eq!(
        Some(palette.border),
        pixel_at(
            &canvas,
            rect.x + tree_parts::LINE_X,
            rect.y + tree_parts::TREE_PANEL_Y + 48
        )
    );
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn hero_preview_signature(canvas: &Canvas) -> u64 {
    let (x, y, width, height) = preview_contract::selected_detail_rect();
    let mut signature = PREVIEW_SIGNATURE_SEED;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * canvas.width() + current_x;
            let pixel = u64::from(canvas.pixels()[index]);
            signature ^= pixel.wrapping_add(index as u64);
            signature = signature.wrapping_mul(PREVIEW_SIGNATURE_PRIME);
        }
    }
    signature
}

fn selected_detail_pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    let (x, y, width, height) = preview_contract::selected_detail_rect();
    let mut diff = 0;
    for current_y in y..y + height {
        for current_x in x..x + width {
            let index = current_y * before.width() + current_x;
            if before.pixels()[index] != after.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}

fn pixel_diff(before: &Canvas, after: &Canvas) -> usize {
    before
        .pixels()
        .iter()
        .zip(after.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}
