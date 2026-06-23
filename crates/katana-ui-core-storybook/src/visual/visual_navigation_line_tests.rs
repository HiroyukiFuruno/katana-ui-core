use super::visual_navigation_support::{
    navigation_expandable_sample_rows, navigation_horizontal_connector_sample_x, navigation_line_x,
    navigation_row_y_and_depth_for_page, navigation_row_y_for_group, navigation_row_y_for_section,
    navigation_sample_rows, navigation_text_connector_sample_x, render_navigation_canvas,
    require_navigation_value, row_y_and_depth_in_navigation,
};
use super::{StorybookVisual, layout_metrics, palette};
use crate::catalog::story_map::StoryGroup;
use crate::visual::navigation_tree::TreeExpansionState;
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn navigation_tree_lines_obey_show_navigation_lines_option() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let with_lines = render_navigation_canvas(true, false, "button");
    let without_lines = render_navigation_canvas(false, true, "button");
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    let (row_y, row_depth) = require_navigation_value(
        row_y_and_depth_in_navigation("tree-view", expansion),
        "target row should be visible",
    )?;
    let line_x = navigation_line_x(row_depth);
    let text_connector_x = navigation_text_connector_sample_x(row_depth);
    let row_center_y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;
    let below_row_center_y = row_center_y + 2;

    assert_eq!(
        Some(palette.border),
        super::visual_navigation_support::pixel_at(&with_lines, line_x, row_center_y),
    );
    assert_eq!(
        Some(palette.code_background),
        super::visual_navigation_support::pixel_at(&without_lines, line_x, row_center_y),
    );
    assert_eq!(
        Some(palette.code_background),
        super::visual_navigation_support::pixel_at(&without_lines, line_x, below_row_center_y),
    );
    assert_ne!(
        super::visual_navigation_support::pixel_at(&without_lines, line_x, row_center_y),
        Some(palette.border),
    );
    assert_ne!(
        super::visual_navigation_support::pixel_at(&without_lines, text_connector_x, row_center_y),
        Some(palette.border),
    );
    Ok(())
}

#[test]
fn navigation_section_disclosure_is_indented_right_of_group_disclosure() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);

    let group_row_y = require_navigation_value(
        navigation_row_y_for_group(expansion, StoryGroup::Foundation),
        "group row should be present in default navigation expansion",
    )?;
    let section_row_y = require_navigation_value(
        navigation_row_y_for_section(expansion),
        "section row should be present in default navigation expansion",
    )?;
    let group_disclosure_x = super::visual_navigation_support::navigation_disclosure_center_x(0);
    let section_disclosure_x = super::visual_navigation_support::navigation_disclosure_center_x(1);
    let group_row_disclosure_y =
        super::visual_navigation_support::navigation_disclosure_center_y(group_row_y);
    let section_row_disclosure_y =
        super::visual_navigation_support::navigation_disclosure_center_y(section_row_y);

    assert!(section_disclosure_x > group_disclosure_x);
    assert_eq!(
        Some(palette.text),
        super::visual_navigation_support::pixel_at(
            &canvas,
            group_disclosure_x,
            group_row_disclosure_y
        ),
    );
    assert_eq!(
        Some(palette.text),
        super::visual_navigation_support::pixel_at(
            &canvas,
            section_disclosure_x,
            section_row_disclosure_y,
        ),
    );
    assert_ne!(
        Some(palette.text),
        super::visual_navigation_support::pixel_at(
            &canvas,
            group_disclosure_x,
            section_row_disclosure_y,
        ),
    );
    Ok(())
}

#[test]
fn navigation_text_connectors_are_hidden_by_default() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);
    for (depth, row_y) in require_navigation_value(
        navigation_sample_rows(expansion),
        "navigation sample rows should be visible",
    )? {
        let y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;
        assert_ne!(
            Some(palette.border),
            super::visual_navigation_support::pixel_at(
                &canvas,
                navigation_text_connector_sample_x(depth),
                y,
            ),
            "navigation text connector should be hidden by default at depth {depth}",
        );
    }
    Ok(())
}

#[test]
fn navigation_text_connectors_extend_to_label_when_enabled() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, true, "tree-view");
    for (depth, row_y) in require_navigation_value(
        navigation_expandable_sample_rows(expansion),
        "navigation expandable sample rows should be visible",
    )? {
        let y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;
        for x in navigation_line_x(depth)
            ..super::visual_navigation_support::navigation_connector_target_x(depth)
        {
            assert_eq!(
                Some(palette.border),
                super::visual_navigation_support::pixel_at(&canvas, x, y),
                "navigation text connector should continue at depth {depth} ({x}, {y})",
            );
        }
    }
    Ok(())
}

#[test]
fn navigation_expandable_rows_draw_horizontal_elbow() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, false, "tree-view");

    for (depth, row_y) in require_navigation_value(
        navigation_expandable_sample_rows(expansion),
        "navigation expandable sample rows should be visible",
    )? {
        let y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;

        assert_eq!(
            Some(palette.border),
            super::visual_navigation_support::pixel_at(
                &canvas,
                navigation_horizontal_connector_sample_x(depth),
                y,
            ),
            "expandable navigation row should draw horizontal elbow at depth {depth}",
        );
    }
    Ok(())
}

#[test]
fn navigation_leaf_page_rows_do_not_draw_horizontal_elbow() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, false, "button");

    for page in ["button", "theme-tokens"] {
        let (row_y, depth) = require_navigation_value(
            navigation_row_y_and_depth_for_page(expansion, page),
            "page row should be visible",
        )?;
        let y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;

        assert_ne!(
            Some(palette.border),
            super::visual_navigation_support::pixel_at(
                &canvas,
                navigation_horizontal_connector_sample_x(depth),
                y,
            ),
            "leaf navigation row must not draw a horizontal elbow for {page}",
        );
    }
    Ok(())
}

#[test]
fn navigation_text_connectors_skip_leaf_page_rows_when_enabled() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, true, "button");

    for page in ["button", "theme-tokens"] {
        let (row_y, depth) = require_navigation_value(
            navigation_row_y_and_depth_for_page(expansion, page),
            "page row should be visible",
        )?;
        let y = row_y + layout_metrics::NAV_ROW_HEIGHT / 2;

        assert_ne!(
            Some(palette.border),
            super::visual_navigation_support::pixel_at(
                &canvas,
                navigation_horizontal_connector_sample_x(depth),
                y,
            ),
            "leaf navigation row must not draw an elbow for {page} even when text connectors are enabled",
        );
        assert_ne!(
            Some(palette.border),
            super::visual_navigation_support::pixel_at(
                &canvas,
                navigation_text_connector_sample_x(depth),
                y,
            ),
            "leaf navigation row must not draw text connector for {page} when enabled",
        );
    }
    Ok(())
}
