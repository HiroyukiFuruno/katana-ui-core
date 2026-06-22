use super::inspector_rows;
use super::render_context::ScenarioContext;
use super::screen_state::StorybookScreenState;
use crate::StoryCatalog;
use crate::test_assert::KucTestExpect;

const PAGE: &str = "progress-bar";

#[test]
fn progress_bar_inspector_shows_reduced_motion_before_after_without_clipping() {
    let examples = StoryCatalog.examples();
    let example = examples
        .iter()
        .find(|example| example.page == PAGE)
        .kuc_expect("progress-bar story example must exist");
    let screen_state = StorybookScreenState::default();
    let rows = inspector_rows::settings_rows(
        example.tree.root(),
        example,
        ScenarioContext {
            selected_page: PAGE,
            selected_instance_id: crate::visual::window_interaction::DEFAULT_INSTANCE_ID,
            preset_index: 0,
            preset_tab_scroll_x: 0,
            tree_expansion: Default::default(),
            scrollbar_visible: true,
            panel_scroll: Default::default(),
            screen_state: &screen_state,
            show_navigation_lines: true,
            show_navigation_text_connectors: false,
        },
    );

    assert!(
        rows.iter()
            .any(|row| row == "loading.reduced_motion: false -> true"),
        "progress-bar Inspector must show complete reduced_motion before/after row: {rows:?}"
    );
}
