use super::{StoryExample, molecules};

const FORM_SELECTION_PAGES: &[&str] = &[
    "form-field",
    "search-box",
    "segmented-toggle",
    "select-box",
    "combo-box",
    "menu-button",
    "dynamic-array-editor",
    "selection-list",
    "closeable-tab-strip",
    "search-control-strip",
];

pub(super) fn examples() -> Vec<StoryExample> {
    molecules::examples()
        .into_iter()
        .filter(|it| FORM_SELECTION_PAGES.contains(&it.page))
        .collect()
}
