use super::story_hierarchy::{StoryGroup, StoryPath, StorySection};

pub(crate) const PATHS: &[StoryPath] = &[
    StoryPath {
        page: "panel",
        group: StoryGroup::Foundation,
        section: Some(StorySection::Theme),
        path: "foundation_theme",
    },
    StoryPath {
        page: "theme-tokens",
        group: StoryGroup::Foundation,
        section: Some(StorySection::Theme),
        path: "foundation_theme/mod.rs",
    },
];
