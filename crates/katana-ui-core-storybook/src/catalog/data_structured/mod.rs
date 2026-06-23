use super::{StoryExample, molecules};

const DATA_STRUCTURED_PAGES: &[&str] = &[
    "card",
    "list",
    "tree-view",
    "diagnostics-list",
    "empty-state",
    "settings-list",
    "attachment-chip",
    "chip-group",
    "code-diff",
];

pub(super) fn examples() -> Vec<StoryExample> {
    molecules::examples()
        .into_iter()
        .filter(|it| DATA_STRUCTURED_PAGES.contains(&it.page))
        .collect()
}
