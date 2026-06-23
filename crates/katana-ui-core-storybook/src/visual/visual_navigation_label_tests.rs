use super::visual_navigation_label_support::{
    count_text_antialias_pixels, ink_vertical_bounds_in_rect,
};
use super::visual_navigation_support::{
    navigation_label_sample_width, navigation_label_x, navigation_row_y_for_group,
    navigation_row_y_for_section, pixel_at, require_navigation_value,
};
use super::{StorybookVisual, layout_metrics, palette, preview_detail};
use crate::catalog::story_map::StoryGroup;
use crate::visual::dedicated_dod_molecule_tree_parts as tree_parts;
use crate::visual::navigation_tree::{NavigationRow, TreeExpansionState, row_from_click};
use katana_ui_core::theme::ThemeSnapshot;

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
        pixel_at(&canvas, root_disclosure_x, root_row_y),
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, child_disclosure_x, child_row_y),
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, root_marker_x, root_row_y),
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, child_marker_x, child_row_y),
    );
    assert_ne!(
        Some(palette::DEFAULT_BACKGROUND),
        pixel_at(&canvas, grandchild_marker_x, grandchild_row_y),
    );
}

#[test]
fn navigation_disclosure_and_labels_share_row_center() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);
    let rows = [
        (
            0,
            require_navigation_value(
                navigation_row_y_for_group(expansion, StoryGroup::Foundation),
                "group row should be visible",
            )?,
        ),
        (
            1,
            require_navigation_value(
                navigation_row_y_for_section(expansion),
                "section row should be visible",
            )?,
        ),
    ];

    for (depth, row_y) in rows {
        let disclosure_center_y =
            super::visual_navigation_support::navigation_disclosure_center_y(row_y) as f32;
        let bounds = ink_vertical_bounds_in_rect(
            &canvas,
            navigation_label_x(depth),
            row_y,
            navigation_label_sample_width(depth),
            layout_metrics::NAV_ROW_HEIGHT,
            palette.code_background,
        )
        .ok_or_else(|| "navigation label should have visible ink".to_string())?;
        let label_center_y = (bounds.top + bounds.bottom) as f32 / 2.0;

        assert!(
            (label_center_y - disclosure_center_y).abs() <= 2.0,
            "navigation depth {depth} label center {label_center_y} should align with disclosure center {disclosure_center_y}",
        );
    }
    Ok(())
}

#[test]
fn navigation_label_text_uses_antialiased_edges() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);
    let group_row_y = require_navigation_value(
        navigation_row_y_for_group(expansion, StoryGroup::Foundation),
        "group row should be visible",
    )?;
    let antialiased_pixels = count_text_antialias_pixels(
        &canvas,
        navigation_label_x(0),
        group_row_y,
        navigation_label_sample_width(0),
        layout_metrics::NAV_ROW_HEIGHT,
        palette.code_background,
        palette.muted,
    );

    assert!(
        antialiased_pixels > 0,
        "navigation label should contain blended edge pixels",
    );
    Ok(())
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
                },
            ),
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
        "left navigation must not render page icon-like accent square marker",
    );
}
