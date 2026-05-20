use super::layout_metrics::{NAV_FIRST_ROW_Y, NAV_ROW_STEP, navigation_hit_rect};
use crate::requirements::StoryRequirements;

const NAVIGATION_GROUP_COUNT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NavigationGroup {
    Foundation,
    Atoms,
    Selection,
    Molecules,
    Layout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NavigationRow {
    Group(NavigationGroup),
    Page {
        page: &'static str,
        group: NavigationGroup,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TreeExpansionState {
    foundation: bool,
    atoms: bool,
    selection: bool,
    molecules: bool,
    layout: bool,
}

impl Default for TreeExpansionState {
    fn default() -> Self {
        Self {
            foundation: true,
            atoms: true,
            selection: true,
            molecules: true,
            layout: true,
        }
    }
}

impl TreeExpansionState {
    pub(super) fn is_open(self, group: NavigationGroup) -> bool {
        match group {
            NavigationGroup::Foundation => self.foundation,
            NavigationGroup::Atoms => self.atoms,
            NavigationGroup::Selection => self.selection,
            NavigationGroup::Molecules => self.molecules,
            NavigationGroup::Layout => self.layout,
        }
    }

    pub(super) fn toggle(&mut self, group: NavigationGroup) {
        match group {
            NavigationGroup::Foundation => self.foundation = !self.foundation,
            NavigationGroup::Atoms => self.atoms = !self.atoms,
            NavigationGroup::Selection => self.selection = !self.selection,
            NavigationGroup::Molecules => self.molecules = !self.molecules,
            NavigationGroup::Layout => self.layout = !self.layout,
        }
    }
}

impl NavigationGroup {
    pub(super) fn label(self) -> &'static str {
        match self {
            NavigationGroup::Foundation => "Foundation",
            NavigationGroup::Atoms => "Atoms",
            NavigationGroup::Selection => "Selection",
            NavigationGroup::Molecules => "Molecules",
            NavigationGroup::Layout => "Layout",
        }
    }
}

pub(super) fn visible_rows(expansion: TreeExpansionState) -> Vec<NavigationRow> {
    let mut rows = Vec::new();
    for group in groups() {
        rows.push(NavigationRow::Group(group));
        if expansion.is_open(group) {
            rows.extend(
                StoryRequirements::required_pages()
                    .iter()
                    .copied()
                    .filter(move |page| group_for_page(page) == group)
                    .map(move |page| NavigationRow::Page { page, group }),
            );
        }
    }
    rows
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

pub(super) fn group_for_page(page: &str) -> NavigationGroup {
    match page {
        "theme-tokens" => NavigationGroup::Foundation,
        "text" | "icon" | "button" | "text-button" | "svg-button" | "icon-text-button"
        | "text-input" | "checkbox" | "radio" | "badge" | "divider" | "spacer" | "key-cap"
        | "loading-dots" | "spinner" | "progress-bar" | "color-swatch" | "toggle"
        | "slide-control" => NavigationGroup::Atoms,
        "context-menu" | "selection-list" => NavigationGroup::Selection,
        "row" | "column" | "stack" | "grid" | "scroll-area" | "split-pane" | "align-center" => {
            NavigationGroup::Layout
        }
        _ => NavigationGroup::Molecules,
    }
}

fn groups() -> [NavigationGroup; NAVIGATION_GROUP_COUNT] {
    [
        NavigationGroup::Foundation,
        NavigationGroup::Atoms,
        NavigationGroup::Selection,
        NavigationGroup::Molecules,
        NavigationGroup::Layout,
    ]
}

#[cfg(test)]
mod tests {
    use super::{NavigationRow, TreeExpansionState, visible_rows};

    #[test]
    fn tree_rows_include_groups_and_can_collapse_atoms() {
        let mut expansion = TreeExpansionState::default();
        let before = visible_rows(expansion);
        expansion.toggle(super::NavigationGroup::Atoms);
        let after = visible_rows(expansion);

        assert!(before.len() > after.len());
        assert!(
            after
                .iter()
                .any(|it| { matches!(it, NavigationRow::Group(super::NavigationGroup::Atoms)) })
        );
        assert!(
            !after
                .iter()
                .any(|it| { matches!(it, NavigationRow::Page { page: "button", .. }) })
        );
    }

    #[test]
    fn context_menu_is_grouped_under_selection() {
        let rows = visible_rows(TreeExpansionState::default());
        let selection_index = rows
            .iter()
            .position(|it| matches!(it, NavigationRow::Group(super::NavigationGroup::Selection)));
        let context_menu_index = rows.iter().position(|it| {
            matches!(
                it,
                NavigationRow::Page {
                    page: "context-menu",
                    group: super::NavigationGroup::Selection
                }
            )
        });

        assert!(selection_index.is_some());
        assert!(context_menu_index.is_some());
        assert!(selection_index < context_menu_index);
    }
}
