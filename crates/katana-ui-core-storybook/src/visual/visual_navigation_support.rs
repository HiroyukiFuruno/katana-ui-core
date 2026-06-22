use super::{Canvas, layout_metrics};
use crate::catalog::story_map::StoryGroup;
use crate::visual::navigation_tree::{
    NavigationRow, TreeExpansionState, row_from_click, visible_rows,
};
use crate::visual::render;

const NAV_SAMPLE_ROW_COUNT: usize = 3;
const NAV_EXPANDABLE_SAMPLE_ROW_COUNT: usize = 2;
const NAV_DEPTH_GROUP: usize = 0;
const NAV_DEPTH_SECTION: usize = 1;
const NAV_DEPTH_PAGE: usize = 2;
const NAV_GROUP_LINE_X: usize = 54;
const NAV_SECTION_LINE_X: usize = 68;
const NAV_PAGE_LINE_X: usize = 84;
const NAV_GROUP_LABEL_X: usize = 62;
const NAV_SECTION_LABEL_X: usize = 78;
const NAV_PAGE_LABEL_X: usize = 98;
const NAV_GROUP_LABEL_SAMPLE_WIDTH: usize = 140;
const NAV_SECTION_LABEL_SAMPLE_WIDTH: usize = 140;
const NAV_PAGE_LABEL_SAMPLE_WIDTH: usize = 160;
const NAV_DISCLOSURE_INSET: usize = 4;
const NAV_DISCLOSURE_SIZE: usize = 7;

pub(super) fn navigation_sample_rows(
    expansion: TreeExpansionState,
) -> Option<[(usize, usize); NAV_SAMPLE_ROW_COUNT]> {
    Some([
        (
            NAV_DEPTH_GROUP,
            navigation_row_y_for_group(expansion, StoryGroup::Foundation)?,
        ),
        (NAV_DEPTH_SECTION, navigation_row_y_for_section(expansion)?),
        (
            NAV_DEPTH_PAGE,
            navigation_row_y_for_section_page(expansion)?,
        ),
    ])
}

pub(super) fn navigation_expandable_sample_rows(
    expansion: TreeExpansionState,
) -> Option<[(usize, usize); NAV_EXPANDABLE_SAMPLE_ROW_COUNT]> {
    Some([
        (
            NAV_DEPTH_GROUP,
            navigation_row_y_for_group(expansion, StoryGroup::Foundation)?,
        ),
        (NAV_DEPTH_SECTION, navigation_row_y_for_section(expansion)?),
    ])
}

pub(super) fn navigation_line_x(depth: usize) -> usize {
    match depth {
        NAV_DEPTH_GROUP => NAV_GROUP_LINE_X,
        NAV_DEPTH_SECTION => NAV_SECTION_LINE_X,
        NAV_DEPTH_PAGE => NAV_PAGE_LINE_X,
        _ => NAV_PAGE_LINE_X,
    }
}

pub(super) fn require_navigation_value<T>(value: Option<T>, message: &str) -> Result<T, String> {
    value.ok_or_else(|| message.to_string())
}

pub(super) fn row_y_and_depth_in_navigation(
    page: &str,
    expansion: TreeExpansionState,
) -> Option<(usize, usize)> {
    for y in layout_metrics::NAV_FIRST_ROW_Y..layout_metrics::CONTENT_HEIGHT {
        let Some(row) = row_from_click(layout_metrics::NAV_ROW_X + 1, y, expansion) else {
            continue;
        };
        let depth = match row {
            NavigationRow::Group { .. } => NAV_DEPTH_GROUP,
            NavigationRow::Section { .. } => NAV_DEPTH_SECTION,
            NavigationRow::Page { page: row_page, .. } if row_page == page => NAV_DEPTH_PAGE,
            NavigationRow::PageWithoutSection { page: row_page, .. } if row_page == page => {
                NAV_DEPTH_SECTION
            }
            _ => continue,
        };
        return Some((y, depth));
    }
    None
}

pub(super) fn navigation_row_y_for_group(
    expansion: TreeExpansionState,
    group: StoryGroup,
) -> Option<usize> {
    visible_rows(expansion)
        .iter()
        .position(|row| matches!(row, NavigationRow::Group(found_group) if *found_group == group))
        .map(|index| layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP)
}

pub(super) fn navigation_row_y_for_section(expansion: TreeExpansionState) -> Option<usize> {
    visible_rows(expansion)
        .iter()
        .position(|row| matches!(row, NavigationRow::Section { .. }))
        .map(|index| layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP)
}

pub(super) fn navigation_row_y_for_section_page(expansion: TreeExpansionState) -> Option<usize> {
    visible_rows(expansion)
        .iter()
        .position(|row| matches!(row, NavigationRow::Page { .. }))
        .map(|index| layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP)
}

pub(super) fn navigation_next_page_row_y_after_page(
    expansion: TreeExpansionState,
    page: &str,
) -> Option<usize> {
    let rows = visible_rows(expansion);
    let current = rows.iter().position(|row| {
        matches!(
            row,
            NavigationRow::Page { page: row_page, .. }
                | NavigationRow::PageWithoutSection { page: row_page, .. }
                if *row_page == page
        )
    })?;
    rows.iter()
        .enumerate()
        .skip(current + 1)
        .find(|(_, row)| {
            matches!(
                row,
                NavigationRow::Page { .. } | NavigationRow::PageWithoutSection { .. },
            )
        })
        .map(|(index, _)| layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP)
}

pub(super) fn navigation_row_y_and_depth_for_page(
    expansion: TreeExpansionState,
    page: &str,
) -> Option<(usize, usize)> {
    visible_rows(expansion)
        .iter()
        .position(|row| {
            matches!(
                row,
                NavigationRow::Page { page: row_page, .. }
                    | NavigationRow::PageWithoutSection { page: row_page, .. }
                    if *row_page == page
            )
        })
        .and_then(|index| {
            let row = visible_rows(expansion).get(index).copied()?;
            let depth = match row {
                NavigationRow::Page { .. } => NAV_DEPTH_PAGE,
                NavigationRow::PageWithoutSection { .. } => NAV_DEPTH_SECTION,
                _ => return None,
            };
            Some((
                layout_metrics::NAV_FIRST_ROW_Y + index * layout_metrics::NAV_ROW_STEP,
                depth,
            ))
        })
}

pub(super) fn navigation_label_x(depth: usize) -> usize {
    match depth {
        NAV_DEPTH_GROUP => NAV_GROUP_LABEL_X,
        NAV_DEPTH_SECTION => NAV_SECTION_LABEL_X,
        NAV_DEPTH_PAGE => NAV_PAGE_LABEL_X,
        _ => NAV_PAGE_LABEL_X,
    }
}

pub(super) fn navigation_connector_target_x(depth: usize) -> usize {
    navigation_label_x(depth).saturating_sub(1)
}

pub(super) fn navigation_text_connector_sample_x(depth: usize) -> usize {
    navigation_label_x(depth).saturating_sub(2)
}

pub(super) fn navigation_horizontal_connector_sample_x(depth: usize) -> usize {
    navigation_line_x(depth) + 1
}

pub(super) fn navigation_label_sample_width(depth: usize) -> usize {
    match depth {
        NAV_DEPTH_GROUP => NAV_GROUP_LABEL_SAMPLE_WIDTH,
        NAV_DEPTH_SECTION => NAV_SECTION_LABEL_SAMPLE_WIDTH,
        NAV_DEPTH_PAGE => NAV_PAGE_LABEL_SAMPLE_WIDTH,
        _ => NAV_PAGE_LABEL_SAMPLE_WIDTH,
    }
}

pub(super) fn navigation_disclosure_center_x(depth: usize) -> usize {
    navigation_disclosure_left_x(depth) + navigation_disclosure_center_offset()
}

fn navigation_disclosure_left_x(depth: usize) -> usize {
    navigation_line_x(depth).saturating_sub(NAV_DISCLOSURE_INSET)
}

fn navigation_disclosure_center_offset() -> usize {
    navigation_disclosure_size() / 2
}

fn navigation_disclosure_size() -> usize {
    NAV_DISCLOSURE_SIZE
}

pub(super) fn navigation_disclosure_center_y(row_y: usize) -> usize {
    row_y + layout_metrics::NAV_ROW_HEIGHT / 2
}

pub(super) fn render_navigation_canvas(
    show_navigation_lines: bool,
    show_navigation_text_connectors: bool,
    selected_page: &'static str,
) -> Canvas {
    render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: "dark",
        selected_page,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: 0,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: crate::visual::panel_scroll_state::PanelScrollOffsets::default(),
        tree_expansion: TreeExpansionState::default(),
        show_navigation_lines,
        show_navigation_text_connectors,
        screen_state: crate::visual::screen_state::StorybookScreenState::default(),
    })
}

pub(super) fn assert_vertical_tree_segment(
    canvas: &Canvas,
    x: usize,
    from_y: usize,
    to_y: usize,
    color: u32,
) {
    assert!(
        from_y <= to_y,
        "vertical tree segment range should be non-empty",
    );
    for y in from_y..=to_y {
        assert_eq!(
            Some(color),
            pixel_at(canvas, x, y),
            "vertical tree segment should continue at ({x}, {y})",
        );
    }
}

pub(super) fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}
