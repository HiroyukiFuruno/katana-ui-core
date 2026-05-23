#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum StoryGroup {
    Foundation,
    Atoms,
    Forms,
    Layout,
    Navigation,
    Feedback,
    Data,
    Runtime,
}

impl StoryGroup {
    pub(crate) const COUNT: usize = 8;
    pub(crate) const ALL: &[Self] = &[
        Self::Foundation,
        Self::Atoms,
        Self::Forms,
        Self::Layout,
        Self::Navigation,
        Self::Feedback,
        Self::Data,
        Self::Runtime,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Foundation => "Foundation",
            Self::Atoms => "Atoms",
            Self::Forms => "Forms",
            Self::Layout => "Layout",
            Self::Navigation => "Navigation",
            Self::Feedback => "Feedback",
            Self::Data => "Data",
            Self::Runtime => "Runtime",
        }
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum StorySection {
    Theme,
    Selection,
    Overlay,
    Structured,
    AppPrimitives,
}

impl StorySection {
    pub(crate) const COUNT: usize = 5;
    pub(crate) const ALL: &[Self] = &[
        Self::Theme,
        Self::Selection,
        Self::Overlay,
        Self::Structured,
        Self::AppPrimitives,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Theme => "Theme",
            Self::Selection => "Selection",
            Self::Overlay => "Overlay",
            Self::Structured => "Structured",
            Self::AppPrimitives => "App primitives",
        }
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StoryPath {
    pub(crate) page: &'static str,
    pub(crate) group: StoryGroup,
    pub(crate) section: Option<StorySection>,
    pub(crate) path: &'static str,
}

pub(crate) const STORY_GROUPS: &[StoryGroup] = StoryGroup::ALL;
pub(crate) const STORY_PATH_GROUPS: &[&[StoryPath]] = &[
    super::story_paths_foundation::PATHS,
    super::story_paths_atoms::PATHS,
    super::story_paths_forms::PATHS,
    super::story_paths_layout::PATHS,
    super::story_paths_navigation::PATHS,
    super::story_paths_feedback::PATHS,
    super::story_paths_data::PATHS,
    super::story_paths_runtime::PATHS,
];

impl StoryPath {
    fn for_page(page: &str) -> Option<&'static StoryPath> {
        STORY_PATH_GROUPS
            .iter()
            .flat_map(|paths| paths.iter())
            .find(|entry| entry.page == page)
    }

    pub(crate) fn path_for_page(page: &str) -> Option<&'static str> {
        Self::for_page(page).map(|entry| entry.path)
    }
}
