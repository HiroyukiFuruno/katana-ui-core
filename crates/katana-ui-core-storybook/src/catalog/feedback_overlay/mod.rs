use super::{StoryExample, molecules};

const FEEDBACK_PAGES: &[&str] = &[
    "context-menu",
    "tooltip",
    "modal",
    "popover",
    "modal-overlay",
    "notification-toast",
    "hover-card",
    "toast-stack-manager",
    "banner",
    "collapsible-panel",
];

pub(super) fn examples() -> Vec<StoryExample> {
    molecules::examples()
        .into_iter()
        .filter(|it| FEEDBACK_PAGES.contains(&it.page))
        .collect()
}
