use super::{StoryExample, molecules};

const RUNTIME_APP_PRIMITIVE_PAGES: &[&str] = &[
    "virtualization",
    "motion",
    "shortcut-combo",
    "shortcut-cheatsheet",
    "drag-and-drop",
    "window-control-button-group",
    "skeleton-cluster",
    "color-picker-rgba",
    "accordion",
    "startup-state-panel",
];

pub(super) fn examples() -> Vec<StoryExample> {
    molecules::examples()
        .into_iter()
        .filter(|it| RUNTIME_APP_PRIMITIVE_PAGES.contains(&it.page))
        .collect()
}
