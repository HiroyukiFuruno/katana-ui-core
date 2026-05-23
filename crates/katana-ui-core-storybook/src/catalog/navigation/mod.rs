use super::{StoryExample, molecules};

const NAVIGATION_PAGES: &[&str] = &[
    "menu",
    "command-palette",
    "breadcrumb",
    "side-menu",
    "tabs",
    "status-bar",
    "toolbar",
];

pub(super) fn examples() -> Vec<StoryExample> {
    molecules::examples()
        .into_iter()
        .filter(|it| NAVIGATION_PAGES.contains(&it.page))
        .collect()
}
