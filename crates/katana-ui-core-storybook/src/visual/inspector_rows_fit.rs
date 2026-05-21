use super::{
    ROW_MAX_CHARS, ScenarioContext, StoryExample, history_rows, quality_rows, settings_rows,
    state_rows,
};

pub(super) fn rows_fit(examples: &[StoryExample]) -> bool {
    let scenario = ScenarioContext {
        selected_page: "button",
        preset_index: 0,
        tree_expansion: Default::default(),
        scrollbar_visible: true,
        panel_scroll: Default::default(),
        screen_state: Default::default(),
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
