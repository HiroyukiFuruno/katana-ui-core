use super::{
    ROW_MAX_CHARS, ScenarioContext, StoryExample, history_rows, quality_rows, settings_rows,
    state_rows,
};
use crate::DEFAULT_STORYBOOK_PAGE;
use crate::visual::screen_state::StorybookScreenState;

pub(super) fn rows_fit(examples: &[StoryExample]) -> bool {
    let screen_state = StorybookScreenState::default();
    let scenario = ScenarioContext {
        selected_page: DEFAULT_STORYBOOK_PAGE,
        preset_index: 0,
        preset_tab_scroll_x: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        show_navigation_lines: true,
        show_navigation_text_connectors: false,
        screen_state: &screen_state,
    };
    examples.iter().all(|example| {
        let node = example.tree.root();
        settings_rows(node, example, scenario)
            .iter()
            .chain(state_rows(node, scenario).iter())
            .chain(history_rows(example, scenario).iter())
            .chain(quality_rows(scenario).iter())
            .all(|value| value.chars().count() <= ROW_MAX_CHARS)
    })
}
