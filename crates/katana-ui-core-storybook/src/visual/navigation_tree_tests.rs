#[cfg(test)]
use super::{NavigationRow, TreeExpansionState, visible_rows};
use crate::catalog::story_map::{
    STORY_GROUPS, STORY_PATH_GROUPS, StoryGroup, StoryPath, StorySection,
};
use crate::requirements::StoryRequirements;

#[cfg(test)]
#[test]
fn all_required_pages_are_registered_once() {
    let rows = visible_rows(TreeExpansionState::default());
    for page in StoryRequirements::required_pages() {
        let path = StoryPath::path_for_page(page).unwrap_or_default();

        let count_in_hierarchy = rows
            .iter()
            .filter(|row| {
                matches!(
                    row,
                    NavigationRow::Page { page: found, .. }
                        | NavigationRow::PageWithoutSection { page: found, .. }
                    if *found == *page
                )
            })
            .count();
        assert_eq!(
            1, count_in_hierarchy,
            "{page} must appear once in hierarchy pages"
        );

        let count_in_raw = STORY_PATH_GROUPS
            .iter()
            .flat_map(|entry| entry.iter())
            .filter(|entry| entry.page == *page)
            .count();
        assert_eq!(
            1, count_in_raw,
            "{page} must be registered once in raw paths"
        );

        assert!(
            !path.is_empty(),
            "story_path missing path entry for required page: {page}"
        );
    }

    for row in rows.iter() {
        if let NavigationRow::Page { page, .. } | NavigationRow::PageWithoutSection { page, .. } =
            row
        {
            assert!(
                !page.contains('/'),
                "pseudo-hierarchy by slash not allowed: {page}"
            );
        }
    }
}

#[cfg(test)]
#[test]
fn tree_rows_include_groups_and_sections() {
    let rows = visible_rows(TreeExpansionState::default());
    for group in STORY_GROUPS {
        assert!(rows.iter().any(|row| {
            matches!(row, NavigationRow::Group(found_group) if *found_group == *group)
        }));
    }

    let forms_section = StorySection::Selection;
    assert!(rows.iter().any(|row| {
        matches!(row, NavigationRow::Section { group: StoryGroup::Forms, section: found_section } if *found_section == forms_section)
    }));
    let has_select_box = rows
        .iter()
        .position(|it| {
            matches!(
                it,
                NavigationRow::Page {
                    page: "select-box",
                    group: StoryGroup::Forms,
                    section: StorySection::Selection
                }
            )
        })
        .is_some();
    assert!(
        has_select_box,
        "select-box should be under Forms > Selection"
    );

    let has_theme_tokens = rows
        .iter()
        .position(|it| {
            matches!(
                it,
                NavigationRow::Page {
                    page: "theme-tokens",
                    group: StoryGroup::Foundation,
                    section: StorySection::Theme
                }
            )
        })
        .is_some();
    assert!(
        has_theme_tokens,
        "theme-tokens should be under Foundation > Theme"
    );

    let rows_str = rows
        .iter()
        .map(|row| match row {
            NavigationRow::Group { .. } => "group",
            NavigationRow::Section { .. } => "section",
            NavigationRow::Page { .. } => "page",
            NavigationRow::PageWithoutSection { .. } => "page-without-section",
        })
        .collect::<Vec<_>>();
    let has_two_level_sequence = rows_str
        .windows(3)
        .any(|it| it[0] == "group" && it[1] == "section" && it[2] == "page");
    assert!(
        has_two_level_sequence,
        "left menu should render at least one 2-level hierarchy row"
    );
}

#[cfg(test)]
#[test]
fn tree_rows_include_all_groups_and_can_collapse_group_and_section() {
    let mut expansion = TreeExpansionState::default();
    let before = visible_rows(expansion);

    expansion.toggle(StoryGroup::Atoms);
    let after_group = visible_rows(expansion);
    assert!(before.len() > after_group.len());
    assert!(
        !after_group
            .iter()
            .any(|it| matches!(it, NavigationRow::Page { page: "button", .. }))
    );

    expansion.toggle(StoryGroup::Atoms);
    expansion.toggle_section(StoryGroup::Forms, StorySection::Selection);
    let after_section = visible_rows(expansion);
    assert!(after_section.iter().any(|it| {
        matches!(
            it,
            NavigationRow::Section {
                group: StoryGroup::Forms,
                section: StorySection::Selection,
            }
        )
    }));
    assert!(after_section.len() < before.len());
    assert!(!after_section.iter().any(|it| {
        matches!(
            it,
            NavigationRow::Page {
                page: "select-box",
                group: StoryGroup::Forms,
                ..
            }
        )
    }));
}

#[cfg(test)]
#[test]
fn tree_rows_use_hierarchy_labels_without_slash() {
    for group in STORY_GROUPS {
        assert!(
            !group.label().contains('/'),
            "{group:?} has slash-style label"
        );
    }
    for section in StorySection::ALL {
        assert!(
            !section.label().contains('/'),
            "{section:?} has slash-style label"
        );
    }
}

#[cfg(test)]
#[test]
fn tree_rows_expand_section_state_is_scoped_to_subtree() {
    let rows_before = visible_rows(TreeExpansionState::default());
    let group_position = rows_before
        .iter()
        .position(|it| matches!(it, NavigationRow::Group(StoryGroup::Forms)));
    let section_position = rows_before.iter().position(|it| {
        matches!(
            it,
            NavigationRow::Section {
                group: StoryGroup::Forms,
                section: StorySection::Selection
            }
        )
    });
    assert!(group_position.is_some(), "Forms row should exist");
    assert!(section_position.is_some(), "Selection row should exist");
    if let (Some(group_position), Some(section_position)) = (group_position, section_position) {
        assert!(
            section_position > group_position,
            "Selection should be nested under Forms"
        );
    }

    let mut expansion = TreeExpansionState::default();
    expansion.toggle_section(StoryGroup::Forms, StorySection::Selection);
    let collapsed = visible_rows(expansion);
    assert!(!collapsed.iter().any(|it| {
        matches!(
            it,
            NavigationRow::Page {
                page: "select-box",
                group: StoryGroup::Forms,
                ..
            }
        )
    }));
}

#[cfg(test)]
#[test]
fn max_scroll_places_last_navigation_row_at_panel_bottom() {
    let expansion = TreeExpansionState::default();
    let max_scroll = super::max_scroll_y(expansion);

    assert_eq!(
        super::navigation_menu_panel_rect().bottom(),
        super::last_row_bottom_at_scroll(expansion, max_scroll)
    );
}

#[cfg(test)]
#[test]
fn section_expansion_state_is_scoped_by_group_and_section_pair() {
    let mut expansion = TreeExpansionState::default();

    assert!(expansion.is_section_open(StoryGroup::Forms, StorySection::Selection));
    assert!(expansion.is_section_open(StoryGroup::Atoms, StorySection::Selection));

    expansion.toggle_section(StoryGroup::Forms, StorySection::Selection);

    assert!(!expansion.is_section_open(StoryGroup::Forms, StorySection::Selection));
    assert!(expansion.is_section_open(StoryGroup::Atoms, StorySection::Selection));
}
