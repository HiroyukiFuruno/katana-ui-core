use super::layout_metrics::{
    NAV_FIRST_ROW_Y, NAV_ROW_HEIGHT, NAV_ROW_STEP, navigation_hit_rect, navigation_menu_panel_rect,
};
use crate::catalog::story_map::{STORY_GROUPS, STORY_PATH_GROUPS, StoryGroup, StorySection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NavigationRow {
    Group(StoryGroup),
    Section {
        group: StoryGroup,
        section: StorySection,
    },
    Page {
        page: &'static str,
        group: StoryGroup,
        section: StorySection,
    },
    PageWithoutSection {
        page: &'static str,
        group: StoryGroup,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TreeExpansionState {
    groups: [bool; StoryGroup::COUNT],
    sections: [[bool; StorySection::COUNT]; StoryGroup::COUNT],
}

impl Default for TreeExpansionState {
    fn default() -> Self {
        Self {
            groups: [true; StoryGroup::COUNT],
            sections: [[true; StorySection::COUNT]; StoryGroup::COUNT],
        }
    }
}

impl TreeExpansionState {
    pub(super) fn is_open(self, group: StoryGroup) -> bool {
        self.groups[group.index()]
    }

    pub(super) fn is_section_open(self, group: StoryGroup, section: StorySection) -> bool {
        self.sections[group.index()][section.index()]
    }

    pub(super) fn toggle(&mut self, group: StoryGroup) {
        self.groups[group.index()] = !self.groups[group.index()];
    }

    pub(super) fn toggle_section(&mut self, group: StoryGroup, section: StorySection) {
        self.sections[group.index()][section.index()] =
            !self.sections[group.index()][section.index()];
    }
}

pub(super) fn visible_rows(expansion: TreeExpansionState) -> Vec<NavigationRow> {
    let mut rows = Vec::new();
    for group in STORY_GROUPS.iter().copied() {
        rows.push(NavigationRow::Group(group));
        if !expansion.is_open(group) {
            continue;
        }
        let section_opened = section_opened_for_group(group);
        append_sectionless_pages(group, &mut rows);
        append_section_rows(group, expansion, &section_opened, &mut rows);
    }
    rows
}

fn append_section_rows(
    group: StoryGroup,
    expansion: TreeExpansionState,
    section_opened: &[bool; StorySection::COUNT],
    rows: &mut Vec<NavigationRow>,
) {
    for section in StorySection::ALL.iter().copied() {
        if !section_opened[section.index()] {
            continue;
        }
        rows.push(NavigationRow::Section { group, section });
        if !expansion.is_section_open(group, section) {
            continue;
        }
        append_section_pages(group, section, rows);
    }
}

fn append_section_pages(group: StoryGroup, section: StorySection, rows: &mut Vec<NavigationRow>) {
    for paths in STORY_PATH_GROUPS.iter() {
        for path in *paths {
            if path.group != group || path.section != Some(section) {
                continue;
            }
            rows.push(NavigationRow::Page {
                page: path.page,
                group,
                section,
            });
        }
    }
}

fn append_sectionless_pages(group: StoryGroup, rows: &mut Vec<NavigationRow>) {
    for paths in STORY_PATH_GROUPS.iter() {
        for path in *paths {
            if path.group != group || path.section.is_some() {
                continue;
            }
            rows.push(NavigationRow::PageWithoutSection {
                page: path.page,
                group,
            });
        }
    }
}

fn section_opened_for_group(group: StoryGroup) -> [bool; StorySection::COUNT] {
    let mut section_opened = [false; StorySection::COUNT];
    for paths in STORY_PATH_GROUPS.iter() {
        for path in *paths {
            let Some(section) = path.section else {
                continue;
            };
            if path.group != group {
                continue;
            }
            section_opened[section.index()] = true;
        }
    }
    section_opened
}

pub(super) fn row_from_click(
    x: usize,
    y: usize,
    expansion: TreeExpansionState,
) -> Option<NavigationRow> {
    let mut row_y = NAV_FIRST_ROW_Y;
    for row in visible_rows(expansion) {
        if navigation_hit_rect(row_y).contains(x, y) {
            return Some(row);
        }
        row_y += NAV_ROW_STEP;
    }
    None
}

pub(super) fn max_scroll_y(expansion: TreeExpansionState) -> usize {
    navigation_content_height(expansion).saturating_sub(navigation_viewport_height())
}

#[cfg(test)]
pub(super) fn last_row_bottom_at_scroll(expansion: TreeExpansionState, scroll_y: usize) -> usize {
    let row_count = visible_rows(expansion).len();
    if row_count == 0 {
        return NAV_FIRST_ROW_Y;
    }
    NAV_FIRST_ROW_Y + (row_count - 1) * NAV_ROW_STEP + NAV_ROW_HEIGHT - scroll_y
}

fn navigation_content_height(expansion: TreeExpansionState) -> usize {
    let row_count = visible_rows(expansion).len();
    if row_count == 0 {
        return 0;
    }
    (row_count - 1) * NAV_ROW_STEP + NAV_ROW_HEIGHT
}

fn navigation_viewport_height() -> usize {
    navigation_menu_panel_rect()
        .bottom()
        .saturating_sub(NAV_FIRST_ROW_Y)
}

#[cfg(test)]
#[path = "navigation_tree_tests.rs"]
mod navigation_tree_tests;
