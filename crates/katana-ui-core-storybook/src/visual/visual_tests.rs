use super::{
    Canvas, StorybookVisual, layout_metrics, palette, preview, preview_contract, preview_detail,
};
use crate::catalog::story_map::StoryGroup;
use crate::visual::dedicated_dod_molecule_tree_parts as tree_parts;
use crate::visual::navigation_tree::{
    NavigationRow, TreeExpansionState, row_from_click, visible_rows,
};
use crate::visual::render;
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
    assert!(report.tree_view_node_marker_option_visible);
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
fn tree_view_preview_renders_depth_guides_disclosure_and_markers() {
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);
    let root_row_y = preview_detail::HERO_PREVIEW_Y_FOR_TEST
        + tree_parts::TREE_PANEL_Y
        + tree_parts::ROW_HEIGHT / 2;
    let child_row_y = root_row_y + tree_parts::ROW_HEIGHT;
    let grandchild_row_y = child_row_y + tree_parts::ROW_HEIGHT;
    let root_disclosure_x = preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::DISCLOSURE_X + 4;
    let child_disclosure_x = preview_detail::HERO_PREVIEW_X_FOR_TEST
        + tree_parts::DISCLOSURE_X
        + tree_parts::INDENT_STEP
        + 4;
    let root_marker_x = preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::NODE_ICON_X + 6;
    let child_marker_x = preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::CHILD_ICON_X + 6;
    let grandchild_marker_x =
        preview_detail::HERO_PREVIEW_X_FOR_TEST + tree_parts::GRANDCHILD_ICON_X + 6;

    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, root_disclosure_x, root_row_y)
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, child_disclosure_x, child_row_y)
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, root_marker_x, root_row_y)
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, child_marker_x, child_row_y)
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, grandchild_marker_x, grandchild_row_y)
    );
}

#[test]
fn navigation_tree_lines_obey_show_navigation_lines_option() {
    let expansion = TreeExpansionState::default();
    let with_lines = render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: "dark",
        selected_page: "button",
        preset_index: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: crate::visual::panel_scroll_state::PanelScrollOffsets::default(),
        tree_expansion: expansion,
        show_navigation_lines: true,
        screen_state: crate::visual::screen_state::StorybookScreenState::default(),
    });
    let without_lines =
        render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
            theme_id: "dark",
            selected_page: "button",
            preset_index: 0,
            scroll_y: 0,
            scrollbar_visible: true,
            panel_scroll: crate::visual::panel_scroll_state::PanelScrollOffsets::default(),
            tree_expansion: expansion,
            show_navigation_lines: false,
            screen_state: crate::visual::screen_state::StorybookScreenState::default(),
        });
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    let (row_y, row_depth) = row_y_and_depth_in_navigation("tree-view", expansion)
        .expect("target row should be visible");
    let line_x = navigation_line_x(row_depth);
    let row_center_y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;
    let below_row_center_y = row_center_y + 2;

    assert_eq!(
        Some(palette.border),
        pixel_at(&with_lines, line_x, row_center_y)
    );
    assert_eq!(
        Some(palette.code_background),
        pixel_at(&without_lines, line_x, row_center_y)
    );
    assert_eq!(
        Some(palette.code_background),
        pixel_at(&without_lines, line_x, below_row_center_y)
    );
    assert_ne!(
        pixel_at(&without_lines, line_x, row_center_y),
        Some(palette.border)
    );
}

#[test]
fn navigation_section_disclosure_is_indented_right_of_group_disclosure() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);

    let group_row_y = navigation_row_y_for_group(expansion, StoryGroup::Foundation)
        .expect("group row should be present in default navigation expansion");
    let section_row_y = navigation_row_y_for_section(expansion)
        .expect("section row should be present in default navigation expansion");
    let group_disclosure_x = navigation_disclosure_center_x(0);
    let section_disclosure_x = navigation_disclosure_center_x(1);
    let group_row_disclosure_y = navigation_disclosure_center_y(group_row_y);
    let section_row_disclosure_y = navigation_disclosure_center_y(section_row_y);

    assert!(section_disclosure_x > group_disclosure_x);
    assert_eq!(
        Some(palette.text),
        pixel_at(&canvas, group_disclosure_x, group_row_disclosure_y)
    );
    assert_eq!(
        Some(palette.text),
        pixel_at(&canvas, section_disclosure_x, section_row_disclosure_y)
    );
    assert_ne!(
        Some(palette.text),
        pixel_at(&canvas, group_disclosure_x, section_row_disclosure_y)
    );
}

#[test]
fn navigation_selected_page_does_not_render_page_icon_style_square_marker() {
    const LEGACY_MARK_X: usize = 74;
    const LEGACY_MARK_SIZE: usize = 14;

    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);
    let colors = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let expansion = TreeExpansionState::default();
    let row_y = (layout_metrics::NAV_FIRST_ROW_Y..layout_metrics::CONTENT_HEIGHT).find(|y| {
        matches!(
            row_from_click(layout_metrics::NAV_ROW_X + 1, *y, expansion),
            Some(
                NavigationRow::Page {
                    page: "tree-view",
                    ..
                } | NavigationRow::PageWithoutSection {
                    page: "tree-view",
                    ..
                }
            )
        )
    });
    assert!(row_y.is_some(), "tree-view navigation row should exist");
    let row_y = row_y.unwrap_or(layout_metrics::NAV_FIRST_ROW_Y);
    let mark_y = row_y + (layout_metrics::NAV_ROW_HEIGHT - LEGACY_MARK_SIZE) / 2;
    let mut accent_border_pixels = 0usize;
    for x in LEGACY_MARK_X..LEGACY_MARK_X + LEGACY_MARK_SIZE {
        if pixel_at(&canvas, x, mark_y) == Some(colors.accent) {
            accent_border_pixels += 1;
        }
        if pixel_at(&canvas, x, mark_y + LEGACY_MARK_SIZE - 1) == Some(colors.accent) {
            accent_border_pixels += 1;
        }
    }
    for y in mark_y..mark_y + LEGACY_MARK_SIZE {
        if pixel_at(&canvas, LEGACY_MARK_X, y) == Some(colors.accent) {
            accent_border_pixels += 1;
        }
        if pixel_at(&canvas, LEGACY_MARK_X + LEGACY_MARK_SIZE - 1, y) == Some(colors.accent) {
            accent_border_pixels += 1;
        }
    }
    assert_eq!(
        0, accent_border_pixels,
        "left navigation must not render page icon-like accent square marker"
    );
}

fn navigation_line_x(depth: usize) -> usize {
    match depth {
        0 => 54,
        1 => 68,
        _ => 84,
    }
}

fn row_y_and_depth_in_navigation(
    page: &str,
    expansion: TreeExpansionState,
) -> Option<(usize, usize)> {
    for y in layout_metrics::NAV_FIRST_ROW_Y..layout_metrics::CONTENT_HEIGHT {
        let Some(row) = row_from_click(layout_metrics::NAV_ROW_X + 1, y, expansion) else {
            continue;
        };
        let depth = match row {
            NavigationRow::Group { .. } => 0,
            NavigationRow::Section { .. } => 1,
            NavigationRow::Page { page: row_page, .. } if row_page == page => 2,
            NavigationRow::PageWithoutSection { page: row_page, .. } if row_page == page => 1,
            _ => continue,
        };
        return Some((y, depth));
    }
    None
}

fn navigation_row_y_for_group(expansion: TreeExpansionState, group: StoryGroup) -> Option<usize> {
    visible_rows(expansion)
        .iter()
        .position(|row| matches!(row, NavigationRow::Group(found_group) if *found_group == group))
        .map(|index| layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP)
}

fn navigation_row_y_for_section(expansion: TreeExpansionState) -> Option<usize> {
    visible_rows(expansion)
        .iter()
        .position(|row| matches!(row, NavigationRow::Section { .. }))
        .map(|index| layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP)
}

fn navigation_disclosure_center_x(depth: usize) -> usize {
    navigation_disclosure_left_x(depth) + navigation_disclosure_center_offset()
}

fn navigation_disclosure_left_x(depth: usize) -> usize {
    navigation_line_x(depth).saturating_sub(4)
}

fn navigation_disclosure_center_offset() -> usize {
    navigation_disclosure_size() / 2
}

fn navigation_disclosure_size() -> usize {
    7
}

fn navigation_disclosure_center_y(row_y: usize) -> usize {
    row_y
        + (layout_metrics::NAV_ROW_HEIGHT - navigation_disclosure_size()) / 2
        + navigation_disclosure_center_offset()
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
