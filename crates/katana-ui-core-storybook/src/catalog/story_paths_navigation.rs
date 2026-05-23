use super::story_hierarchy::{StoryGroup, StoryPath};

pub(crate) const PATHS: &[StoryPath] = &[
    StoryPath {
        page: "menu",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
    StoryPath {
        page: "command-palette",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
    StoryPath {
        page: "breadcrumb",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
    StoryPath {
        page: "side-menu",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
    StoryPath {
        page: "tabs",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
    StoryPath {
        page: "status-bar",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
    StoryPath {
        page: "toolbar",
        group: StoryGroup::Navigation,
        section: None,
        path: "navigation/mod.rs",
    },
];
