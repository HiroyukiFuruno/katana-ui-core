use super::coverage_legacy_preview::legacy_preview_signature_stats;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use super::{
    inspector_rows, layout_metrics, navigation_tree, palette, panel_scroll_state, panel_scrollbars,
    preview_contract_rows, preview_detail, render, scrollbar,
};
use crate::DEFAULT_STORYBOOK_PAGE;
use crate::catalog::StoryExample;

const MIN_NAV_COLLAPSE_DIFF: usize = 1_000;
const LEGACY_DETAIL_TABLE_Y: usize = 398;
const LEGACY_DETAIL_TABLE_SAMPLE_OFFSET: usize = 10;
const PANEL_PREVIEW_SLOT_X: usize = 174;
const PANEL_PREVIEW_SLOT_Y: usize = 64;
const PANEL_PREVIEW_SLOT_WIDTH: usize = 296;
const PANEL_PREVIEW_SLOT_HEIGHT: usize = 192;
const PANEL_CHILD_BAR_THICKNESS: usize = 5;
const PANEL_CHILD_HORIZONTAL_TRACK_X_OFFSET: usize = 10;
const PANEL_CHILD_TRACK_EDGE_OFFSET: usize = 12;
const PANEL_CHILD_HORIZONTAL_TRACK_WIDTH_INSET: usize = 24;
const PANEL_CHILD_VERTICAL_TRACK_Y_OFFSET: usize = 8;
const PANEL_CHILD_VERTICAL_TRACK_HEIGHT_INSET: usize = 16;
pub(super) struct CoverageMarkers {
    pub(super) selected_preview_visible: bool,
    pub(super) selected_preview_interaction_visible: bool,
    pub(super) detail_tables_hidden: bool,
    pub(super) scrollbar_thumb_bottom: bool,
    pub(super) contract_rows_fit: bool,
    pub(super) inspector_rows_fit: bool,
    pub(super) tree_view_selected: bool,
    pub(super) tree_view_settings_visible: bool,
    pub(super) tree_view_line_option_visible: bool,
    pub(super) tree_view_node_marker_option_visible: bool,
    pub(super) tree_view_trigger_option_visible: bool,
    pub(super) tree_view_action_logged: bool,
    pub(super) panel_scrollbars_visible: bool,
    pub(super) navigation_collapsed_pixels_changed: usize,
    pub(super) legacy_preview_signatures: usize,
    pub(super) legacy_preview_signature_collisions: usize,
}

pub(super) fn build(examples: &[StoryExample]) -> CoverageMarkers {
    let tree_view = examples.iter().find(|it| it.page == "tree-view");
    let legacy_preview = legacy_preview_signature_stats();
    CoverageMarkers {
        selected_preview_visible: selected_preview_visible(),
        selected_preview_interaction_visible: selected_preview_interaction_visible(),
        detail_tables_hidden: detail_tables_hidden(),
        scrollbar_thumb_bottom: scrollbar_thumb_bottom(),
        contract_rows_fit: preview_contract_rows::rows_fit(examples),
        inspector_rows_fit: inspector_rows::rows_fit(examples),
        tree_view_selected: tree_view.is_some(),
        tree_view_settings_visible: tree_view_option_visible(tree_view, "context menu: enabled"),
        tree_view_line_option_visible: tree_view_option_visible(
            tree_view,
            "line: solid 1px enabled",
        ),
        tree_view_node_marker_option_visible: tree_view_option_visible(
            tree_view,
            "node markers: branch/leaf visible",
        ),
        tree_view_trigger_option_visible: tree_view_option_visible(
            tree_view,
            "trigger: icon+text chevron",
        ),
        tree_view_action_logged: tree_view.is_some_and(|it| !it.callback_logs.is_empty()),
        panel_scrollbars_visible: panel_scrollbars_visible(),
        navigation_collapsed_pixels_changed: navigation_collapsed_pixels_changed(),
        legacy_preview_signatures: legacy_preview.signatures,
        legacy_preview_signature_collisions: legacy_preview.collisions,
    }
}

fn panel_scrollbars_visible() -> bool {
    let canvas = render::render_storybook_canvas_for("dark", "panel", false);
    let accent =
        palette::VisualPalette::from_theme(&katana_ui_core::theme::ThemeSnapshot::dark()).accent;
    panel_scrollbar_center_is_accent(
        &canvas,
        accent,
        panel_scroll_state::PanelScrollRegion::Navigation,
    ) && panel_scrollbar_center_is_accent(
        &canvas,
        accent,
        panel_scroll_state::PanelScrollRegion::Inspector,
    ) && internal_panel_preview_scrollbar_visible(&canvas, accent, false)
        && internal_panel_preview_scrollbar_visible(&canvas, accent, true)
}

fn panel_scrollbar_center_is_accent(
    canvas: &super::Canvas,
    accent: u32,
    region: panel_scroll_state::PanelScrollRegion,
) -> bool {
    let thumb = panel_scrollbars::thumb_rect_for_state(
        region,
        panel_scroll_state::PanelScrollOffsets::default(),
        "panel",
        Default::default(),
    );
    pixel_at(
        canvas,
        thumb.x + thumb.width / 2,
        thumb.y + thumb.height / 2,
    ) == Some(accent)
}

fn internal_panel_preview_scrollbar_visible(
    canvas: &super::Canvas,
    accent: u32,
    horizontal: bool,
) -> bool {
    let component = preview_detail::component_action_hit_rect("panel");
    let rect = if horizontal {
        layout_metrics::LayoutRect::new(
            component.x + PANEL_PREVIEW_SLOT_X + PANEL_CHILD_HORIZONTAL_TRACK_X_OFFSET,
            component.y + PANEL_PREVIEW_SLOT_Y + PANEL_PREVIEW_SLOT_HEIGHT
                - PANEL_CHILD_TRACK_EDGE_OFFSET,
            PANEL_PREVIEW_SLOT_WIDTH - PANEL_CHILD_HORIZONTAL_TRACK_WIDTH_INSET,
            PANEL_CHILD_BAR_THICKNESS,
        )
    } else {
        layout_metrics::LayoutRect::new(
            component.x + PANEL_PREVIEW_SLOT_X + PANEL_PREVIEW_SLOT_WIDTH
                - PANEL_CHILD_TRACK_EDGE_OFFSET,
            component.y + PANEL_PREVIEW_SLOT_Y + PANEL_CHILD_VERTICAL_TRACK_Y_OFFSET,
            PANEL_CHILD_BAR_THICKNESS,
            PANEL_PREVIEW_SLOT_HEIGHT - PANEL_CHILD_VERTICAL_TRACK_HEIGHT_INSET,
        )
    };
    color_count(canvas, rect, accent) > 0
}

fn color_count(canvas: &super::Canvas, rect: layout_metrics::LayoutRect, color: u32) -> usize {
    let mut count = 0;
    for current_y in rect.y..rect.bottom() {
        for current_x in rect.x..rect.right() {
            if pixel_at(canvas, current_x, current_y) == Some(color) {
                count += 1;
            }
        }
    }
    count
}

fn pixel_at(canvas: &super::Canvas, x: usize, y: usize) -> Option<u32> {
    canvas.pixels().get(y * canvas.width() + x).copied()
}

fn selected_preview_visible() -> bool {
    visible_in_initial_viewport(preview_detail::selected_hero_y())
}

fn selected_preview_interaction_visible() -> bool {
    visible_in_initial_viewport(
        preview_detail::component_action_hit_rect(DEFAULT_STORYBOOK_PAGE).bottom(),
    )
}

fn visible_in_initial_viewport(content_y: usize) -> bool {
    content_y < render::VIEWPORT_HEIGHT
}

fn detail_tables_hidden() -> bool {
    let canvas = render::render_storybook_canvas_for("dark", DEFAULT_STORYBOOK_PAGE, false);
    let palette = palette::VisualPalette::from_theme(&katana_ui_core::theme::ThemeSnapshot::dark());
    let sample = pixel_at(
        &canvas,
        layout_metrics::PREVIEW_X + LEGACY_DETAIL_TABLE_SAMPLE_OFFSET,
        LEGACY_DETAIL_TABLE_Y + LEGACY_DETAIL_TABLE_SAMPLE_OFFSET,
    );
    sample != Some(palette.code_background) && sample != Some(palette.accent)
}

fn scrollbar_thumb_bottom() -> bool {
    let track = scrollbar::track_rect();
    let thumb = scrollbar::thumb_rect(super::layout_metrics::MAX_SCROLL_Y);
    thumb.bottom() == track.bottom()
}

fn tree_view_option_visible(tree_view: Option<&StoryExample>, expected: &str) -> bool {
    let screen_state = StorybookScreenState::default();
    let scenario = ScenarioContext {
        selected_page: "tree-view",
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: 0,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: &screen_state,
    };
    tree_view.is_some_and(|example| {
        inspector_rows::settings_rows(example.tree.root(), example, scenario)
            .iter()
            .any(|it| it == expected)
    })
}

fn navigation_collapsed_pixels_changed() -> usize {
    let open = render::render_storybook_canvas_for("dark", DEFAULT_STORYBOOK_PAGE, false);
    let mut collapsed = navigation_tree::TreeExpansionState::default();
    collapsed.toggle(crate::catalog::story_map::StoryGroup::Atoms);
    let closed = render::render_storybook_canvas_with_options(render::StorybookRenderOptions {
        theme_id: "dark",
        selected_page: DEFAULT_STORYBOOK_PAGE,
        selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
        preset_index: 0,
        preset_tab_scroll_x: 0,
        scroll_y: 0,
        scrollbar_visible: true,
        panel_scroll: panel_scroll_state::PanelScrollOffsets::default(),
        tree_expansion: collapsed,
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: StorybookScreenState::default(),
    });
    let diff = pixel_difference(open.pixels(), closed.pixels());
    navigation_diff_above_minimum(diff)
}

fn navigation_diff_above_minimum(diff: usize) -> usize {
    if diff > MIN_NAV_COLLAPSE_DIFF {
        diff
    } else {
        0
    }
}

fn pixel_difference(left: &[u32], right: &[u32]) -> usize {
    left.iter()
        .zip(right.iter())
        .filter(|(left, right)| left != right)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{MIN_NAV_COLLAPSE_DIFF, navigation_diff_above_minimum};

    #[test]
    fn navigation_diff_requires_more_than_the_minimum_pixel_count() {
        assert_eq!(0, navigation_diff_above_minimum(MIN_NAV_COLLAPSE_DIFF));
        assert_eq!(
            MIN_NAV_COLLAPSE_DIFF + 1,
            navigation_diff_above_minimum(MIN_NAV_COLLAPSE_DIFF + 1)
        );
    }
}
