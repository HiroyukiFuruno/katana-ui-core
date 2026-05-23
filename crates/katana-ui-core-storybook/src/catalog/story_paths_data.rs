use super::story_hierarchy::{StoryGroup, StoryPath, StorySection};

pub(crate) const PATHS: &[StoryPath] = &[
    StoryPath {
        page: "card",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "list",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "tree-view",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "diagnostics-list",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "empty-state",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "settings-list",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "attachment-chip",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "chip-group",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
    StoryPath {
        page: "code-diff",
        group: StoryGroup::Data,
        section: Some(StorySection::Structured),
        path: "data_structured/mod.rs",
    },
];
