use super::visual_navigation_support::{
    assert_vertical_tree_segment, navigation_expandable_sample_rows,
    navigation_horizontal_connector_sample_x, navigation_line_x,
    navigation_next_page_row_y_after_page, navigation_row_y_and_depth_for_page,
    navigation_row_y_for_group, navigation_row_y_for_section, navigation_row_y_for_section_page,
    navigation_sample_rows, navigation_text_connector_sample_x, render_navigation_canvas,
    row_y_and_depth_in_navigation,
};
use super::{StorybookVisual, layout_metrics, palette};
use crate::catalog::story_map::StoryGroup;
use crate::visual::navigation_tree::TreeExpansionState;
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn navigation_tree_lines_obey_show_navigation_lines_option() {
    let expansion = TreeExpansionState::default();
    let with_lines = render_navigation_canvas(true, false, "button");
    let without_lines = render_navigation_canvas(false, true, "button");
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());

    let (row_y, row_depth) = row_y_and_depth_in_navigation("tree-view", expansion)
        .expect("target row should be visible");
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
}

#[test]
fn navigation_text_connectors_are_hidden_by_default() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = StorybookVisual.render_scenario("dark", "tree-view", false);
    for (depth, row_y) in navigation_sample_rows(expansion) {
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
}

#[test]
fn navigation_text_connectors_extend_to_label_when_enabled() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, true, "tree-view");
    for (depth, row_y) in navigation_expandable_sample_rows(expansion) {
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
}

#[test]
fn navigation_expandable_rows_draw_horizontal_elbow() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, false, "tree-view");

    for (depth, row_y) in navigation_expandable_sample_rows(expansion) {
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
}

#[test]
fn navigation_leaf_page_rows_do_not_draw_horizontal_elbow() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, false, "button");

    for page in ["button", "theme-tokens"] {
        let (row_y, depth) = navigation_row_y_and_depth_for_page(expansion, page)
            .expect("page row should be visible");
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
}

#[test]
fn navigation_text_connectors_skip_leaf_page_rows_when_enabled() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, true, "button");

    for page in ["button", "theme-tokens"] {
        let (row_y, depth) = navigation_row_y_and_depth_for_page(expansion, page)
            .expect("page row should be visible");
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
}

#[test]
fn navigation_tree_lines_remain_continuous_without_text_connectors() {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, false, "button");
    let foundation_row_y = navigation_row_y_for_group(expansion, StoryGroup::Foundation)
        .expect("foundation group row should be visible");
    let section_row_y =
        navigation_row_y_for_section(expansion).expect("section row should be visible");
    let first_page_row_y =
        navigation_row_y_for_section_page(expansion).expect("section page row should be visible");
    let (button_row_y, button_depth) = navigation_row_y_and_depth_for_page(expansion, "button")
        .expect("button row should be visible");
    let next_button_sibling_row_y = navigation_next_page_row_y_after_page(expansion, "button")
        .expect("button sibling row should be visible");
    let line_x = navigation_line_x(button_depth);
    let selected_row_center_y = button_row_y + layout_metrics::NAV_ROW_HEIGHT / 2;
    let selected_row_top_y = button_row_y + 1;
    let sibling_gap_y = button_row_y + layout_metrics::NAV_ROW_HEIGHT + 1;

    assert_vertical_tree_segment(
        &canvas,
        navigation_line_x(0),
        super::visual_navigation_support::navigation_disclosure_center_y(foundation_row_y) + 1,
        section_row_y.saturating_sub(1),
        palette.border,
    );
    assert_vertical_tree_segment(
        &canvas,
        navigation_line_x(1),
        super::visual_navigation_support::navigation_disclosure_center_y(section_row_y) + 1,
        first_page_row_y.saturating_sub(1),
        palette.border,
    );
    assert_vertical_tree_segment(
        &canvas,
        line_x,
        selected_row_center_y + 1,
        next_button_sibling_row_y.saturating_sub(1),
        palette.border,
    );
    assert_eq!(
        Some(palette.border),
        super::visual_navigation_support::pixel_at(&canvas, line_x, selected_row_top_y),
    );
    assert_eq!(
        Some(palette.border),
        super::visual_navigation_support::pixel_at(&canvas, line_x, selected_row_center_y),
    );
    assert_eq!(
        Some(palette.border),
        super::visual_navigation_support::pixel_at(&canvas, line_x, sibling_gap_y),
    );
    assert_ne!(
        Some(palette.border),
        super::visual_navigation_support::pixel_at(
            &canvas,
            navigation_text_connector_sample_x(button_depth),
            selected_row_center_y,
        ),
    );
}
