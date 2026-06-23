use super::visual_navigation_support::{
    assert_vertical_tree_segment, navigation_line_x, navigation_next_page_row_y_after_page,
    navigation_row_y_and_depth_for_page, navigation_row_y_for_group, navigation_row_y_for_section,
    navigation_row_y_for_section_page, navigation_text_connector_sample_x,
    render_navigation_canvas, require_navigation_value,
};
use super::{layout_metrics, palette};
use crate::catalog::story_map::StoryGroup;
use crate::visual::navigation_tree::TreeExpansionState;
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn navigation_tree_lines_remain_continuous_without_text_connectors() -> Result<(), String> {
    let expansion = TreeExpansionState::default();
    let palette = palette::VisualPalette::from_theme(&ThemeSnapshot::dark());
    let canvas = render_navigation_canvas(true, false, "button");
    let foundation_row_y = require_navigation_value(
        navigation_row_y_for_group(expansion, StoryGroup::Foundation),
        "foundation group row should be visible",
    )?;
    let section_row_y = require_navigation_value(
        navigation_row_y_for_section(expansion),
        "section row should be visible",
    )?;
    let first_page_row_y = require_navigation_value(
        navigation_row_y_for_section_page(expansion),
        "section page row should be visible",
    )?;
    let (button_row_y, button_depth) = require_navigation_value(
        navigation_row_y_and_depth_for_page(expansion, "button"),
        "button row should be visible",
    )?;
    let next_button_sibling_row_y = require_navigation_value(
        navigation_next_page_row_y_after_page(expansion, "button"),
        "button sibling row should be visible",
    )?;
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
    Ok(())
}
